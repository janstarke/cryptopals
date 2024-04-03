use anyhow::Result;
use cryptopals::{Bytes, PadWith, Pkcs7};
use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    let mut input = Bytes::from_ascii("YELLOW SUBMARINE");
    input.pad_with(20, Pkcs7)?;
    println!("{:?}", input.to_string(WINDOWS_1252).0);
    Ok(())
}