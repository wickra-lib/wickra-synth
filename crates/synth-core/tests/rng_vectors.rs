//! The anchor of the moat: fixed reference vectors for the portable PRNG.
//!
//! Byte-for-byte cross-language reproducibility rests entirely on every language
//! computing the *same* pseudo-random sequence from the same seed. These vectors
//! pin the exact `SplitMix64` seeding and `xoshiro256++` output; any port in any
//! language must reproduce them, and any change to the arithmetic here would
//! silently break parity across all ten bindings.

use synth_core::{mix, DetRng, SplitMix64};

#[test]
fn splitmix64_reference_vector() {
    let mut sm = SplitMix64::new(0);
    assert_eq!(sm.next_u64(), 0xE220_A839_7B1D_CDAF);
    assert_eq!(sm.next_u64(), 0x6E78_9E6A_A1B9_65F4);
    assert_eq!(sm.next_u64(), 0x06C4_5D18_8009_454F);
}

#[test]
fn detrng_seed_42_reference_vector() {
    let mut rng = DetRng::from_seed(42);
    assert_eq!(rng.next_u64(), 0xD076_4D4F_4476_689F);
    assert_eq!(rng.next_u64(), 0x519E_4174_576F_3791);
}

#[test]
fn detrng_same_seed_same_stream() {
    let mut a = DetRng::from_seed(12345);
    let mut b = DetRng::from_seed(12345);
    for _ in 0..256 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn next_f64_stays_in_unit_interval() {
    let mut rng = DetRng::from_seed(1);
    for _ in 0..100_000 {
        let x = rng.next_f64();
        assert!((0.0..1.0).contains(&x), "next_f64 out of [0,1): {x}");
    }
}

#[test]
fn next_normal_is_always_finite() {
    let mut rng = DetRng::from_seed(2);
    for _ in 0..100_000 {
        assert!(rng.next_normal().is_finite());
    }
}

#[test]
fn next_poisson_is_deterministic_and_bounded() {
    let mut a = DetRng::from_seed(3);
    let mut b = DetRng::from_seed(3);
    for _ in 0..10_000 {
        assert_eq!(a.next_poisson(5.0), b.next_poisson(5.0));
    }
    // Non-positive lambda draws nothing.
    let mut rng = DetRng::from_seed(4);
    assert_eq!(rng.next_poisson(0.0), 0);
    assert_eq!(rng.next_poisson(-3.0), 0);
}

#[test]
fn mix_is_a_pure_function() {
    assert_eq!(mix(7, 11), mix(7, 11));
    assert_ne!(mix(7, 11), mix(7, 12));
    assert_ne!(mix(7, 11), mix(8, 11));
}
