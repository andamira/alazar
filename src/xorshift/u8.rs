// alazar::xorshift::u8
//
//! 8-bit versions of XorShift generators.
//

/// The `XorShift8` pseudo-random number generator.
///
/// It has an 8-bit state and generates 8-bit numbers.
///
/// This is a simple 8-bit version (3, 4, 2) of [`XorShift16`][super::XorShift16].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift8(u8);

impl Default for XorShift8 {
    fn default() -> Self {
        Self::new_unchecked(0xDE)
    }
}

impl XorShift8 {
    /// Returns a seeded `XorShift8` generator from the given 8-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u8) -> Option<Self> {
        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }
    #[cold]
    #[inline]
    const fn cold_path_result() -> Option<Self> {
        None
    }

    /// Returns a seeded `XorShift8` generator from the given 8-bit seed, unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(seed: u8) -> Self {
        debug_assert![seed == 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u8`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.0
    }

    /// Returns the next random `u8`.
    #[inline]
    #[must_use]
    pub fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 3;
        x ^= x >> 4;
        x ^= x << 2;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << 3;
        x ^= x >> 4;
        x ^= x << 2;
        Self(x)
    }
}

/// A version of [`XorShift8`] that allows customizing the shift values.
///
/// It has an 8-bit state and generates 8-bit numbers.
pub struct XorShift8Custom<const SH1: usize = 3, const SH2: usize = 4, const SH3: usize = 2>(u8);

impl<const SH1: usize, const SH2: usize, const SH3: usize> Default
    for XorShift8Custom<SH1, SH2, SH3>
{
    fn default() -> Self {
        Self::new_unchecked(0xDE)
    }
}

impl<const SH1: usize, const SH2: usize, const SH3: usize> XorShift8Custom<SH1, SH2, SH3> {
    /// Returns a seeded `XorShift8Custom` generator from the given 8-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    ///
    /// # Panics
    /// Panics in debug if either `SH1`, `SH2` or `SH3` are < 1 or > 7.
    #[inline]
    pub const fn new(seed: u8) -> Option<Self> {
        debug_assert![SH1 > 0 && SH1 <= 7];
        debug_assert![SH2 > 0 && SH1 <= 7];
        debug_assert![SH3 > 0 && SH1 <= 7];

        if seed == 0 {
            Self::cold_path_result()
        } else {
            Some(Self(seed))
        }
    }
    #[cold]
    #[inline]
    const fn cold_path_result() -> Option<Self> {
        None
    }

    /// Returns a seeded `XorShift8Custom` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    ///
    /// # Panics
    /// Panics in debug if either `SH1`, `SH2` or `SH3` are < 1 or > 7,
    /// or if the seed is `0`.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(seed: u8) -> Self {
        debug_assert![SH1 > 0 && SH1 <= 7];
        debug_assert![SH2 > 0 && SH1 <= 7];
        debug_assert![SH3 > 0 && SH1 <= 7];
        debug_assert![seed == 0, "Seed must be non-zero"];
        Self(seed)
    }

    /// Returns the current random `u8`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.0
    }

    /// Updates the state and returns the next random `u8`.
    ///
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << SH1;
        x ^= x >> SH2;
        x ^= x << SH3;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << SH1;
        x ^= x >> SH2;
        x ^= x << SH3;
        Self(x)
    }
}
