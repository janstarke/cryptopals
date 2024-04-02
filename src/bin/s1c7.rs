use std::fs::File;

use anyhow::Result;
use cryptopals::Bytes;
use encoding_rs::WINDOWS_1252;
use openssl::symm::decrypt;
use openssl::symm::Cipher;

fn main() -> Result<()> {
    let data = Bytes::from_base64_stream(File::open("data/7.txt")?)?;
    let key = Bytes::from_ascii("YELLOW SUBMARINE");
    let plaintext = Bytes::from(decrypt(Cipher::aes_128_ecb(), &key[..], None, &data[..])?);
    let plaintext = plaintext.to_string(WINDOWS_1252).0;
    println!("{plaintext}");
    Ok(())
}
