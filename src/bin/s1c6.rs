use std::fs::File;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::FindSingleXorKey;
use cryptopals::Score;
use cryptopals::SimpleScoring;
use cryptopals::Transpose;
use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    find_with(SimpleScoring, "simple")?;
    Ok(())
}

fn find_with(score_fn: impl Score, _caption: &str) -> Result<()> {
    let input = Bytes::from_base64_stream(File::open("data/6.txt")?)?;

    let keysize = guess_key_sizes(&input).next().unwrap();

    let key = brute_with_keysize(score_fn, keysize, &input);
    assert_eq!(key.len(), keysize);
    let decrypted = (&input ^ &key).to_string(WINDOWS_1252).0.to_string();
    println!(
        "found key {key:?}: {decrypted:.20} ...",
        key = key.to_string(WINDOWS_1252).0
    );

    Ok(())
}

fn guess_key_sizes(input: &Bytes) -> impl Iterator<Item = usize> {
    let mut key_sizes = Vec::new();
    for keysize in 2..40 {
        let blocks = input.chunkify(keysize);
        let mut distance = 0.0;
        let mut count: u32 = 0;
        'outer: for idx1 in 0..blocks.len() {
            for idx2 in 0..idx1 {
                let block1 = Bytes::from(&blocks[idx1][..]);
                let block2 = Bytes::from(&blocks[idx2][..]);
                distance += f64::from(block1 - block2);
                count += 1;

                // performance: this is sufficient
                if count > 10 {
                    break 'outer;
                }
            }
        }

        let mut distance = (distance / f64::from(count)).round();

        // normalize
        distance /= f64::from(u32::try_from(keysize).unwrap());
        //println!("keysize {keysize} has distance {distance}");

        key_sizes.push((keysize, distance));
    }
    key_sizes.sort_by(|lhs, rhs| lhs.1.total_cmp(&rhs.1));
    key_sizes.into_iter().map(|x| x.0)
}

fn brute_with_keysize(score_fn: impl Score, keysize: usize, input: &Bytes) -> Bytes {
    let blocks = input.chunkify(keysize).transpose();
    assert_eq!(blocks.len(), keysize);
    let mut key = Vec::new();
    for block in blocks.into_iter().map(Bytes::from) {
        let (key_part, _) = block
            .sort_single_xor_keys(score_fn)
            .into_iter()
            .next()
            .unwrap();
        key.push(key_part);
    }
    key.into()
}
