# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [1.1.0] - 2020-11-12
- Add `get_bank_by_bic` to fetch a bank by its BIC

## [1.0.6] - 2020-11-09
- Actually publish all binaries.

## [1.0.5] - 2020-11-09
- Add binaries for more architectures to releases.
- Bump deps.

## [1.0.4] - 2020-10-07
- Fix docs.rs build.
- Update bank list.
- Bump deps.

## [1.0.3] - 2020-07-23
- Fix binary artifact name to ensure that binary builds work again.

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
[Unreleased]: https://github.com/svenstaro/fints-institute-db/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.6...v1.1.0
[1.0.6]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/svenstaro/fints-institute-db/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/svenstaro/miniserve/compare/0.4.0...v1.0.0
