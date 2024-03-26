use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::CharacterFrequency;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination. 
fn main() -> Result<()> {
    let input = Bytes::from_hex(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;
    
    for key in 0x01u8..=0xffu8 {
        let key_bytes = Bytes::from(&[key][..]);
        let result = Bytes::xor(&input, &key_bytes);
        let content = result.to_string(encoding_rs::WINDOWS_1252).0;

        if content.words() > 1 {
            let alnum = content.alnum_frequency().unwrap();
            if alnum > 0.7 {
                let word_len = content.avg_word_length().unwrap();
                if word_len < 10.0 {
                    let lc_words = content.lowercase_words().unwrap();
                    if lc_words > 0.25 {
                        println!("with key 0x{key:02x} ({alnum}, {word_len}, {lc_words}): {content:?}");
                    }
                }
            }
        }
    }
    Ok(())
}
