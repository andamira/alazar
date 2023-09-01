// alazar::xorshift::other::xyza8
//
//!
//

use devela::convert::{u16_into_u8_le, u32_into_u8_le};

/// A simple 8-bit pseudo-random number generator with 32-bit of state,
/// based on the *XorShift* algorithm.
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

impl Default for Xyza8a {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

// private associated items
impl Xyza8a {
    const DEFAULT_SEED: [u8; 4] = [0xDE, 0xFA, 0x00, 0x17];
}

impl Xyza8a {
    /// Returns a seeded `Xyza8a` generator from the given 4 × 8-bit seeds.
    #[inline]
    pub const fn new(seeds: [u8; 4]) -> Self {
        Self {
            x: seeds[0],
            y: seeds[1],
            z: seeds[2],
            a: seeds[3],
        }
    }

    /// Returns the current random `u8`.
    #[inline(always)]
    #[must_use]
    pub const fn current_u8(&self) -> u8 {
        self.a
    }

    /// Updates the state and returns the next random `u8`.
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        let t = self.x ^ (self.x << 4);
        self.x = self.y;
        self.y = self.z;
        self.z = self.a;
        self.a = self.z ^ t ^ (self.z >> 1) ^ (t << 1);
        self.a
    }

    /// Returns a copy of the next new random state.
    #[inline]
    #[must_use]
    pub const fn next_new(&self) -> Self {
        let mut new = *self;

        let t = new.x ^ (new.x << 4);
        new.x = new.y;
        new.y = new.z;
        new.z = new.a;
        new.a = new.z ^ t ^ (new.z >> 1) ^ (t << 1);
        new
    }
}

/// # Extra constructors
impl Xyza8a {
    /// Returns a seeded `Xyza8a` generator from the given 32-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_32(seed: u32) -> Self {
        Self::new4_8(u32_into_u8_le(seed))
    }

    /// Returns a seeded `Xyza8a` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new2_16(seeds: [u16; 2]) -> Self {
        let [x, y] = u16_into_u8_le(seeds[0]);
        let [z, a] = u16_into_u8_le(seeds[1]);
        Self::new4_8([x, y, z, a])
    }

    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seeds.
    /// This is an alias of [`new`][Self#method.new].
    #[inline(always)]
    pub const fn new4_8(seeds: [u8; 4]) -> Self {
        Self::new(seeds)
    }
}

// -----------------------------------------------------------------------------

/// A simple 8-bit pseudo-random number generator with 32-bit of state,
/// based on the *XorShift* algorithm.
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

impl Default for Xyza8b {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

// private associated items
impl Xyza8b {
    const DEFAULT_SEED: [u8; 4] = [0xDE, 0xFA, 0x00, 0x17];
}

impl Xyza8b {
    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seeds.
    /// This is the fastest constructor.
    #[inline]
    pub const fn new(seeds: [u8; 4]) -> Self {
        Self {
            x: seeds[0],
            y: seeds[1],
            z: seeds[2],
            a: seeds[3],
        }
    }

    /// Returns the current random `u8`.
    #[inline(always)]
    pub const fn current_u8(&self) -> u8 {
        self.a
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

    /// Returns a copy of the next new random state.
    #[inline]
    pub const fn next_new(&self) -> Self {
        let mut new = *self;

        let t = new.x ^ (new.x >> 1);
        new.x = new.y;
        new.y = new.z;
        new.z = new.a;
        new.a = new.z ^ t ^ (new.z >> 3) ^ (t << 1);
        new
    }
}

/// # Extra constructors
impl Xyza8b {
    /// Returns a seeded `Xyza8a` generator from the given 32-bit seed.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new1_32(seed: u32) -> Self {
        Self::new4_8(u32_into_u8_le(seed))
    }

    /// Returns a seeded `Xyza8a` generator from the given 2 × 16-bit seeds.
    ///
    /// The seeds will be split in little endian order.
    #[inline]
    pub const fn new2_16(seeds: [u16; 2]) -> Self {
        let [x, y] = u16_into_u8_le(seeds[0]);
        let [z, b] = u16_into_u8_le(seeds[1]);
        Self::new4_8([x, y, z, b])
    }

    /// Returns a seeded `Xyza8b` generator from the given 4 × 8-bit seeds.
    /// This is an alias of [`new`][Self#method.new].
    #[inline(always)]
    pub const fn new4_8(seeds: [u8; 4]) -> Self {
        Self::new(seeds)
    }
}
