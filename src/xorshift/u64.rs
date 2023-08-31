// alazar::xorshift::u64
//
//! 64-bit versions of XorShift generators.
//

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
