use std::env;
use std::fs;

static HBCI4J_COMMIT: &'static str = "0072d401187a69a451eae1cbe928da3e2be6109f";

fn main() -> Result<(), Box<std::error::Error>> {
    let final_url = format!(
        "https://raw.githubusercontent.com/hbci4j/hbci4java/{commit}/src/main/resources/blz.properties",
	commit=HBCI4J_COMMIT,
    );
    let mut res = reqwest::get(&final_url)?;

    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf)?;

    let filename = final_url.split("/").last().unwrap();
    let out_file = format!("{out_dir}/{filename}", out_dir=env::var("OUT_DIR")?, filename=filename);
    fs::write(out_file, &*buf)?;
    Ok(())
}
