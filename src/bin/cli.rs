#[cfg(feature = "cli")]
use std::{error::Error, io};

#[cfg(feature = "cli")]
use clap::{ArgGroup, CommandFactory, Parser};
#[cfg(feature = "cli")]
use iban::Iban;

#[cfg(feature = "cli")]
use fints_institute_db::{get_bank_by_bank_code, get_bank_by_bic};

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "fints-institute-db", author, about, version)]
#[command(group(ArgGroup::new("mode").args(["iban", "bank_code", "bic"])))]
struct CliArgs {
    /// Look up bank by IBAN (format: DE02120300000000202051)
    #[arg(long)]
    iban: Option<Iban>,

    /// Look up bank by German bank code (format: 12030000)
    #[arg(long = "bankcode")]
    bank_code: Option<String>,

    /// Look up bank by Bank Identifier Code (BIC) (format: GENODEM1MEN)
    #[arg(long = "bic")]
    bic: Option<String>,

    /// Change tool behavior to output all data for the record as JSON
    #[arg(short = 'j', long = "json")]
    print_json: bool,

    /// Generate completion file for a shell
    #[arg(long = "print-completions", value_name = "shell")]
    pub print_completions: Option<clap_complete::Shell>,

    /// Generate man page
    #[arg(long = "print-manpage")]
    pub print_manpage: bool,
}

#[cfg(feature = "cli")]
fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    if let Some(shell) = args.print_completions {
        let mut clap_app = CliArgs::command();
        let app_name = clap_app.get_name().to_string();
        clap_complete::generate(shell, &mut clap_app, app_name, &mut io::stdout());
        return Ok(());
    }

    if args.print_manpage {
        let clap_app = CliArgs::command();
        let man = clap_mangen::Man::new(clap_app);
        man.render(&mut io::stdout())?;
        return Ok(());
    }

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
        eprintln!("No lookup mode specified");
        std::process::exit(1);
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

#[cfg(not(feature = "cli"))]
fn main() {
}
