# fints-institute-db
[![Build Status](https://travis-ci.com/svenstaro/fints-institute-db.svg?branch=master)](https://travis-ci.com/svenstaro/fints-institute-db)
[![Crates.io](https://img.shields.io/crates/v/fints-institute-db.svg)](https://crates.io/crates/fints-institute-db)
[![dependency status](https://deps.rs/repo/github/svenstaro/fints-institute-db/status.svg)](https://deps.rs/repo/github/svenstaro/fints-institute-db)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/svenstaro/fints-institute-db/blob/master/LICENSE)

This is a simple crate providing a convenient and safe interface to FinTS information of German banks.
During the build it will download a CSV file with all the banks which it will then put into the library itself so that no extra files have to be taken care of.

## Usage

Put this into your `Cargo.toml`:

    [dependencies]
    fints_institute_db = "0.2"

Then to use it:

```rust
extern crate fints_institute_db;
use fints_institute_db::get_bank_by_bank_code;
let bank = get_bank_by_bank_code("12070000");
```

## Command line utility

Additionally this crate includes a CLI tool for your convenience:

    fints-institute-db 0.2.0
    Sven-Hendrik Haase <svenstaro@gmail.com>
    Tool to query the FinTS database.
    
    By default it will return just the FinTS URL for the first matching bank.
    
    USAGE:
        cli [FLAGS] [OPTIONS] <--iban <iban>|--bankcode <bank_code>>
    
    FLAGS:
        -a, --all        Print all banks as opposed to just the first matching one (implies --json)
        -h, --help       Prints help information
        -j, --json       Change tool behavior to output all data for the record as JSON
        -V, --version    Prints version information
    
    OPTIONS:
        -b, --bankcode <bank_code>    Look up bank by German bank code (format: 12030000)
        -i, --iban <iban>             Look up bank by IBAN (format: DE02120300000000202051)

Example usage:

    cargo run -- -b 12030000

This crate is inspired by https://github.com/jhermsmeier/fints-institute-db and https://github.com/dr-duplo/python-fints-url
