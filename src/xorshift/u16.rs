// alazar::xorshift::u16
//
//! 16-bit versions of XorShift generators.
//

use devela::convert::u16_from_u8_le;

/// The `XorShift16` pseudo-random number generator.
///
/// It has a 16-bit state and generates 16-bit numbers.
///
/// This is John Metcalf's 16-bit (7, 8, 9) version of George Marsaglia's
/// original [`XorShift32`][super::XorShift32].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift16(u16);

impl Default for XorShift16 {
    fn default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}

// private associated items
impl XorShift16 {
    const DEFAULT_SEED: u16 = 0xDEFA;

    #[cold]
    #[inline]
    const fn cold_path_result() -> Option<Self> {
        None
    }

    #[cold]
    #[inline]
    #[allow(dead_code)]
    const fn cold_path_default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}

impl XorShift16 {
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u16) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift16` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(seed: u16) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u16`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u16(&self) -> u16 {
        self.0
    }

    /// Returns the next random `u16`.
    ///
    #[inline]
    #[must_use]
    pub fn next_u16(&mut self) -> u16 {
        let mut x = self.0;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;
        Self(x)
    }
}

/// # Extra constructors
impl XorShift16 {
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_u16(seed: u16) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift16` generator from the given 2 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new2_u8(seeds: [u8; 2]) -> Option<Self> {
        Self::new(u16_from_u8_le(seeds))
    }
}

#[cfg(feature = "rand_core")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "rand_core")))]
mod impl_rand {
    use super::XorShift16;
    use devela::convert::{u32_from_u16_le, u64_from_u16_le};
    use rand_core::{Error, RngCore, SeedableRng};

    impl RngCore for XorShift16 {
        /// Returns the next 2 × random `u16` combined as a single `u32`.
        fn next_u32(&mut self) -> u32 {
            u32_from_u16_le([self.next_u16(), self.next_u16()])
        }

        /// Returns the next 4 × random `u16` combined as a single `u64`.
        fn next_u64(&mut self) -> u64 {
            u64_from_u16_le([
                self.next_u16(),
                self.next_u16(),
                self.next_u16(),
                self.next_u16(),
            ])
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let mut i = 0;
            while i < dest.len() {
                let random_u16 = self.next_u16();
                let bytes = random_u16.to_le_bytes();
                let remaining = dest.len() - i;

                if remaining >= 2 {
                    dest[i] = bytes[0];
                    dest[i + 1] = bytes[1];
                    i += 2;
                } else {
                    dest[i] = bytes[0];
                    i += 1;
                }
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for XorShift16 {
        type Seed = [u8; 2];

        /// When seeded with zero this implementation uses the default seed
        /// value as the cold path.
        fn from_seed(seed: Self::Seed) -> Self {
            if seed == [0; 2] {
                Self::cold_path_default()
            } else {
                Self::new_unchecked(u16::from_le_bytes(seed))
            }
        }
    }
}
