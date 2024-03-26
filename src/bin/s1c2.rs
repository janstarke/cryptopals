use anyhow::Result;
use cryptopals::Bytes;

/// Fixed XOR
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination. 
fn main() -> Result<()> {
    let input = Bytes::from_hex(
        "1c0111001f010100061a024b53535009181c")?;
    let key = Bytes::from_hex(
        "686974207468652062756c6c277320657965")?;
    let output = (input ^ key).to_hex();
    println!("{output}");
    Ok(())
}
