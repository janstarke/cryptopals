use encoding_rs::WINDOWS_1252;

use crate::{Bytes, Score};

pub trait FindSingleXorKey {
    fn sort_single_xor_keys(&self, score_fn: impl Score) -> Vec<(u8, f64)>;
}

impl FindSingleXorKey for Bytes {
    fn sort_single_xor_keys(&self, score_fn: impl Score) -> Vec<(u8, f64)> {
        let mut keys = Vec::new();
        for key in 0x01u8..=0xffu8 {
            let key_bytes = Bytes::from(&[key][..]);
            let result = Bytes::xor(self, &key_bytes);

            let score = score_fn.score_bytes(&result, WINDOWS_1252);
            if !score.is_nan() {
                keys.push((key, score));
            }
        }
        keys.sort_by(|e1, e2| e1.1.total_cmp(&e2.1));
        keys
    }
}
