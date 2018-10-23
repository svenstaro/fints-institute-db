#[macro_use]
extern crate clap;
extern crate fints_institute_db;
extern crate iban;
extern crate serde_json;

use clap::{App, Arg, ArgGroup};
use fints_institute_db::{get_bank_by_bank_code, get_banks_by_bank_code};
use iban::{BbanResult, Iban};

fn is_iban(iban: String) -> Result<(), String> {
    if let Ok(iban) = iban.parse::<Iban>() {
        match iban.validate_bban() {
            BbanResult::Valid => Ok(()),
            BbanResult::Invalid => Err("Invalid BBAN".to_string()),
            BbanResult::CountryUnknown => Err("Unknown BBAN country".to_string()),
        }
    } else {
        Err("Invalid IBAN format".to_string())
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tool to query the FinTS database")
        .long_about(
            "Tool to query the FinTS database.

By default it will return just the FinTS URL for the first matching bank.",
        )
        .arg(
            Arg::with_name("iban")
                .short("i")
                .long("iban")
                .takes_value(true)
                .validator(is_iban)
                .help("Look up bank by IBAN (format: DE02120300000000202051)"),
        )
        .arg(
            Arg::with_name("bank_code")
                .short("b")
                .long("bankcode")
                .takes_value(true)
                .help("Look up bank by German bank code (format: 12030000)"),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Print all banks as opposed to just the first matching one (implies --json)"),
        )
        .arg(
            Arg::with_name("json")
                .short("j")
                .long("json")
                .help("Change tool behavior to output all data for the record as JSON"),
        )
        .group(
            ArgGroup::with_name("input_mode")
                .args(&["iban", "bank_code"])
                .required(true),
        )
        .get_matches();

    let bank_code = if let Some(iban_string) = matches.value_of("iban") {
        // We already validated this above so we can safely unwrap.
        let iban = iban_string.parse::<Iban>().unwrap();
        let bban = iban.get_bban();
        // We're only interested in German IBANs and so we can safely assume that the bank code
        // will be contained in the first 8 chars of the BBAN.
        bban[..8].to_owned()
    } else {
        value_t_or_exit!(matches, "bank_code", String)
    };
    let print_all = matches.is_present("all");
    let print_json = matches.is_present("json");

    if print_all {
        let banks = get_banks_by_bank_code(&bank_code);
        if banks.is_empty() {
            eprintln!("No matching banks found");
            std::process::exit(1);
        }
        println!("{}", serde_json::to_string_pretty(&banks)?);
    } else {
        let bank = get_bank_by_bank_code(&bank_code);
        if let Some(bank) = bank {
            if print_json {
                println!("{}", serde_json::to_string_pretty(&bank)?);
            } else {
                println!("{}", bank.pin_tan_url);
            }
        } else {
            eprintln!("No matching bank found");
            std::process::exit(1);
        }
    };
    Ok(())
}
