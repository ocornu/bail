# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

## [0.2.0] - 2026-06-16

### Added
- Added the `None` extension trait for `bool` to support short-circuiting functions that return an `Option<T>`.
- Comprehensive unit tests covering the new `Option` control flow paths.
- Updated documentation and `README.md` examples demonstrating `Option` usage.
- Added this `CHANGELOG.md`.

## [0.1.1] - 2026-06-16

### Changed
- Leaner `Err()` implementation returns a `Result<(), E>` (instead of `Result<bool, E>`) to prevent any 'unused' compiler warning.

## [0.1.0] - 2026-06-16

### Added
- Initial release of the `bail` crate.
- Added the `Err` extension trait for `bool` to support functional method-chaining early-exits with `Result<T, E>`.
- Fully compliant `#![no_std]` support for embedded systems, kernel modules, and WebAssembly environments.
- Baseline unit test suite.