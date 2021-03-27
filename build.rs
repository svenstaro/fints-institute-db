use std::env;
use std::fs;

// Check here for latest updates:
// https://github.com/hbci4j/hbci4java/blob/master/src/main/resources/blz.properties
static HBCI4J_COMMIT: &str = "b67343789d536661e4751f2d04aa1b5486c01d08";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let final_url = format!(
        "https://raw.githubusercontent.com/hbci4j/hbci4java/{commit}/src/main/resources/blz.properties",
        commit=HBCI4J_COMMIT,
    );

    let mut buf: Vec<u8> = vec![];

    if env::var("DOCS_RS").is_err() {
        let mut res = reqwest::blocking::get(&final_url)?;
        res.copy_to(&mut buf)?;
    } else {
        // Fake data for docs.rs because it doesn't have network access.
        buf.push(88);
    }

    let filename = final_url.split('/').last().unwrap();
    let out_file = format!(
        "{out_dir}/{filename}",
        out_dir = env::var("OUT_DIR")?,
        filename = filename
    );
    fs::write(out_file, &*buf)?;
    Ok(())
}
