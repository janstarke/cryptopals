use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use cryptopals::{encryption_oracle_c12, Bytes};
use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    let mut block_size = None;
    for length in 0..20 {
        let data = Bytes::from_ascii(&"A".repeat(length));
        let encrypted = encryption_oracle_c12(&data)?;

        match block_size {
            None => block_size = Some(encrypted.len()),
            Some(bs) => {
                let new_block_size = encrypted.len();
                if new_block_size > bs {
                    block_size = Some(new_block_size - bs);
                    break;
                }
            }
        }
    }
    let block_size = block_size.unwrap();

    println!("{block_size} ({} bit)", block_size * 8);

    // Discover the block size of the cipher
    assert_eq!(block_size, 16);

    // Detect that the function is using ECB
    assert!(!is_using_cbc(&encryption_oracle_c12(&Bytes::from_ascii(
        &"A".repeat(2 * block_size)
    ))?));

    // Knowing the block size, craft an input block that is exactly 1 byte shorter
    let mut encrypted = encryption_oracle_c12(&Bytes::from_ascii(&"A".repeat(block_size - 1)))?;
    encrypted = Bytes::from(&encrypted[0..block_size]);
    assert_eq!(encrypted.len(), block_size);

    // the last position contains the first byte of the plain text
    let base_block = Bytes::from_ascii(&"A".repeat(block_size - 1));
    let blocks: HashMap<Bytes, u8> = (0x00..0xff)
        .map(|b| {
            let result = encryption_oracle_c12(&(&base_block + &Bytes::from(vec![b]))).unwrap();
            (Bytes::from(&result[0..block_size]), b)
        })
        .collect();

    if let Some(first_byte) = blocks.get(&encrypted) {
        let ch = *first_byte as char;
        println!("found first byte to be '{ch}' ({first_byte})");
    }

    let mut known_bytes = Bytes::from(vec![]);
    for _ in 0..block_size {
        println!(
            "decrypted first block: {}",
            known_bytes.to_string(WINDOWS_1252).0
        );

        if let Some(b) = grab_next_byte(block_size, &known_bytes)? {
            known_bytes.append(b)
        } else {
            bail!("found no more values");
        }
    }

    println!(
        "decrypted first block: {}",
        known_bytes.to_string(WINDOWS_1252).0
    );

    Ok(())
}

fn grab_next_byte(block_size: usize, known_bytes: &Bytes) -> Result<Option<u8>> {
    if known_bytes.len() >= block_size {
        bail!("cannot decrypt more than {block_size} bytes");
    }

    let prefix_size = block_size - (known_bytes.len() + 1);

    let mut prefix = Bytes::from_ascii(&"A".repeat(prefix_size));
    let mut encrypted = encryption_oracle_c12(&prefix)?;
    encrypted = Bytes::from(&encrypted[0..block_size]);

    prefix = &prefix + known_bytes;
    assert_eq!(prefix.len(), block_size - 1);

    let blocks: HashMap<Bytes, u8> = (0x00..0xff)
        .map(|b| {
            let result = encryption_oracle_c12(&(&prefix + &Bytes::from(vec![b]))).unwrap();
            (Bytes::from(&result[0..block_size]), b)
        })
        .collect();

    Ok(blocks.get(&encrypted).copied())
}

fn is_using_cbc(encrypted: &Bytes) -> bool {
    let chunks = encrypted.chunkify(128 / 8);
    let chunks_count = chunks.len();
    let unique_chunks: HashSet<Vec<u8>> = chunks.into_iter().collect();

    unique_chunks.len() >= chunks_count
}
