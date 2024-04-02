use getset::Getters;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct LanguageProperties {
    mapping: HashMap<char, f64>,
    maximum_score: f64,
}

impl LanguageProperties {
    pub fn new(mapping: HashMap<char, f64>) -> Self {
        let maximum_score = mapping.values().sum();
        Self {
            mapping,
            maximum_score,
        }
    }

    pub fn space_score(&self) -> f64 {
        self.mapping()[&'e'] * 1.0
    }
}

lazy_static! {
    pub static ref ENGLISH: LanguageProperties = LanguageProperties::new(
        vec![
            ('a', 8.2),
            ('b', 1.5),
            ('c', 1.8),
            ('d', 4.3),
            ('e', 12.7),
            ('f', 2.2),
            ('g', 2.0),
            ('h', 6.1),
            ('i', 7.0),
            ('j', 0.15),
            ('k', 0.77),
            ('l', 4.0),
            ('m', 1.4),
            ('n', 6.7),
            ('o', 7.5),
            ('p', 1.9),
            ('q', 0.095),
            ('r', 6.0),
            ('s', 6.3),
            ('t', 9.1),
            ('u', 2.8),
            ('v', 0.98),
            ('w', 2.4),
            ('x', 0.15),
            ('y', 2.0),
            ('z', 0.074),
        ]
        .into_iter()
        .map(|(ch, sc)| (ch, sc / 100.0))
        .collect()
    );
}
