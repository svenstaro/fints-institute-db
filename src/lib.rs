//! This is a simple crate providing a convenient and safe interface to FinTS information of German
//! banks. During the build it will download a CSV file with all the banks which it will then put
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
//! println!("{:?}", bank.pin_tan_url);
//! ```

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use serde::{Deserialize, Deserializer};

static BANKS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fints_institute.csv"));

fn empty_or_unsupported<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.is_empty() || value == "nicht unterstützt" {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

fn from_german_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let german_bool = String::deserialize(deserializer)?;
    if german_bool == "ja" {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn from_german_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let german_date = String::deserialize(deserializer)?;
    if german_date.is_empty() {
        return Ok(None);
    }
    let parsed_date =
        NaiveDate::parse_from_str(&german_date, "%d.%m.%Y").map_err(serde::de::Error::custom)?;
    Ok(Some(parsed_date))
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Bank {
    #[serde(rename(deserialize = "BLZ"))]
    pub bank_code: String,
    #[serde(rename(deserialize = "Institut"))]
    pub institute: String,
    #[serde(rename(deserialize = "Ort"))]
    pub location: String,
    #[serde(rename(deserialize = "RZ"))]
    pub data_center: String,
    #[serde(rename(deserialize = "Organisation"))]
    pub organisation: String,
    #[serde(rename(deserialize = "HBCI-Zugang DNS"), deserialize_with = "empty_or_unsupported")]
    pub hbci_domain: Option<String>,
    #[serde(rename(deserialize = "HBCI- Zugang     IP-Adresse"), deserialize_with = "empty_or_unsupported")]
    pub hbci_address: Option<String>,
    #[serde(rename(deserialize = "HBCI-Version"), deserialize_with = "empty_or_unsupported")]
    pub hbci_version: Option<String>,
    #[serde(rename(deserialize = "DDV"), deserialize_with = "from_german_bool")]
    pub ddv: bool,
    #[serde(rename(deserialize = "RDH-1"), deserialize_with = "from_german_bool")]
    pub rdh1: bool,
    #[serde(rename(deserialize = "RDH-2"), deserialize_with = "from_german_bool")]
    pub rdh2: bool,
    #[serde(rename(deserialize = "RDH-3"), deserialize_with = "from_german_bool")]
    pub rdh3: bool,
    #[serde(rename(deserialize = "RDH-4"), deserialize_with = "from_german_bool")]
    pub rdh4: bool,
    #[serde(rename(deserialize = "RDH-5"), deserialize_with = "from_german_bool")]
    pub rdh5: bool,
    #[serde(rename(deserialize = "RDH-6"), deserialize_with = "from_german_bool")]
    pub rdh6: bool,
    #[serde(rename(deserialize = "RDH-7"), deserialize_with = "from_german_bool")]
    pub rdh7: bool,
    #[serde(rename(deserialize = "RDH-8"), deserialize_with = "from_german_bool")]
    pub rdh8: bool,
    #[serde(rename(deserialize = "RDH-9"), deserialize_with = "from_german_bool")]
    pub rdh9: bool,
    #[serde(rename(deserialize = "RDH-10"), deserialize_with = "from_german_bool")]
    pub rdh10: bool,
    #[serde(rename(deserialize = "RAH-7"), deserialize_with = "from_german_bool")]
    pub rah7: bool,
    #[serde(rename(deserialize = "RAH-9"), deserialize_with = "from_german_bool")]
    pub rah9: bool,
    #[serde(rename(deserialize = "RAH-10"), deserialize_with = "from_german_bool")]
    pub rah10: bool,
    #[serde(rename(deserialize = "PIN/TAN-Zugang URL"), deserialize_with = "empty_or_unsupported")]
    pub pin_tan_url: Option<String>,
    #[serde(rename(deserialize = "Version"), deserialize_with = "empty_or_unsupported")]
    pub version: Option<String>,
    #[serde(rename(deserialize = "Datum letzte Änderung"), deserialize_with = "from_german_date")]
    pub updated: Option<NaiveDate>,
}

/// Retrieves all banks with `bank_code`.
///
/// Use this only if you want to receive all branches of a certain type of bank.
/// For FinTS purposes the specific branch of the bank shouldn't matter as the FinTS access
/// data should be the same across all branches.
///
/// # Examples
///
/// ```
/// use fints_institute_db::get_banks_by_bank_code;
///
/// let banks = get_banks_by_bank_code("12070000");
/// println!("{:#?}", banks);
/// ```
pub fn get_banks_by_bank_code(bank_code: &str) -> Vec<Bank> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(BANKS);
    let mut banks = rdr
        .deserialize()
        .map(|r: Result<Bank, _>| r.unwrap())
        .collect::<Vec<_>>();
    banks.retain(|bank| bank.bank_code == bank_code);
    banks
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
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(BANKS);
    for result in rdr.deserialize() {
        let bank: Bank = result.unwrap();
        if bank.bank_code == bank_code {
            return Some(bank);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests use the real CSV input data.
    // As such, the tests might break randomly if the input document changes.
    // This should happen rarely enough that it's still worthwhile to test with real data, though.

    #[test]
    fn getting_random_bank_works() {
        let bank_code = "44761312";
        let bank = get_bank_by_bank_code(bank_code).unwrap();

        assert_eq!(bank.bank_code, bank_code);
        assert_eq!(bank.hbci_domain, Some("hbci.gad.de".to_string()));
        assert_eq!(bank.hbci_address, None);
        assert_eq!(bank.hbci_version, Some("3.0".to_string()));
        assert_eq!(bank.pin_tan_url, Some("https://hbci-pintan.gad.de/cgi-bin/hbciservlet".to_string()));
        assert_eq!(bank.hbci_version, Some("3.0".to_string()));
        assert_eq!(bank.updated, Some(NaiveDate::from_ymd(2016, 4, 28)));
    }

    #[test]
    fn getting_multiple_banks_works() {
        let bank_code = "44570004";
        let banks = get_banks_by_bank_code(bank_code);

        assert_eq!(banks.len(), 7);
    }
}
