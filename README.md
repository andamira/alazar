# alazar

[![Crate](https://img.shields.io/crates/v/alazar.svg)](https://crates.io/crates/alazar)
[![API](https://docs.rs/alazar/badge.svg)](https://docs.rs/alazar/)
[![MSRV: 1.72.0](https://flat.badgen.net/badge/MSRV/1.72.0/purple)](https://releases.rs/docs/1.72.0/)

Random number generation.

See [the documentation](https://docs.rs/alazar/) for more information.

## Status

This is currently in an experimental stage of development.

## License
This project is dual licensed under either [MIT](LICENSE-MIT)
or [Apache-2.0](LICENSE-APACHE) at your option.

### Derived Works

This project includes the following derived works:
- [`Mult13P1`] is based on code from B. J. Murphy for the RCA1802,
  published in [Byte Magazine][0] in nov 1977.
- [`Xabc`] is based on code from user *EternityForest*, published in 
  [Electro-Tech-Online.com][1] in dec 2011.
- [`Xyza8a`] and [`Xyza8b`] are based on code from Edward Rosten,
  published in [8bit_rng][2], licensed as [BSD-2][2L].

[`Mult13P1`]: https://docs.rs/devela/latest/alazar/misc/strut.Mult13P1.html
[0]: https://archive.org/details/BYTE_Vol_02-11_1977-11_Sweet_16/page/n219/
[`Xabc`]: https://docs.rs/devela/latest/alazar/misc/strut.Xabc.html
[1]: https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
[`Xyza8a`]: https://docs.rs/devela/latest/alazar/xorshift/strut.Xyza8a.html
[`Xyza8b`]: https://docs.rs/devela/latest/alazar/xorshift/strut.Xyza8b.html
[2]: https://github.com/edrosten/8bit_rng
[2L]: https://github.com/andamira/alazar/blob/main/src/xorshift/xyza8/LICENSE-BSD2.md

## Contributing

Contributions are welcomed to help refine and improve this library over time.
If you notice a bug, have an idea for a new feature, or simply want to
suggest improvements to the existing codebase, please get in touch.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by shall be dual licensed as above,
without any additional terms or conditions.
