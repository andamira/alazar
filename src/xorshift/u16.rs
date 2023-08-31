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
/// original [`XorShift32`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift16(u16);

impl XorShift16 {
    /// Returns a seeded `XorShift16` generator from the given 16-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u16) -> Option<Self> {
        if seed == 0 {
            None
        } else {
            Some(Self(seed))
        }
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
    pub const fn new1_16(seed: u16) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift16` generator from the given 2 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new2_8(seeds: [u8; 2]) -> Option<Self> {
        Self::new(u16_from_u8_le(seeds))
    }
}
