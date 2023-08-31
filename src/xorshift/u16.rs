// alazar::xorshift::u16
//
//! 16-bit versions of XorShift generators.
//

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
}
