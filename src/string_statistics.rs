pub trait CharacterFrequency {
    fn character_frequency_abs(self, ch: char) -> usize;
    fn character_frequency_rel(self, ch: char) -> Option<f64>;

    fn alnum_frequency(self) -> Option<f64>;
    fn words(self) -> usize;
    fn avg_word_length(self) -> Option<f64>;
    fn lowercase_words(self) -> Option<f64>;
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
}
