// alazar::xorshift
//
//! Pseudo-random number generators based on [Xorshift].
//!
//! This module defines several types:
//! - classic XorShift algorithms:
//!   ([`XorShift32`], [`XorShift64`], [`XorShift128`], [`XorShift128p`]).
//! - variations with a smaller state:
//!   ([`XorShift16`], [`XorShift8`]).
//! - other implementations loosely based on XorShift:
//!   ([`Xyza8a`], [`Xyza8b`]).
//!
//! [Xorshift]: https://en.wikipedia.org/wiki/Xorshift
//

mod generic;
mod xyza8;

pub use generic::XorShift8Gen;
pub use xyza8::{Xyza8a, Xyza8b};

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
}

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
    pub fn new(seed: u64) -> Option<Self> {
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

/// The `XorShift128` pseudo-random number generator.
///
/// It has a 128-bit state and generates 64-bit numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XorShift128([u32; 4]);

impl XorShift128 {
    /// Returns a seeded XorShift128 generator from the given 2 × 64-bit seeds.
    #[inline]
    #[must_use]
    pub const fn new2(seed1: u64, seed2: u64) -> Option<Self> {
        if (seed1 | seed2) != 0 {
            let high1: u32 = (seed1 >> 32) as u32;
            let low1: u32 = (seed1 & u32::MAX as u64) as u32;

            let high2: u32 = (seed2 >> 32) as u32;
            let low2: u32 = (seed2 & u32::MAX as u64) as u32;

            Some(Self([low1, high1, low2, high2]))
        } else {
            None
        }
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new4(seed1: u32, seed2: u32, seed3: u32, seed4: u32) -> Option<Self> {
        if (seed1 | seed2 | seed3 | seed4) != 0 {
            Some(Self([seed1, seed2, seed3, seed4]))
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
    pub const fn new2(seed1: u64, seed2: u64) -> Option<Self> {
        if (seed1 | seed2) != 0 {
            Some(Self([seed1, seed2]))
        } else {
            None
        }
    }

    /// Returns a seeded `XorShift128+` generator from the given 4 × 32-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new4(seed1: u32, seed2: u32, seed3: u32, seed4: u32) -> Option<Self> {
        if (seed1 | seed2 | seed3 | seed4) != 0 {
            let high1: u64 = (seed2 as u64) << 32;
            let low1: u64 = seed1 as u64;
            let seed1: u64 = high1 | low1;

            let high2: u64 = (seed4 as u64) << 32;
            let low2: u64 = seed3 as u64;
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
