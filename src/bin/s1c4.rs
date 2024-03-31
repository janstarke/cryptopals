use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::ENGLISH;
use cryptopals::FindSingleXorKey;
use encoding_rs::WINDOWS_1252;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination. 
fn main() -> Result<()> {
    let file = File::open("data/4.txt")?;
    let reader = BufReader::new(file);
    
    let mut candidate = Candidate {
        score: f64::MAX,
        key: Bytes::from(vec![]),
        lineno: 0,
        decrypted: String::new()
    };

    for (lineno, line) in reader.lines().map_while(Result::ok).enumerate() {
        if !line.is_empty() {
            let raw = Bytes::from_hex(&line)?;
            let (key, score) = raw.sort_single_xor_keys(&ENGLISH).into_iter().next().unwrap();
            if score < candidate.score {
                let decrypted = (&key ^ &raw).to_string(WINDOWS_1252).0.to_string();
                candidate = Candidate {
                    score,
                    key,
                    lineno,
                    decrypted,
                };
            }
        }
    }

    println!("key 0x{} reveals: {:?} from line {}", candidate.key, candidate.decrypted, candidate.lineno);
    
    Ok(())
}

struct Candidate {
    score: f64,
    key: Bytes,
    lineno: usize,
    decrypted: String
}