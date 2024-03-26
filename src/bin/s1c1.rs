use anyhow::Result;
use cryptopals::Bytes;

/// Convert hex to base64
fn main() -> Result<()> {
    let input = Bytes::from_hex("49276d206b696c6c696e6720796f757220627261696\
    e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")?;
    assert_eq!(input.to_base64(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    println!("{output}", output=input.to_base64());
    Ok(())
}
