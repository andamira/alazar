# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

## [0.0.2] - 2023-09-08

### Changed
- remove `std` from default features.
- deprecate and rename `all` to `full`.
- update `devela` to `0.9.0`.

### Fixed
- fix misc. prngs overflow.
- refactor manifest.

## [0.0.1] - 2023-09-01

### Added
- Add PRNGS: `Mult13P1`, `Xabc`, `XorShift8`, `XorShift8Custom`, `XorShift16`, `XorShift32`, `XorShift64`, `XorShift128`, `XorShift128p`, `Xyza8a`, `Xyza8b`.
- Add optional rand_core dependency for implementing `RngCore` and `SeedableRng`.


[unreleased]: https://github.com/andamira/depura/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/andamira/depura/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/depura/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
