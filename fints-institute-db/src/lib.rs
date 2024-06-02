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

use serde::{Deserialize, Serialize};
use std::str::FromStr;

static BANKS: &str = include_str!(concat!(env!("OUT_DIR"), "/blz.properties"));

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
            rdh_address: if second_part[4].is_empty() {
                None
            } else {
                Some(second_part[4].to_string())
            },
            pin_tan_address: if second_part[5].is_empty() {
                None
            } else {
                Some(second_part[5].to_string())
            },
            rdh_version: if second_part[6].is_empty() {
                None
            } else {
                Some(second_part[6].to_string())
            },
            pin_tan_version: if second_part[7].is_empty() {
                None
            } else {
                Some(second_part[7].to_string())
            },
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

/// Retrieves the bank by its `bic`
///
/// # Examples
///
/// ```
/// use fints_institute_db::get_bank_by_bic;
///
/// let bank = get_bank_by_bic("GENODEM1MEN");
/// println!("{:?}", bank);
/// ```
pub fn get_bank_by_bic(bic: &str) -> Option<Bank> {
    BANKS
        .lines()
        .map(Bank::from_str)
        .filter_map(Result::ok)
        .find(|b| b.bic == bic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// assertion helper to keep the bank find tests DRY
    fn assert_bank_matches(bank: &Bank) {
        assert_eq!(bank.bank_code, "44761312");
        assert_eq!(bank.institute, "Mendener Bank");
        assert_eq!(bank.location, "Menden (Sauerland)");
        assert_eq!(bank.bic, "GENODEM1MEN");
        assert_eq!(bank.checksum_method, "34");
        assert_eq!(bank.rdh_address, Some("hbci.gad.de".to_string()));
        assert_eq!(
            bank.pin_tan_address,
            Some("https://fints1.atruvia.de/cgi-bin/hbciservlet".to_string())
        );
        assert_eq!(bank.rdh_version, Some("300".to_string()));
        assert_eq!(bank.pin_tan_version, Some("300".to_string()));
    }

    // These tests use the real CSV input data.
    // As such, the tests might break randomly if the input document changes.
    // This should happen rarely enough that it's still worthwhile to test with real data, though.

    #[test]
    fn get_bank_by_bank_code_test() {
        let bank_code = "44761312";
        let bank = get_bank_by_bank_code(bank_code).unwrap();

        assert_bank_matches(&bank);
    }

    #[test]
    fn get_bank_by_bic_test() {
        let bic = "GENODEM1MEN";
        let bank = get_bank_by_bic(bic).unwrap();

        assert_bank_matches(&bank);
    }
}

#[cfg(doctest)]
doc_comment::doctest!("../../README.md", readme);
