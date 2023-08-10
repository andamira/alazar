// alazar::xorshift::other::xyza8
//
//!
//

/// A simple 8-bit pseudo-random number generator with 32-bit of state,
/// based on the XorShift algorithm.
///
/// It has a 0.8% chance of falling into a poor quality short chain,
/// a some degree of care is required to seed it. However, the quality of the
/// random numbers is excellent for such a small state (32 bits), and it passes
/// almost all of the die hard tests.
///
/// Its longest cycle is 4,261,412,736.
///
/// # License
/// This algorithm was ported from [8bit_rng](https://github.com/edrosten/8bit_rng).
/// Copyright (c) 2008-2013 Edward Rosten.
/// Licensed under the [BSD 2-Clause "Simplified" License][license]
///
/// [license]: https://github.com/edrosten/8bit_rng/blob/master/LICENSE
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xyza8a {
    x: u8,
    y: u8,
    z: u8,
    a: u8,
}

impl Xyza8a {
    /// Returns a seeded `Xyza8a` generator from the given 4 × 8-bit seed.
    #[inline]
    pub const fn new4(seed1: u8, seed2: u8, seed3: u8, seed4: u8) -> Self {
        Self {
            x: seed1,
            y: seed2,
            z: seed3,
            a: seed4,
        }
    }

    /// Returns the next random `u8`.
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        let t = self.x ^ (self.x << 4);
        self.x = self.y;
        self.y = self.z;
        self.z = self.a;
        self.a = self.z ^ t ^ (self.z >> 1) ^ (t << 1);
        self.a
    }
}

/// A simple 8-bit pseudo-random number generator with 32-bit of state,
/// based on the XorShift algorithm.
///
/// It has an almost optimal cycle so no real care is required
/// for seeding except avoiding all zeros, but it fails many of the die hard
/// random number tests.
///
/// Its longest cycle is 4,294,967,294.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xyza8b {
    x: u8,
    y: u8,
    z: u8,
    a: u8,
}

impl Xyza8b {
    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seed.
    #[inline]
    pub const fn new4(seed1: u8, seed2: u8, seed3: u8, seed4: u8) -> Self {
        Self {
            x: seed1,
            y: seed2,
            z: seed3,
            a: seed4,
        }
    }

    /// Returns the next random `u8`.
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        let t = self.x ^ (self.x >> 1);
        self.x = self.y;
        self.y = self.z;
        self.z = self.a;
        self.a = self.z ^ t ^ (self.z >> 3) ^ (t << 1);
        self.a
    }
}
