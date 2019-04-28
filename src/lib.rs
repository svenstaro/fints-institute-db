//! This is a simple crate providing a convenient and safe interface to FinTS information of German
//! banks. During the build it will download a file with all the banks which it will then put
//! into the library itself so that no extra files have to be taken care of.
//!
//! Usage is easy:
//!
//! # Examples
//!
//! ```
//! use fints_institute_db::get_bank_by_bank_code;
//!
//! let bank = get_bank_by_bank_code("12070000").unwrap();
//! println!("{:?}", bank.pin_tan_address);
//! ```

use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

static BANKS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/blz.properties"));

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Bank {
    pub bank_code: String,
    pub institute: String,
    pub location: String,
    pub bic: String,
    pub checksum_method: String,
    pub rdh_address: Option<String>,
    pub pin_tan_address: Option<String>,
    pub rdh_version: Option<String>,
    pub pin_tan_version: Option<String>,
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It's a bit of weird format that looks like this:
        // bank_code=institute|location|bic|checksum_method|rdh_address|pin_tan_address|rdh_version|pin_tan_version
        let first_part: Vec<&str> = s.split('=').collect();
        let second_part: Vec<&str> = first_part[1].split('|').collect();
        let bank = Bank {
            bank_code: first_part[0].to_string(),
            institute: second_part[0].to_string(),
            location: second_part[1].to_string(),
            bic: second_part[2].to_string(),
            checksum_method: second_part[3].to_string(),
            rdh_address: second_part.get(4).map(|x| x.to_string()),
            pin_tan_address: second_part.get(5).map(|x| x.to_string()),
            rdh_version: second_part.get(6).map(|x| x.to_string()),
            pin_tan_version: second_part.get(7).map(|x| x.to_string()),
        };
        Ok(bank)
    }
}

/// Retrieves the first bank with `bank_code`
///
/// Usually this is what you want unless you care about specific bank branches.
///
/// # Examples
///
/// ```
/// use fints_institute_db::get_bank_by_bank_code;
///
/// let bank = get_bank_by_bank_code("12070000");
/// println!("{:?}", bank);
/// ```
pub fn get_bank_by_bank_code(bank_code: &str) -> Option<Bank> {
    for bank in BANKS.lines() {
        let first_part: Vec<&str> = bank.split('=').collect();
        let current_bank_code = first_part[0];
        if current_bank_code == bank_code {
            return Some(Bank::from_str(bank).expect(
                    "Invalid bank format found in source file.
                    This definitely shouldn't happen and is a serious issue with the bank info source file."
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // These tests use the real CSV input data.
    // As such, the tests might break randomly if the input document changes.
    // This should happen rarely enough that it's still worthwhile to test with real data, though.

    #[test]
    fn getting_random_bank_works() {
        let bank_code = "44761312";
        let bank = get_bank_by_bank_code(bank_code).unwrap();

        assert_eq!(bank.bank_code, bank_code);
        assert_eq!(bank.institute, "Mendener Bank");
        assert_eq!(bank.location, "Menden (Sauerland)");
        assert_eq!(bank.bic, "GENODEM1MEN");
        assert_eq!(bank.checksum_method, "34");
        assert_eq!(bank.rdh_address, Some("hbci.gad.de".to_string()));
        assert_eq!(bank.pin_tan_address, Some("https://hbci-pintan.gad.de/cgi-bin/hbciservlet".to_string()));
        assert_eq!(bank.rdh_version, Some("300".to_string()));
        assert_eq!(bank.pin_tan_version, Some("300".to_string()));
    }
}
