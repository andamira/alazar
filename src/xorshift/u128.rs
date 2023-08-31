// alazar::xorshift::u128
//
//! 128-bit versions of XorShift generators.
//

use devela::convert::{
    u128_into_u32_le, u128_into_u64_le, u32_from_u16_le, u32_from_u8_le, u64_from_u16_le,
    u64_from_u32_le, u64_from_u8_le, u64_into_u32_le,
};

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

    /// Returns the current random `u64`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u64(&self) -> u64 {
        ((self.0[0] as u64) << 32) | (self.0[1] as u64)
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

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        let t = x[3];
        let mut s = x[0];
        x[3] = x[2];
        x[2] = x[1];
        x[1] = s;
        s ^= s << 11;
        s ^= s >> 8;
        x[0] = s ^ t ^ (t >> 19);
        Self(x)
    }
}

/// # Extra constructors
impl XorShift128 {
    /// Returns a seeded `XorShift128` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_128(seed: u128) -> Option<Self> {
        Self::new(u128_into_u32_le(seed))
    }

    /// Returns a seeded `XorShift128` generator from the given 2 × 64-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new2_64(seeds: [u64; 2]) -> Option<Self> {
        let [x, y] = u64_into_u32_le(seeds[0]);
        let [z, a] = u64_into_u32_le(seeds[1]);
        Self::new([x, y, z, a])
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 32-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new4_32(seeds: [u32; 4]) -> Option<Self> {
        Self::new(seeds)
    }

    /// Returns a seeded `XorShift128` generator from the given 8 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new8_16(seeds: [u16; 8]) -> Option<Self> {
        Self::new([
            u32_from_u16_le([seeds[0], seeds[1]]),
            u32_from_u16_le([seeds[2], seeds[3]]),
            u32_from_u16_le([seeds[4], seeds[5]]),
            u32_from_u16_le([seeds[6], seeds[7]]),
        ])
    }

    /// Returns a seeded `XorShift128` generator from the given 16 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new16_8(seeds: [u8; 16]) -> Option<Self> {
        Self::new([
            u32_from_u8_le([seeds[0], seeds[1], seeds[2], seeds[3]]),
            u32_from_u8_le([seeds[4], seeds[5], seeds[6], seeds[7]]),
            u32_from_u8_le([seeds[8], seeds[9], seeds[10], seeds[11]]),
            u32_from_u8_le([seeds[12], seeds[13], seeds[14], seeds[15]]),
        ])
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
    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// Returns `None` if all given seeds are `0`.
    #[inline]
    #[must_use]
    pub const fn new(seeds: [u64; 2]) -> Option<Self> {
        if (seeds[0] | seeds[1]) != 0 {
            Some(Self(seeds))
        } else {
            None
        }
    }

    /// Returns the current random `u64`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u64(&self) -> u64 {
        self.0[0] + self.0[1]
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

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut x = self.0;
        let mut s1 = x[0];
        let s0 = x[1];
        s1 ^= s0;
        x[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16); // a, b
        x[1] = s1.rotate_left(37); // c
        Self(x)
    }
}

/// # Extra constructors
impl XorShift128p {
    /// Returns a seeded `XorShift128+` generator from the given 128-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_128(seed: u128) -> Option<Self> {
        Self::new(u128_into_u64_le(seed))
    }

    /// Returns a seeded `XorShift128+` generator from the given 2 × 64-bit seeds.
    ///
    /// This is an alias of [`new`][Self#method.new].
    #[inline]
    pub const fn new2_64(seeds: [u64; 2]) -> Option<Self> {
        Self::new(seeds)
    }

    /// Returns a seeded `XorShift128+` generator from the given 4 × 32-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new4_32(seeds: [u32; 4]) -> Option<Self> {
        Self::new([
            u64_from_u32_le([seeds[0], seeds[1]]),
            u64_from_u32_le([seeds[2], seeds[3]]),
        ])
    }

    /// Returns a seeded `XorShift128+` generator from the given 8 × 16-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new8_16(seeds: [u16; 8]) -> Option<Self> {
        Self::new([
            u64_from_u16_le([seeds[0], seeds[1], seeds[2], seeds[3]]),
            u64_from_u16_le([seeds[4], seeds[5], seeds[6], seeds[7]]),
        ])
    }

    /// Returns a seeded `XorShift128` generator from the given 4 × 8-bit seeds.
    ///
    /// The seeds will be joined in little endian order.
    #[inline]
    pub const fn new16_8(seeds: [u8; 16]) -> Option<Self> {
        let s = seeds;
        Self::new([
            u64_from_u8_le([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]),
            u64_from_u8_le([s[8], s[9], s[10], s[11], s[12], s[13], s[14], s[15]]),
        ])
    }
}
