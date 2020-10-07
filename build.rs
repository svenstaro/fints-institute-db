use std::env;
use std::fs;

static HBCI4J_COMMIT: &str = "dfc8f67773580d9ba2c20f8b9876355364f46ffe";

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
