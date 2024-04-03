use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use cryptopals::Bytes;

fn main() -> Result<()> {
    let file = File::open("data/8.txt")?;
    let reader = BufReader::new(file);

    for (lineno, line) in reader.lines().map_while(Result::ok).enumerate() {
        let input = Bytes::from_base64(&line)?;
        let chunks = input.chunkify(128 / 8);
        let chunks_count = chunks.len();
        let unique_chunks: HashSet<Vec<u8>> = chunks.into_iter().collect();
        if unique_chunks.len() < chunks_count {
            println!("found duplicate block in line {lineno}");
            break;
        }
    }

    Ok(())
}
