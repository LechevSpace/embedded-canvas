# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Removed

## [0.2.0] - 2022-08-08

### Added

- Feature `alloc` - enables `Canvas` & `CanvasAt`
- `CCanavas` & `CCanvasAt` - const generic alternatives to `Canvas` & `CanvasAt` which don't use `alloc`.

## [0.1.2] - 2022-06-24

### Changed

- Fix error on docs.rs regarding `doc(cfg)` feature flag being an unstable feature.
  See docs.rs build for `v0.1.1` <https://docs.rs/crate/embedded-canvas/0.1.1/builds/581694>

## [0.1.1] - 2022-06-24

### Added

- Badges [#3](https://github.com/LechevSpace/embedded-canvas/issues/3)
- Examples for transparency & cropping with screenshots in both Readme & lib.rs [#1](https://github.com/LechevSpace/embedded-canvas/issues/1)
- Show feature flags implementation in docs.rs (for feature transform)


## [0.1.0] - 2022-06-16

The first release of `embedded-canvas` :tada:!

This first release is `no_std` but requires `alloc` for embedded devices.

### Added

- `Canvas` - a canvas with origin `Point::zero()` and no set location on the provided display.
- `CanvasAt` - a type of canvas ready to be drawn on the display at the specified location.

**Read the full documentation at**: https://docs.rs/embedded-canvas/0.1.0

**Full Changelog**: https://github.com/LechevSpace/embedded-canvas/commits/v0.1.0


[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.1...v0.2.0
[0.1.2]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.1.0