// alazar::xorshift::generic
//
//! Const generic versions of XorShift generators.
//

/// Const generic version of the 8-bit XorShift algorithm letting you customize
/// the shifts.
///
/// It has an 8-bit state and generates 8-bit numbers.
pub struct XorShift8Gen<const S1: usize, const S2: usize, const S3: usize>(u8);

impl<const S1: usize, const S2: usize, const S3: usize> XorShift8Gen<S1, S2, S3> {
    /// Returns `None` if seed == `0`.
    ///
    /// # Panic
    /// Panics in debug if either `S1`, `S2` or `S3` are < 1 or > 7.
    #[inline]
    pub fn new(seed: u8) -> Option<Self> {
        debug_assert![S1 > 0 && S1 <= 7];
        debug_assert![S2 > 0 && S1 <= 7];
        debug_assert![S3 > 0 && S1 <= 7];

        if seed == 0 {
            None
        } else {
            Some(Self(seed))
        }
    }

    /// Returns the next random `u8`.
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
}
