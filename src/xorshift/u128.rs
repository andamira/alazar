// alazar::xorshift::u128
//
//! 128-bit versions of XorShift generators.
//

/// The `XorShift128` pseudo-random number generator.
///
/// It has a 128-bit state and generates 64-bit numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128([u32; 4]);

impl XorShift128 {
    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new(seeds: [u32; 4]) -> Option<Self> {
        if (seeds[0] | seeds[1] | seeds[2] | seeds[3]) != 0 {
            Some(Self([seeds[0], seeds[1], seeds[2], seeds[3]]))
        } else {
            None
        }
    }

    /// Returns the next random `u64`.
    #[inline]
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        let t = self.0[3];
        let mut s = self.0[0];
        self.0[3] = self.0[2];
        self.0[2] = self.0[1];
        self.0[1] = s;
        s ^= s << 11;
        s ^= s >> 8;
        self.0[0] = s ^ t ^ (t >> 19);
        ((self.0[0] as u64) << 32) | (self.0[1] as u64)
    }
}

/// The `XorShift128+` pseudo-random number generator.
///
/// It has a 128-bit state and generates 64-bit numbers.
///
/// It is generally considered to have better statistical properties than
/// [`XorShift128`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128p([u64; 2]);

impl XorShift128p {
    /// Returns a seeded `XorShift128+` generator from the given 128-bit seed.
    ///
    /// Returns `None` if the given seed is `0`.
    #[inline]
    #[must_use]
    pub const fn new(seed: u128) -> Option<Self> {
        if seed != 0 {
            let high: u64 = (seed >> 64) as u64;
            let low: u64 = (seed & u64::MAX as u128) as u64;

            Some(Self([low, high]))
        } else {
            None
        }
    }

    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new2(seeds: [u64; 2]) -> Option<Self> {
        if (seeds[0] | seeds[1]) != 0 {
            Some(Self(seeds))
        } else {
            None
        }
    }

    /// Returns a seeded `XorShift128+` generator from the given 4 × 32-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new4(seeds: [u32; 4]) -> Option<Self> {
        if (seeds[0] | seeds[1] | seeds[2] | seeds[3]) != 0 {
            let high1: u64 = (seeds[1] as u64) << 32;
            let low1: u64 = seeds[0] as u64;
            let seed1: u64 = high1 | low1;

            let high2: u64 = (seeds[3] as u64) << 32;
            let low2: u64 = seeds[2] as u64;
            let seed2: u64 = high2 | low2;

            Some(Self([seed1, seed2]))
        } else {
            None
        }
    }

    /// Returns the next random `u64`.
    #[inline]
    #[must_use]
    pub fn next_64(&mut self) -> u64 {
        let mut s1 = self.0[0];
        let s0 = self.0[1];
        let result = s0 + s1;
        s1 ^= s0;
        self.0[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16); // a, b
        self.0[1] = s1.rotate_left(37); // c
        result
    }
}
