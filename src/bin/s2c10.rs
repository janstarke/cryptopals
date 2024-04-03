use std::fs::File;

use anyhow::Result;
use cryptopals::{Bytes, Mode, AES, AES_BLOCKSIZE};
use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    let data = Bytes::from_base64_stream(File::open("data/10.txt")?)?;
    let key = Bytes::from_ascii("YELLOW SUBMARINE").try_into()?;
    let iv = Bytes::from(vec![0; AES_BLOCKSIZE]).try_into()?;
    let decrypted = data.aes_cbc(Mode::Decrypt, &key, &iv)?;
    println!("{}", decrypted.to_string(WINDOWS_1252).0);
    Ok(())
}
