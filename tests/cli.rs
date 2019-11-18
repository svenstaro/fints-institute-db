use assert_cmd::prelude::*;
use clap::{crate_name, crate_version};
use std::process::Command;

type Error = Box<dyn std::error::Error>;

#[test]
fn get_bank_by_bank_code() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("-b")
        .arg("12030000")
        .assert()
        .success();

    Ok(())
}

#[test]
fn invalid_bank_code() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("-b")
        .arg("test")
        .assert()
        .stderr("No matching bank found\n")
        .failure();

    Ok(())
}

#[test]
fn get_bank_by_iban() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("-i")
        .arg("DE02120300000000202051")
        .assert()
        .success();

    Ok(())
}

#[test]
fn invalid_iban() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("-i")
        .arg("test")
        .assert()
        .stderr("error: Invalid value for '--iban <iban>': Invalid IBAN format\n")
        .failure();

    Ok(())
}

#[test]
fn help_shows() -> Result<(), Error> {
    Command::cargo_bin("cli")?.arg("-h").assert().success();

    Ok(())
}

#[test]
fn version_shows() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("-V")
        .assert()
        .success()
        .stdout(format!("{} {}\n", crate_name!(), crate_version!()));

    Ok(())
}
