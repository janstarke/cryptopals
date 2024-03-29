use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::FindSingleXorKey;
use cryptopals::ENGLISH;
use encoding_rs::WINDOWS_1252;
use cryptopals::CharacterFrequency;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination.
fn main() -> Result<()> {
    let input =
        Bytes::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let key = 0x58;
    let key_bytes = Bytes::from(&[key][..]);
    let result = Bytes::xor(&input, &key_bytes);
    let content = result.to_string(encoding_rs::WINDOWS_1252).0;

    let score = content.to_lowercase().language_score(&ENGLISH);
    
    for (key, score) in input.sort_single_xor_keys(&ENGLISH) {
        let decrypted = &input ^ &key;
        println!(
            "found key 0x{key} with score {score}, result is '{decrypted:?}'",
            decrypted = decrypted.to_string(WINDOWS_1252).0
        )
    }
    
    Ok(())
}
