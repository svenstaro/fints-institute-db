extern crate chrono;
extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use chrono::prelude::*;
use serde::{Deserialize, Deserializer};

static BANKS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fints_institute.csv"));

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
    #[serde(rename(deserialize = "HBCI-Zugang DNS"))]
    pub hbci_domain: String,
    #[serde(rename(deserialize = "HBCI- Zugang     IP-Adresse"))]
    pub hbci_address: String,
    #[serde(rename(deserialize = "HBCI-Version"))]
    pub hbci_version: String,
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
    #[serde(rename(deserialize = "PIN/TAN-Zugang URL"))]
    pub pin_tan_url: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "Datum letzte Änderung"), deserialize_with = "from_german_date")]
    pub updated: Option<NaiveDate>,
}

/// Retrieves all banks with `bank_code`.
///
/// Use this only if you want to receive all branches of a certain type of bank.
/// For FinTS purposes the specific branch of the bank shouldn't matter as the FinTS access
/// data should be the same across all branches.
pub fn get_banks_by_bank_code(bank_code: &str) -> Vec<Bank> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(BANKS);
    let mut banks = rdr.deserialize()
        .map(|r: Result<Bank, _>| r.unwrap())
        .collect::<Vec<_>>();
    banks.retain(|bank| bank.bank_code == bank_code);
    banks
}

/// Retrieves the first bank with `bank_code`
///
/// Usually this is what you want unless you care about specific bank branches.
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
