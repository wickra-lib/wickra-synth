//! The deterministic, portable PRNG — the moat of the whole project.
//!
//! All randomness in `wickra-synth-core` flows through this module, and nowhere else:
//! no language binding ever draws its own randomness, so a given seed produces
//! the byte-identical stream on every platform and in every language. The
//! generator is a [`SplitMix64`] seed expander feeding a [`DetRng`]
//! (xoshiro256++) stream, implemented with explicit wrapping 64-bit arithmetic
//! so the bit sequence is reproducible everywhere.
//!
//! This is a **non-cryptographic** generator chosen for speed and
//! reproducibility. It must never be used for keys, tokens, or any security
//! purpose.

use core::f64::consts::TAU;

/// SplitMix64 — used only to seed / derive substreams, never for draws directly.
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Construct a SplitMix64 from a 64-bit seed.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advance the state and return the next 64-bit output.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Combine two 64-bit values into one seed, for deriving an independent
/// substream from `(master_seed, index)`. Keeps parallel work reproducible
/// regardless of scheduling order (v1 is single-symbol and uses one stream; the
/// mechanism is here and tested for the future parallel path).
#[must_use]
pub fn mix(a: u64, b: u64) -> u64 {
    SplitMix64::new(a ^ b.rotate_left(32)).next_u64()
}

/// xoshiro256++ — the actual draw generator (portable, fixed 64-bit arithmetic).
#[derive(Debug, Clone)]
pub struct DetRng {
    s: [u64; 4],
}

impl DetRng {
    /// Seed a fresh stream from a master seed via the standard SplitMix64 recipe.
    #[must_use]
    pub fn from_seed(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        DetRng {
            s: [sm.next_u64(), sm.next_u64(), sm.next_u64(), sm.next_u64()],
        }
    }

    /// The next 64-bit output of the xoshiro256++ stream.
    pub fn next_u64(&mut self) -> u64 {
        let result = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(23)
            .wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// Map the next output to `[0, 1)` using the top 53 bits over `2^53`. This
    /// mapping is part of the determinism contract — do not change it.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// A standard-normal draw via Box-Muller. Always consumes **two** uniforms
    /// (the second `cos`-partner is used; a `sin` partner would be the only
    /// value discarded) so the draw order stays stable regardless of how many
    /// normals a regime uses. `u1` is clamped away from zero to guard `ln(0)`.
    pub fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(f64::MIN_POSITIVE);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (TAU * u2).cos()
    }

    /// A Poisson draw via Knuth's inverse-CDF method. Deterministic and
    /// order-stable. `lambda <= 0` returns `0` without drawing.
    pub fn next_poisson(&mut self, lambda: f64) -> u64 {
        if lambda <= 0.0 {
            return 0;
        }
        let limit = (-lambda).exp();
        let mut k: u64 = 0;
        let mut p = 1.0_f64;
        loop {
            k += 1;
            p *= self.next_f64();
            if p <= limit {
                break;
            }
        }
        k - 1
    }
}

#[cfg(test)]
mod tests {
    use super::{mix, DetRng, SplitMix64};

    // Regression pins for the generator. These exact values are produced by the
    // reference SplitMix64 / xoshiro256++ implementations above; any change to
    // the arithmetic that shifts them would silently break cross-language
    // byte-parity, so they are pinned here.
    #[test]
    fn splitmix64_reference() {
        let mut sm = SplitMix64::new(0);
        assert_eq!(sm.next_u64(), 0xE220_A839_7B1D_CDAF);
        assert_eq!(sm.next_u64(), 0x6E78_9E6A_A1B9_65F4);
        assert_eq!(sm.next_u64(), 0x06C4_5D18_8009_454F);
    }

    #[test]
    fn detrng_seed_42_reference() {
        let mut rng = DetRng::from_seed(42);
        assert_eq!(rng.next_u64(), 0xD076_4D4F_4476_689F);
        assert_eq!(rng.next_u64(), 0x519E_4174_576F_3791);
    }

    #[test]
    fn detrng_is_deterministic() {
        let mut a = DetRng::from_seed(7);
        let mut b = DetRng::from_seed(7);
        for _ in 0..64 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn next_f64_in_unit_interval() {
        let mut rng = DetRng::from_seed(1);
        for _ in 0..10_000 {
            let x = rng.next_f64();
            assert!((0.0..1.0).contains(&x), "next_f64 out of [0,1): {x}");
        }
    }

    #[test]
    fn next_normal_is_finite() {
        let mut rng = DetRng::from_seed(2);
        for _ in 0..10_000 {
            assert!(rng.next_normal().is_finite());
        }
    }

    #[test]
    fn next_poisson_zero_lambda_is_zero() {
        let mut rng = DetRng::from_seed(3);
        assert_eq!(rng.next_poisson(0.0), 0);
        assert_eq!(rng.next_poisson(-1.0), 0);
    }

    #[test]
    fn next_poisson_mean_is_close() {
        let mut rng = DetRng::from_seed(4);
        let n = 20_000;
        let mut total = 0u64;
        for _ in 0..n {
            total += rng.next_poisson(8.0);
        }
        let mean = total as f64 / f64::from(n);
        assert!((mean - 8.0).abs() < 0.2, "poisson mean off: {mean}");
    }

    #[test]
    fn mix_is_pure_function_of_inputs() {
        assert_eq!(mix(42, 0), mix(42, 0));
        assert_ne!(mix(42, 0), mix(42, 1));
    }
}
