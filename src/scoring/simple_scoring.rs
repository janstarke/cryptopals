use std::collections::HashMap;

use crate::Score;

#[derive(Default, Clone, Copy)]
pub struct SimpleScoring;

impl Score for SimpleScoring {
    fn score_string(&self, value: &str) -> f64 {
        let pattern = " ETAOINSHRDLU";
        let language_scores: HashMap<_, _> = pattern
            .to_lowercase()
            .char_indices()
            .map(|(idx, ch)| (ch, pattern.len() - idx))
            .collect();

        let maximum_achievable = f64::from(u32::try_from(value.len() * pattern.len()).unwrap());
        let score = value
            .to_lowercase()
            .chars()
            .map(|ch| *language_scores.get(&ch).unwrap_or(&0))
            .sum::<usize>();

        (1.0 - f64::from(u32::try_from(score).unwrap()) / maximum_achievable) * 100.0
    }
}
