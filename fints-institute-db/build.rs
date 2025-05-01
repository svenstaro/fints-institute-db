use std::env;
use std::fs;

// Check here for latest updates:
// https://github.com/hbci4j/hbci4java/blob/master/src/main/resources/blz.properties
static HBCI4J_COMMIT: &str = "91cb7520a5d47e4e913d4391f31107a949184e05";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let final_url = format!(
        "https://raw.githubusercontent.com/hbci4j/hbci4java/{HBCI4J_COMMIT}/src/main/resources/blz.properties",
    );

    let mut buf: Vec<u8> = vec![];

    if env::var("DOCS_RS").is_err() {
        let mut res = reqwest::blocking::get(&final_url)?;
        res.copy_to(&mut buf)?;
    } else {
        // Fake data for docs.rs because it doesn't have network access.
        buf.push(88);
    }

    let filename = final_url.split('/').next_back().unwrap();
    let out_file = format!(
        "{out_dir}/{filename}",
        out_dir = env::var("OUT_DIR")?,
        filename = filename
    );
    fs::write(out_file, &*buf)?;
    Ok(())
}
