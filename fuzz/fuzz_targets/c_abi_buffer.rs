#![no_main]
//! Fuzz the C ABI's length-out buffer protocol.
//!
//! `bindings/c/src/lib.rs` is the only `unsafe` in the workspace, and
//! `SECURITY.md` names it and its buffer protocol as in scope. Until now it was
//! covered by ten hand-written unit tests, all with well-formed inputs — while
//! the four fuzz targets pointed at safe Rust in the core. This drives the part
//! where a memory fault can actually originate: a caller-supplied capacity that
//! disagrees with the response length, and commands arriving in an order the
//! protocol does not expect.
//!
//! What must hold, whatever the bytes say:
//!
//!   * The call never reads or writes outside the caller's `cap` bytes. The
//!     buffer is bracketed with a guard region and checked afterwards.
//!   * `len < cap` means the response and a NUL were written; anything else
//!     leaves the buffer untouched, which is the promise the header makes and
//!     the one every binding's two-call loop relies on.
//!   * Nothing panics across the boundary.

use libfuzzer_sys::fuzz_target;
use std::ffi::CString;
// The crate is `wickra-synth-c`; its lib is named `wickra_synth`, which is the
// symbol prefix the header uses.
use wickra_synth::{
    wickra_synth_command, wickra_synth_free, wickra_synth_new, WICKRA_SYNTH_ERR_PENDING,
};

const SPEC: &str = r#"{"seed":42,"bars":4,"start_price":100.0,
    "regimes":[{"kind":"trend","len":4,"drift":0.001,"vol":0.01}],
    "microstructure":{"book_depth":2,"spread_bps":4.0,"trade_rate":2.0}}"#;

/// Byte written either side of the caller's window; the ABI must never touch it.
const GUARD: u8 = 0xA5;

fuzz_target!(|data: &[u8]| {
    let Ok(cmd_text) = std::str::from_utf8(data) else {
        return;
    };
    // A NUL inside the string cannot be expressed as a C string; that is the
    // caller's problem, not the ABI's.
    let Ok(cmd) = CString::new(cmd_text) else {
        return;
    };

    let spec = CString::new(SPEC).expect("static spec has no interior NUL");
    let handle = unsafe { wickra_synth_new(spec.as_ptr()) };
    assert!(!handle.is_null(), "the static spec must build a handle");

    // First call: measure. `out = NULL, cap = 0` must never write anywhere.
    let len = unsafe { wickra_synth_command(handle, cmd.as_ptr(), std::ptr::null_mut(), 0) };
    if len < 0 {
        // A negative code is a refusal, and a refusal leaves nothing pending, so
        // the handle stays usable. PENDING cannot happen on a first call.
        assert_ne!(len, WICKRA_SYNTH_ERR_PENDING);
        unsafe { wickra_synth_free(handle) };
        return;
    }
    let len = usize::try_from(len).expect("a non-negative i32 fits usize");

    // Second call: write into a capacity the fuzzer chooses, which is often the
    // wrong one. The window sits inside a guarded allocation so an overrun in
    // either direction is caught rather than merely surviving.
    let cap = usize::from(data.first().copied().unwrap_or(0)) * 8;
    let pad = 16;
    let mut arena = vec![GUARD; pad + cap + pad];
    let written = unsafe {
        wickra_synth_command(
            handle,
            cmd.as_ptr(),
            arena.as_mut_ptr().add(pad).cast(),
            cap,
        )
    };

    assert!(arena[..pad].iter().all(|&b| b == GUARD), "wrote before the buffer");
    assert!(
        arena[pad + cap..].iter().all(|&b| b == GUARD),
        "wrote past the buffer"
    );
    assert_eq!(
        usize::try_from(written).expect("second call returns the same length"),
        len,
        "the two calls disagreed about the response length"
    );

    let window = &arena[pad..pad + cap];
    if len < cap {
        // Delivered: the response plus a NUL, and nothing beyond it.
        assert_eq!(window[len], 0, "no terminating NUL");
        assert!(
            window[len + 1..].iter().all(|&b| b == GUARD),
            "wrote past the response"
        );
    } else {
        // Not delivered: the header promises the buffer is left untouched.
        assert!(
            window.iter().all(|&b| b == GUARD),
            "buffer was modified on a too-small capacity"
        );
    }

    unsafe { wickra_synth_free(handle) };
});
