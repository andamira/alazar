// alazar::xorshift::u64
//
//! 64-bit versions of XorShift generators.
//

use devela::convert::{u64_from_u16_le, u64_from_u32_le, u64_from_u8_le};

/// The `XorShift64` pseudo-random number generator.
///
/// It has a 64-bit state and generates 64-bit numbers.
///
/// This is the classic 64-bit *XorShift* algorithm (13, 7, 17),
/// by George Marsaglia.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift64(u64);

impl Default for XorShift64 {
    fn default() -> Self {
        Self::new_unchecked(Self::DEFAULT_SEED)
    }
}

// private associated items
impl XorShift64 {
    const DEFAULT_SEED: u64 = 0xDEFA0017_DEFA0017;

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

impl XorShift64 {
    /// Returns a seeded `XorShift64` generator from the given 64-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift64` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(seed: u64) -> Self {
        debug_assert![seed != 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u64`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u16(&self) -> u64 {
        self.0
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

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        Self(x)
    }
}

/// # Extra constructors
impl XorShift64 {
    /// Returns a seeded `XorShift64` generator from the given 64-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_u64(seed: u64) -> Option<Self> {
        Self::new(seed)
    }

    /// Returns a seeded `XorShift64` generator from the given 2 × 32-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new2_u32(seeds: [u32; 2]) -> Option<Self> {
        Self::new(u64_from_u32_le(seeds))
    }

    /// Returns a seeded `XorShift64` generator from the given 4 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new4_u16(seeds: [u16; 4]) -> Option<Self> {
        Self::new(u64_from_u16_le(seeds))
    }

    /// Returns a seeded `XorShift64` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new8_u8(seeds: [u8; 8]) -> Option<Self> {
        Self::new(u64_from_u8_le(seeds))
    }
}
