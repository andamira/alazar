// alazar::xorshift::u8
//
//! 8-bit versions of XorShift generators.
//

/// The `XorShift8` pseudo-random number generator.
///
/// It has an 8-bit state and generates 8-bit numbers.
///
/// This is a simple 8-bit version (3, 4, 2) of [`XorShift16`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift8(u8);

impl XorShift8 {
    /// Returns a seeded `XorShift8` generator from the given 8-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u8) -> Option<Self> {
        if seed == 0 {
            None
        } else {
            Some(Self(seed))
        }
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

/// Const generic version of the 8-bit XorShift algorithm letting you customize
/// the shifts.
///
/// It has an 8-bit state and generates 8-bit numbers.
pub struct XorShift8Gen<const S1: usize, const S2: usize, const S3: usize>(u8);

impl<const S1: usize, const S2: usize, const S3: usize> XorShift8Gen<S1, S2, S3> {
    /// Returns a seeded `XorShift8Gen` generator from the given 8-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    ///
    /// # Panic
    /// Panics in debug if either `S1`, `S2` or `S3` are < 1 or > 7.
    #[inline]
    pub const fn new(seed: u8) -> Option<Self> {
        debug_assert![S1 > 0 && S1 <= 7];
        debug_assert![S2 > 0 && S1 <= 7];
        debug_assert![S3 > 0 && S1 <= 7];

        if seed == 0 {
            None
        } else {
            Some(Self(seed))
        }
    }

    /// Returns a seeded `XorShift8Gen` generator from the given 8-bit seed,
    /// unchecked.
    ///
    /// The seed must not be `0`, otherwise every result will also be `0`.
    ///
    /// # Panic
    /// Panics in debug if either `S1`, `S2` or `S3` are < 1 or > 7,
    /// or if the seed is `0`.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(seed: u8) -> Self {
        debug_assert![S1 > 0 && S1 <= 7];
        debug_assert![S2 > 0 && S1 <= 7];
        debug_assert![S3 > 0 && S1 <= 7];
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
        x ^= x << S1;
        x ^= x >> S2;
        x ^= x << S3;
        self.0 = x;
        x
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        x ^= x << S1;
        x ^= x >> S2;
        x ^= x << S3;
        Self(x)
    }
}
