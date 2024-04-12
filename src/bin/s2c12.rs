use std::{
    collections::{HashMap, HashSet},
};

use anyhow::{bail, Result};
use cryptopals::{encryption_oracle_c12, Bytes};
use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    let mut block_size = None;
    let mut plaintext_length = 0;
    for length in 0..20 {
        let data = Bytes::from_ascii(&"A".repeat(length));
        let encrypted = encryption_oracle_c12(&data)?;

        match block_size {
            None => block_size = Some(encrypted.len()),
            Some(bs) => {
                let new_block_size = encrypted.len();
                if new_block_size > bs {

                    // we have prefixed the plaintext with enough bytes to force
                    // the oracle to create a new block. So we know, the plaintext
                    // length equals the length of the
                    // ciphertext - blocksize - prefix length
                    //
                    // we must subtract blocksize, because if the plaintext length
                    // is a multiple of blocksize, a padding block is appended
                    block_size = Some(new_block_size - bs);
                    plaintext_length = encrypted.len() - length - (new_block_size - bs);
                    break;
                }
            }
        }
    }
    let block_size = block_size.unwrap();

    // Discover the block size of the cipher
    assert_eq!(block_size, 16);

    // Detect that the function is using ECB
    assert!(!is_using_cbc(&encryption_oracle_c12(&Bytes::from_ascii(
        &"A".repeat(2 * block_size)
    ))?));

    let mut known_bytes = Bytes::from(vec![]);
    for _ in 0..plaintext_length {
        if let Some(b) = grab_next_byte(block_size, &known_bytes, plaintext_length)? {
            //print!("{}", b as char);
            known_bytes.append(b)
        } else {
            bail!("found no more values ({} missing), found so far: {}",
            plaintext_length - known_bytes.len(),
            known_bytes.to_string(WINDOWS_1252).0);
            //break;
        }
    }

    println!("{}", known_bytes.to_string(WINDOWS_1252).0);

    Ok(())
}

fn grab_next_byte(block_size: usize, known_bytes: &Bytes, _expected_length: usize) -> Result<Option<u8>> {
    let destined_block = known_bytes.len() / block_size;
    let known_bytes_in_block = known_bytes.len() % block_size;
    //let is_last_block = (expected_length - known_bytes.len()) <= block_size;

    let dtblock_range = destined_block * block_size..destined_block * block_size + block_size;

    let prefix_size = block_size - (known_bytes_in_block + 1);

    // this is something like "AAAAAAA"
    let mut prefix = Bytes::from_ascii(&"A".repeat(prefix_size));
    let mut encrypted = encryption_oracle_c12(&prefix)?;
    encrypted = Bytes::from(&encrypted[dtblock_range.clone()]);

    prefix = &prefix + known_bytes;
    assert_eq!(prefix.len(), (destined_block + 1) * block_size - 1);

    let blocks: HashMap<Bytes, u8> = (0x00..0xff)
        .map(|b| {
            let result = encryption_oracle_c12(&(&prefix + &Bytes::from(vec![b]))).unwrap();
            (Bytes::from(&result[dtblock_range.clone()]), b)
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
