use clap::{
    crate_authors, crate_name, crate_version, value_t_or_exit, App, AppSettings, Arg, ArgGroup,
};
use iban::{BbanResult, Iban};

use fints_institute_db::get_bank_by_bank_code;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tool to query the FinTS database")
        .global_setting(AppSettings::ColoredHelp)
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
    let print_json = matches.is_present("json");

    let bank = get_bank_by_bank_code(&bank_code);
    if let Some(bank) = bank {
        if print_json {
            println!("{}", serde_json::to_string_pretty(&bank)?);
        } else if let Some(pin_tan_url) = bank.pin_tan_address {
            println!("{}", pin_tan_url);
        } else {
            eprintln!("This bank has no available PIN TAN URL");
            std::process::exit(1);
        }
    } else {
        eprintln!("No matching bank found");
        std::process::exit(1);
    }
    Ok(())
}
