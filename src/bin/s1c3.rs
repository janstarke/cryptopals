use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::FindSingleXorKey;
use cryptopals::CharacterFrequency;
use encoding_rs::WINDOWS_1252;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination.
fn main() -> Result<()> {
    let input =
        Bytes::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let key_candidates = input.sort_single_xor_keys(|s| s.simple_english_score());
    let (key, score) = key_candidates.first().unwrap();
    let decrypted = &input ^ &key.into();
    println!(
        "found key 0x{key} with score {score}, result is '{decrypted:?}'",
        decrypted = decrypted.to_string(WINDOWS_1252).0
    );
    Ok(())
}
