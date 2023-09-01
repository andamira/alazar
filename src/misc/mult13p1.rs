// alazar::xorshift::mult13p1
//
//!
//

/// A weak 8-bit pseudo-random number generator from 1977.
///
/// It basically multiplies the previous number by 13, and adds 1.
///
/// It does the multiplication very efficiently by bit-shifting powers of 2.
///
/// The original code by B. J. Murphy was published in
/// [Byte Magazine, November 1977, page 218][link].
///
/// [link]: https://archive.org/details/BYTE_Vol_02-11_1977-11_Sweet_16/page/n219/
/// [RCA1802]: https://en.wikipedia.org/wiki/RCA_1802
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mult13P1 {
    state: u8,
}

impl Default for Mult13P1 {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

// private associated items
impl Mult13P1 {
    const DEFAULT_SEED: u8 = 0xDE;
}

impl Mult13P1 {
    /// Returns a seeded XorShift8 generator from the given 8-bit seed.
    #[inline]
    #[must_use]
    pub const fn new(seed: u8) -> Self {
        Self { state: seed }
    }

    /// Returns the current random `u8`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.state
    }

    /// Returns the next random `u8`.
    #[inline]
    #[must_use]
    pub fn next_u8(&mut self) -> u8 {
        let n = self.state;
        let n_times_2_pow_2 = n << 2;
        let n_times_2_pow_3 = n << 3;
        // 13*n = n + n*2^2 + n*2^3
        self.state = n + n_times_2_pow_2 + n_times_2_pow_3 + 1;
        self.state
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut state = self.state;
        let n_times_2_pow_2 = state << 2;
        let n_times_2_pow_3 = state << 3;
        // 13*n = n + n*2^2 + n*2^3
        state += n_times_2_pow_2 + n_times_2_pow_3 + 1;
        Self { state }
    }
}

/// # Extra constructors
impl Mult13P1 {
    /// Returns a seeded `Mult13P1` generator from the given 8-bit seed.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new1_u8(seed: u8) -> Self {
        Self::new(seed)
    }
}
