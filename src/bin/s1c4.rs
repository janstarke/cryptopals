use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::StringDb;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination. 
fn main() -> Result<()> {
    let file = File::open("data/4.txt")?;
    let reader = BufReader::new(file);
    let mut db = StringDb::default();
    for (lineno, line) in reader.lines().map_while(Result::ok).enumerate() {
        if !line.is_empty() {
            db.add_original(
                Bytes::from_hex(&line)?, 
                lineno,
                (0x01u8..0xffu8).map(Bytes::from));
        }
    }

    let entry = db.iter().next().unwrap();
    println!("key 0x{} reveals: {:?} from line {}", entry.key(), entry.decrypted(), entry.lineno());
    
    Ok(())
}
