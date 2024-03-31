use getset::Getters;
use lazy_static::lazy_static;
use levenshtein::levenshtein;
use std::collections::HashMap;

use crate::chi_squared_test;

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

pub trait CharacterFrequency {
    fn character_frequency_abs(self, ch: char) -> usize;
    fn character_frequency_rel(self, ch: char) -> Option<f64>;

    fn alnum_frequency(self) -> Option<f64>;
    fn words(self) -> usize;
    fn avg_word_length(self) -> Option<f64>;
    fn lowercase_words(self) -> Option<f64>;

    /// scores the given text. zero means maximum similarity, a higher valuse
    /// means less similarity
    fn language_score(self, language: &LanguageProperties) -> f64;

    fn simple_english_score(self) -> f64;
}

impl CharacterFrequency for &str {
    fn character_frequency_abs(self, ch: char) -> usize {
        self.chars().filter(|c| *c == ch).count()
    }

    /// returns the relative frequency of a specific character
    fn character_frequency_rel(self, ch: char) -> Option<f64> {
        match self.chars().count().try_into().unwrap() {
            0 => None,
            count => {
                let filtered_count: u32 = self.character_frequency_abs(ch).try_into().unwrap();
                Some(f64::from(filtered_count) / f64::from(count))
            }
        }
    }

    /// returns the frequency of alphanumeric characters (only ASCII characters)
    fn alnum_frequency(self) -> Option<f64> {
        match self.chars().count().try_into().unwrap() {
            0 => None,
            count => {
                let filtered_count: u32 = self
                    .chars()
                    .filter(|ch| ch.is_ascii_alphanumeric())
                    .count()
                    .try_into()
                    .unwrap();
                Some(f64::from(filtered_count) / f64::from(count))
            }
        }
    }

    /// returns the number of words
    fn words(self) -> usize {
        self.split_ascii_whitespace().count()
    }

    /// returns the average word length
    fn avg_word_length(self) -> Option<f64> {
        let words = self.split_ascii_whitespace();
        match words.clone().count().try_into().unwrap() {
            0 => None,
            word_count => {
                let length: u32 = words.map(|w| w.len()).sum::<usize>().try_into().unwrap();
                Some(f64::from(length) / f64::from(word_count))
            }
        }
    }

    /// returns frequency of words which consist only of lowercase characters
    /// (ASCII only), in relation to the number of words
    ///
    /// # Example
    /// ```rust
    /// use cryptopals::CharacterFrequency;
    ///
    /// assert_eq!("hello world".lowercase_words().unwrap(), 1.0);
    /// assert_eq!("hello, world".lowercase_words().unwrap(), 1.0);
    /// assert_eq!("hello World".lowercase_words().unwrap(), 0.5);
    /// assert_eq!("hello, wOrld".lowercase_words().unwrap(), 0.5);
    /// ```
    fn lowercase_words(self) -> Option<f64> {
        let words = self.split_ascii_whitespace();
        match words.clone().count().try_into().unwrap() {
            0 => None,
            word_count => {
                let lowercase_words: u32 = words
                    .filter(|w| *w == w.to_lowercase())
                    .count()
                    .try_into()
                    .unwrap();
                Some(f64::from(lowercase_words) / f64::from(word_count))
            }
        }
    }

    fn simple_english_score(self) -> f64 {
        let pattern = " ETAOINSHRDLU";
        let language_scores: HashMap<_, _> = pattern
            .to_lowercase()
            .char_indices()
            .map(|(idx, ch)| (ch, pattern.len() - idx))
            .collect();

        let maximum_achievable = f64::from(u32::try_from(self.len() * pattern.len()).unwrap());
        let score = self
            .to_lowercase()
            .chars()
            .map(|ch| *language_scores.get(&ch).unwrap_or(&0))
            .sum::<usize>();

        (1.0 - f64::from(u32::try_from(score).unwrap())/ maximum_achievable)
            * 100.0
    }

    fn language_score(self, language: &LanguageProperties) -> f64 {
        match self.len() {
            0 => 0.0,
            length => {
                let characters: Vec<_> = language.mapping().keys().copied().collect();
                let mut frequencies: HashMap<char, u32> =
                    characters.iter().map(|ch| (*ch, 0)).collect();

                for ch in self.chars() {
                    *frequencies.entry(ch).or_default() += 1;
                }

                chi_squared_test(
                    language.mapping(),
                    &frequencies,
                    u32::try_from(length).unwrap(),
                )
            }
        }
    }
}
