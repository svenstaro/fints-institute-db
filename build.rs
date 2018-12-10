use std::env;
use std::fs;
use encoding_rs::ISO_8859_15;

static FINTS_INSTITUTE_DB_VERSION: &'static str = "0.9.0";

fn main() -> Result<(), Box<std::error::Error>> {
    let final_url = format!(
        "https://github.com/jhermsmeier/fints-institute-db/raw/v{version}/src/fints_institute.csv",
        version = FINTS_INSTITUTE_DB_VERSION
    );
    let filename = final_url.split("/").last().unwrap();
    let mut res = reqwest::get(&final_url)?;

    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf)?;

    let (decoded, _, had_errors) = ISO_8859_15.decode(&buf);

    if had_errors {
        panic!("There were errors while decoding the file at {url}", url=final_url);
    }

    let out_file = format!("{out_dir}/{filename}", out_dir=env::var("OUT_DIR")?, filename=filename);
    fs::write(out_file, &*decoded)?;
    Ok(())
}
