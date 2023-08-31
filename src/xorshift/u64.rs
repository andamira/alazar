// alazar::xorshift::u64
//
//! 64-bit versions of XorShift generators.
//

use devela::convert::{u64_from_u32_le, u64_from_u16_le, u64_from_u8_le};

/// The `XorShift64` pseudo-random number generator.
///
/// It has a 64-bit state and generates 64-bit numbers.
///
/// This is the classic 64-bit XorShift algorithm (13, 7, 17),
/// by George Marsaglia.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift64(u64);

impl XorShift64 {
    /// Returns a seeded `XorShift64` generator from the given 64-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Option<Self> {
        if seed == 0 {
            None
        } else {
            Some(Self(seed))
        }
    }

    /// Returns the next random `u64`.
    #[inline]
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

/// # Extra constructors
impl XorShift64 {
    /// Returns a seeded `XorShift64` generator from the given 64-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_64(seed: u64) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift64` generator from the given 2 × 32-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new2_32(seeds: [u32; 2]) -> Option<Self> {
        Self::new(u64_from_u32_le(seeds))
    }

    /// Returns a seeded `XorShift64` generator from the given 4 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new4_16(seeds: [u16; 4]) -> Option<Self> {
        Self::new(u64_from_u16_le(seeds))
    }

    /// Returns a seeded `XorShift64` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new8_8(seeds: [u8; 8]) -> Option<Self> {
        Self::new(u64_from_u8_le(seeds))
    }
}
