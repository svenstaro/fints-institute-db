#![cfg(feature = "cli")]

use std::process::Command;

use assert_cmd::prelude::*;

type Error = Box<dyn std::error::Error>;

#[test]
fn get_bank_by_bank_code() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--bankcode")
        .arg("12030000")
        .assert()
        .success();

    Ok(())
}

#[test]
fn invalid_bank_code() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--bankcode")
        .arg("test")
        .assert()
        .stderr("No matching bank found\n")
        .failure();

    Ok(())
}

#[test]
fn get_bank_by_iban() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--iban")
        .arg("DE02120300000000202051")
        .assert()
        .success();

    Ok(())
}

#[test]
fn invalid_iban() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--iban")
        .arg("test")
        .assert()
        .stderr(predicates::str::starts_with("error: Invalid value 'test' for '--iban <IBAN>': the string does not follow the base IBAN rules\n"))
        .failure();

    Ok(())
}

#[test]
fn get_bank_by_bic() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--bic")
        .arg("GENODEM1MEN")
        .assert()
        .success();

    Ok(())
}

#[test]
fn invalid_bic() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--bic")
        .arg("test")
        .assert()
        .stderr("No matching bank found\n")
        .failure();

    Ok(())
}

#[test]
fn bank_exists_but_has_no_pin_tan_url() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--bankcode")
        .arg("59010400")
        .assert()
        .stderr("This bank has no available PIN TAN URL\n")
        .failure();

    Ok(())
}

#[test]
fn json_output() -> Result<(), Error> {
    let stdout = Command::cargo_bin("cli")?
        .arg("--bankcode")
        .arg("12030000")
        .arg("--json")
        .assert()
        .get_output()
        .stdout
        .clone();

    serde_json::from_slice::<serde_json::Value>(&stdout)?;
    Ok(())
}

#[test]
fn missing_iban_bank_identifier() -> Result<(), Error> {
    Command::cargo_bin("cli")?
        .arg("--iban")
        .arg("PL30109024023716983694698824")
        .assert()
        .stderr("Invalid or missing IBAN bank identifier\n")
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
        .stdout(format!(
            "{} {}\n",
            clap::crate_name!(),
            clap::crate_version!()
        ));

    Ok(())
}
