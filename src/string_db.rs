use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

use encoding_rs::WINDOWS_1252;
use getset::Getters;

use crate::string_statistics::CharacterFrequency;
use crate::Bytes;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct StringDbEntry {
    original: Rc<Bytes>,
    lineno: usize,
    key: Bytes,
    decrypted: String,
    score: f64,
}

impl StringDbEntry {
    pub fn from(original: Rc<Bytes>, lineno: usize, key: Bytes) -> Self {
        let decrypted = (original.as_ref() ^ &key)
            .to_string(WINDOWS_1252)
            .0
            .to_string();
        let score = decrypted.to_lowercase().simple_english_score();
        Self {
            original,
            lineno,
            key,
            decrypted,
            score,
        }
    }
}

impl Ord for StringDbEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score == other.score {
            self.original.as_ref().cmp(other.original.as_ref())
        } else {
            f64::total_cmp(self.score(), other.score())
        }
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for StringDbEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score == other.score {
            Some(self.original.as_ref().cmp(other.original.as_ref()))
        } else {
            Some(self.cmp(other))
        }
    }
}

impl Eq for StringDbEntry {}

impl PartialEq for StringDbEntry {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original && self.key == other.key
    }
}

#[derive(Default)]
pub struct StringDb {
    originals: HashSet<Rc<Bytes>>,
    decrypted_entries: BTreeSet<StringDbEntry>,
}

impl StringDb {
    pub fn add_original(
        &mut self,
        original: Bytes,
        lineno: usize,
        keys: impl Iterator<Item = Bytes>,
    ) {
        let original = Rc::new(original);
        self.decrypted_entries
            .extend(keys.map(|key| StringDbEntry::from(Rc::clone(&original), lineno, key)));
        assert!(
            self.originals.insert(original),
            "db already contained text from line {lineno}"
        );
    }

    pub fn iter(&self) -> impl Iterator<Item = &StringDbEntry> {
        self.decrypted_entries.iter()
    }
}
