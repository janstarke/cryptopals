use std::collections::HashSet;

use anyhow::Result;
use cryptopals::{encryption_oracle, Bytes};

fn main() -> Result<()> {
    let data = Bytes::from_ascii("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

    for _ in 0..10 {
        let (encrypted, is_using_cbc_hint) = encryption_oracle(&data)?;

        if is_using_cbc(&encrypted) == is_using_cbc_hint {
            println!("correct");
        } else {
            println!("wrong");
        }
    }
    Ok(())
}

fn is_using_cbc(encrypted: &Bytes) -> bool {
    let chunks = encrypted.chunkify(128 / 8);
    let chunks_count = chunks.len();
    let unique_chunks: HashSet<Vec<u8>> = chunks.into_iter().collect();

    unique_chunks.len() >= chunks_count
}
