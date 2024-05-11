# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/LechevSpace/embedded-canvas/compare/v0.3.0...v0.3.1) - 2024-05-11

### Added
- *(release-plz)* config for replacing PR and issue `#..` with urls
- *(ci)* dependabot.yml for outdated actions

### Fixed
- *(release-plz)* config - add commit_preprocessors
- *(consts)* CCanvas - switched Width and Hight in pixels array

### Other
- Merge branch 'main' into dependabot/github_actions/actions/checkout-4
- release-plz PR workflow
- *(Cargo.toml)* Upgrade dependencies

### Added

### Changed

### Removed

## [0.3.0] - 2023-11-06

### Added

- `#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]` in `lib.rs` for `canvas` module.

### Changed
- Updated dependencies: `embedded-graphics-core@0.4` `embedded-graphics@0.8` & dev-dependency `embedded-graphics-simulator@0.5` - [PR #11](https://github.com/LechevSpace/embedded-canvas/pull/11)
- Bumped MSRV from 1.56 to 1.71 [PR #12](https://github.com/LechevSpace/embedded-canvas/pull/12)

### Removed

## [0.2.0] - 2022-08-09

### Added

- Feature `alloc` - enables `Canvas` & `CanvasAt`
- `CCanavas` & `CCanvasAt` - const generic alternatives to `Canvas` & `CanvasAt` which don't use `alloc` and update docs and README to include them
- Fix typos in `Cargo.toml`

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


[Unreleased]: https://github.com/LechevSpace/embedded-canvas/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/LechevSpace/embedded-canvas/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/LechevSpace/embedded-canvas/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/LechevSpace/embedded-canvas/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/LechevSpace/embedded-canvas/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/LechevSpace/embedded-canvas/releases/tag/v0.1.0