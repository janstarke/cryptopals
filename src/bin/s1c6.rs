use std::fs::File;

use anyhow::Result;
use cryptopals::Bytes;
use cryptopals::CharacterFrequency;
use cryptopals::FindSingleXorKey;
use cryptopals::ENGLISH;
use encoding_rs::WINDOWS_1252;

/// Single-byte XOR cipher
/// Write a function that takes two equal-length buffers and produces their XOR
/// combination.
fn main() -> Result<()> {
    find_with(|s| s.simple_english_score(), "simple")?;
    Ok(())
}

fn find_with(_score_fn: fn(&str) -> f64, _caption: &str) -> Result<()> {
    let input = Bytes::from_base64_stream(File::open("data/6.txt")?)?;

    let mut key_sizes = Vec::new();
    for keysize in 2..40 {
        let blocks = input.chunkify(keysize);
        let mut distance = 0.0;
        for idx in 0..blocks.len() / 2 {
            let block1 = Bytes::from(&blocks[idx][..]);
            let block2 = Bytes::from(&blocks[idx + 1][..]);
            distance += f64::from(block1 - block2);
        }

        let mut distance = (distance / f64::from(u32::try_from(blocks.len() / 2).unwrap())).round();

        // normalize
        distance /= f64::from(u32::try_from(keysize).unwrap());

        key_sizes.push((keysize, distance));
    }
    key_sizes.sort_by(|lhs, rhs| lhs.1.total_cmp(&rhs.1));

    for keysize in key_sizes.into_iter().map(|x| x.0) {
        //println!("trying keysize {keysize}");
        let key = brute_with_keysize(keysize, &input);
        assert_eq!(key.len(), keysize);
        let decrypted = (&input ^ &key).to_string(WINDOWS_1252).0.to_string();
        println!("found key {key}: {decrypted:?}");
    }

    Ok(())
}

fn brute_with_keysize(keysize: usize, input: &Bytes) -> Bytes {
    let blocks = transpose2(input.chunkify(keysize));
    assert_eq!(blocks.len(), keysize);
    let mut key = Vec::new();
    for block in blocks.into_iter().map(Bytes::from) {
        let (key_part, _) = block
            .sort_single_xor_keys(|s| s.simple_english_score())
            .into_iter()
            .next()
            .unwrap();
        key.push(key_part);
    }
    key.into()
}

/// transpose a matrix
/// <https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust>
///
/// ```rust
/// let v = vec![vec![1,2,3,4], vec![5,6,7,8]];
/// let v2 = transpose2(v);
/// assert_eq!(v2, vec![vec![1,5],vec![2,6],vec![3,7],vec![4,8]]);
/// ```
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
