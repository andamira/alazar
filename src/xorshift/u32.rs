// alazar::xorshift::u32
//
//! 32-bit versions of XorShift generators.
//

/// The `XorShift32` pseudo-random number generator.
///
/// It has a 32-bit state and generates 32-bit numbers.
///
/// This is the classic 32-bit XorShift algorithm (13, 17, 5),
/// by George Marsaglia.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift32(u32);

impl XorShift32 {
    /// Returns a seeded `XorShift32` generator from the given 32-bit seed.
    ///
    /// Returns `None` if seed == `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u32) -> Option<Self> {
        if seed == 0 {
            None
        } else {
            Some(Self(seed))
        }
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
}
