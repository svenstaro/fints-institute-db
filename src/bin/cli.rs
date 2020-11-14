use iban::Iban;
use std::error::Error;
use structopt::clap::ArgGroup;
use structopt::StructOpt;

use fints_institute_db::{get_bank_by_bank_code, get_bank_by_bic};

fn criterion_group() -> ArgGroup<'static> {
    ArgGroup::with_name("criterion").required(true)
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "fints-institute-db",
    author,
    about,
    global_settings = &[structopt::clap::AppSettings::ColoredHelp],
    group = criterion_group(),
)]
struct CliConfig {
    /// Look up bank by IBAN (format: DE02120300000000202051)
    #[structopt(long, group = "criterion")]
    iban: Option<Iban>,

    /// Look up bank by German bank code (format: 12030000)
    #[structopt(long = "bankcode", group = "criterion")]
    bank_code: Option<String>,

    /// Look up bank by Bank Identifier Code (BIC) (format: GENODEM1MEN)
    #[structopt(long = "bic", group = "criterion")]
    bic: Option<String>,

    /// Change tool behavior to output all data for the record as JSON
    #[structopt(short = "j", long = "json")]
    print_json: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliConfig::from_args();

    let bank = if let Some(iban) = args.iban {
        if let Some(identifier) = iban.bank_identifier() {
            get_bank_by_bank_code(identifier)
        } else {
            eprintln!("Invalid or missing IBAN bank identifier");
            std::process::exit(1);
        }
    } else if let Some(bank_code) = args.bank_code {
        get_bank_by_bank_code(&bank_code)
    } else if let Some(bic) = args.bic {
        get_bank_by_bic(&bic)
    } else {
        unreachable!("Can't touch this.")
    };

    if let Some(bank) = bank {
        if args.print_json {
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
