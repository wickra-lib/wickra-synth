#![no_main]
//! Fuzz the PRNG driver: any seed must yield a total, panic-free draw stream —
//! `next_f64` stays in `[0, 1)`, `next_normal` is finite, `next_poisson` never
//! loops forever, and the same seed always produces the same sequence.

use libfuzzer_sys::fuzz_target;
use synth_core::DetRng;

fuzz_target!(|data: &[u8]| {
    let mut seed = [0u8; 8];
    for (i, s) in seed.iter_mut().enumerate() {
        *s = data.get(i).copied().unwrap_or(0);
    }
    let seed = u64::from_le_bytes(seed);

    let mut a = DetRng::from_seed(seed);
    let mut b = DetRng::from_seed(seed);
    // A lambda derived from the input, clamped to a sane bound.
    let lambda = f64::from(data.get(8).copied().unwrap_or(1)) / 8.0;

    for _ in 0..256 {
        assert_eq!(a.next_u64(), b.next_u64(), "same seed diverged");
        let f = a.next_f64();
        assert!((0.0..1.0).contains(&f), "next_f64 out of range: {f}");
        assert!(a.next_normal().is_finite(), "next_normal not finite");
        let _ = a.next_poisson(lambda);
        // Keep b in lock-step so the seed-equality check stays meaningful.
        let _ = b.next_f64();
        let _ = b.next_normal();
        let _ = b.next_poisson(lambda);
    }
});
