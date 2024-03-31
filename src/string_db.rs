use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use encoding_rs::WINDOWS_1252;
use getset::Getters;

use crate::string_statistics::CharacterFrequency;
use crate::{Bytes, ENGLISH};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct StringDbEntry {
    original: Rc<Bytes>,
    key: Bytes,
    decrypted: String,
    score: f64,
}

impl StringDbEntry {
    pub fn from(original: Rc<Bytes>, key: Bytes) -> Self {
        let decrypted = (original.as_ref() ^ &key)
            .to_string(WINDOWS_1252)
            .0
            .to_string();
        let score = decrypted.to_lowercase().language_score(&ENGLISH);
        Self {
            original,
            key,
            decrypted,
            score,
        }
    }
}

impl Ord for StringDbEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::total_cmp(self.score(), other.score())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for StringDbEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.original == other.original {
            Some(self.cmp(other))
        } else {
            None
        }
    }
}

impl Eq for StringDbEntry {}

impl PartialEq for StringDbEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.original == other.original {
            self.eq(other)
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct StringDb {
    originals: HashSet<Rc<Bytes>>,
    decrypted_entries: BTreeSet<StringDbEntry>,
}

impl StringDb {
    pub fn add(&mut self, original: Bytes, key: Bytes) {
        let original = Rc::new(original);
        self.decrypted_entries
            .insert(StringDbEntry::from(Rc::clone(&original), key));
        self.originals.insert(original);
    }

    pub fn add_original(&mut self, original: Bytes, keys: impl Iterator<Item = Bytes>) {
        let original = Rc::new(original);
        self.decrypted_entries
            .extend(keys.map(|key| StringDbEntry::from(Rc::clone(&original), key)));
        self.originals.insert(original);
    }

    pub fn iter(&self) -> impl Iterator<Item = &StringDbEntry> {
        self.decrypted_entries.iter()
    }
}
