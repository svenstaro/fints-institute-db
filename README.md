# fints-institute-db
[![CI](https://github.com/svenstaro/fints-institute-db/workflows/CI/badge.svg)](https://github.com/svenstaro/fints-institute-db/actions)
[![Crates.io](https://img.shields.io/crates/v/fints-institute-db.svg)](https://crates.io/crates/fints-institute-db)
[![docs](https://docs.rs/fints-institute-db/badge.svg)](https://docs.rs/fints-institute-db)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/svenstaro/fints-institute-db/blob/master/LICENSE)
[![Lines of Code](https://tokei.rs/b1/github/svenstaro/fints-institute-db)](https://github.com/svenstaro/fints-institute-db)

This is a simple crate providing a convenient and safe interface to FinTS information of German banks.
During the build it will download a CSV file with all the banks which it will then put into the library itself so that no extra files have to be taken care of.

## Usage

Put this into your `Cargo.toml`:

    [dependencies]
    fints_institute_db = "1"

Then to use it:

```rust
use fints_institute_db::get_bank_by_bank_code;

let bank = get_bank_by_bank_code("12070000");
println!("{:?}", bank.pin_tan_url);
```

## Command line utility

Additionally this crate includes a CLI tool for your convenience:

    fints-institute-db 1.0.4
    Sven-Hendrik Haase <svenstaro@gmail.com>
    Tool to query the FinTS database.

    By default it will return just the FinTS URL for the first matching bank.

    USAGE:
        cli [FLAGS] [OPTIONS] <--iban <iban>|--bankcode <bank_code>>

    FLAGS:
        -h, --help       Prints help information
        -j, --json       Change tool behavior to output all data for the record as JSON
        -V, --version    Prints version information

    OPTIONS:
        -b, --bankcode <bank_code>    Look up bank by German bank code (format: 12030000)
        -i, --iban <iban>             Look up bank by IBAN (format: DE02120300000000202051)

Example usages:

    cargo run -- -b 12030000

    cargo run -- -i DE02120300000000202051

This crate is inspired by https://github.com/jhermsmeier/fints-institute-db and https://github.com/dr-duplo/python-fints-url

## Releasing

This is mostly a note for me on how to release this thing:

- `cargo release --dry-run`
- `cargo release`
- Releases will automatically be deployed by Github Actions.
