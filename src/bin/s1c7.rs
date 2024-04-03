use std::fs::File;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::AES;
use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    let data = Bytes::from_base64_stream(File::open("data/7.txt")?)?;
    let key = Bytes::from_ascii("YELLOW SUBMARINE");
    let plaintext = data.aes_ecb(cryptopals::Mode::Decrypt, &key.try_into().unwrap())?;
    let plaintext = plaintext.to_string(WINDOWS_1252).0;
    println!("{plaintext}");
    Ok(())
}
