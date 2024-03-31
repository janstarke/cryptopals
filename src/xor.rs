use crate::string_statistics::CharacterFrequency;
use crate::Bytes;

pub trait FindSingleXorKey {
    fn find_single_xor_key(&self) -> Vec<Bytes>;
    fn sort_single_xor_keys(&self, score_fn: fn(&str) -> f64) -> Vec<(u8, f64)>;
}

impl FindSingleXorKey for Bytes {
    fn find_single_xor_key(&self) -> Vec<Bytes> {
        let mut keys = Vec::new();
        for key in 0x01u8..=0xffu8 {
            let key_bytes = Bytes::from(&[key][..]);
            let result = Bytes::xor(self, &key_bytes);
            let content = result.to_string(encoding_rs::WINDOWS_1252).0;

            if content.words() > 1 {
                let alnum = content.alnum_frequency().unwrap();
                if alnum > 0.7 {
                    let word_len = content.avg_word_length().unwrap();
                    if word_len < 10.0 {
                        let lc_words = content.lowercase_words().unwrap();
                        if lc_words > 0.25 {
                            let key_bytes = Bytes::from(&[key][..]);
                            //println!("with key 0x{key:02x} ({alnum}, {word_len}, {lc_words}): {content:?}");
                            keys.push(key_bytes);
                        }
                    }
                }
            }
        }
        keys
    }

    fn sort_single_xor_keys(&self, score_fn: fn(&str) -> f64) -> Vec<(u8, f64)> {
        let mut keys = Vec::new();
        for key in 0x01u8..=0xffu8 {
            let key_bytes = Bytes::from(&[key][..]);
            let result = Bytes::xor(self, &key_bytes);
            let content = result.to_string(encoding_rs::WINDOWS_1252).0;

            let score = score_fn(&content);
            //let score = content.simple_english_score();
            if !score.is_nan() {
                keys.push((key, score));
            }
        }
        keys.sort_by(|e1, e2| e1.1.total_cmp(&e2.1));
        keys
    }
}
