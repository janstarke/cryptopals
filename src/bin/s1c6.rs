use std::fs::File;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::ENGLISH;
use cryptopals::CharacterFrequency;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination.
fn main() -> Result<()> {
    find_with(|s| s.simple_english_score(), "simple")?;
    find_with(|s| s.language_score(&ENGLISH), "Chi²")?;
    Ok(())
}

fn find_with(_score_fn: fn(&str) -> f64, _caption: &str) -> Result<()> {
    let _input = Bytes::from_base64_stream(File::open("data/6.txt")?)?;


    Ok(())
}

struct Candidate {
    score: f64,
    key: Bytes,
    lineno: usize,
    decrypted: String,
}
