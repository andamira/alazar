// alazar::xorshift::u32
//
//! 32-bit versions of XorShift generators.
//

use devela::convert::{u32_from_u16_le, u32_from_u8_le};

/// The `XorShift32` pseudo-random number generator.
///
/// It has a 32-bit state and generates 32-bit numbers.
///
/// This is the classic 32-bit XorShift algorithm (13, 17, 5),
/// by George Marsaglia.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift32(u32);

impl Default for XorShift32 {
    fn default() -> Self {
        Self::new_unchecked(0xDEFA0017)
    }
}

impl XorShift32 {
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u32) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }
    #[inline]
    const fn cold_path_result() -> Option<Self> {
        None
    }

    /// Returns a seeded `XorShift32` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(seed: u32) -> Self {
        debug_assert![seed == 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u32`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u32(&self) -> u32 {
        self.0
    }

    /// Returns the next random `u32`.
    //
    // Algorithm "xor" from p. 4 of Marsaglia, "Xorshift RNGs"
    #[inline]
    #[must_use]
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        Self(x)
    }
}

/// # Extra constructors
impl XorShift32 {
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_32(seed: u32) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift32` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new2_16(seeds: [u16; 2]) -> Option<Self> {
        Self::new(u32_from_u16_le(seeds))
    }

    /// Returns a seeded `XorShift32` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new4_8(seeds: [u8; 4]) -> Option<Self> {
        Self::new(u32_from_u8_le(seeds))
    }
}
