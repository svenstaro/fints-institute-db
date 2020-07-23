# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [1.0.2] - 2020-07-23
- Get rid of OpenSSL dependency by instead making use of RustLS.
  This should decrease binary size and make the lib more portable as it's now pure Rust.

## [1.0.1] - 2020-07-23
- Check in `Cargo.lock` to make sure the binary build is reproducible.

## [1.0.0] - 2020-07-23
- Update institutes database.
- Bump deps.
- Modernize crate.
- Switch away from Travis to GitHub Actions.

<!-- next-url -->
[Unreleased]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.2...HEAD
[1.0.2]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/svenstaro/miniserve/compare/0.4.0...v1.0.0
