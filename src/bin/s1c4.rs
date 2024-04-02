use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::ChiSquaredScoring;
use cryptopals::FindSingleXorKey;
use cryptopals::Score;
use cryptopals::SimpleScoring;
use cryptopals::ENGLISH;
use encoding_rs::WINDOWS_1252;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination.
fn main() -> Result<()> {
    find_with(SimpleScoring, "simple")?;
    find_with(ChiSquaredScoring::r#for(&ENGLISH), "Chi²")?;
    Ok(())
}

fn find_with(score_fn: impl Score, caption: &str) -> Result<()> {
    let file = File::open("data/4.txt")?;
    let reader = BufReader::new(file);

    let mut candidate = Candidate {
        score: f64::MAX,
        key: 0,
        lineno: 0,
        decrypted: String::new(),
    };

    for (lineno, line) in reader.lines().map_while(Result::ok).enumerate() {
        if !line.is_empty() {
            let raw = Bytes::from_hex(&line)?;
            let (key, score) = raw
                .sort_single_xor_keys(score_fn)
                .into_iter()
                .next()
                .unwrap();
            if score < candidate.score {
                let decrypted = (&key.into() ^ &raw).to_string(WINDOWS_1252).0.to_string();
                candidate = Candidate {
                    score,
                    key,
                    lineno,
                    decrypted,
                };
            }
        }
    }

    println!(
        "key 0x{} reveals: {:?} with score {:4.2} from line {} using {caption} method",
        candidate.key, candidate.decrypted, candidate.score, candidate.lineno
    );

    Ok(())
}

struct Candidate {
    score: f64,
    key: u8,
    lineno: usize,
    decrypted: String,
}
