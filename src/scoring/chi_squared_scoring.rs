use std::collections::HashMap;

use crate::{LanguageProperties, Score};

#[derive(Clone, Copy)]
pub struct ChiSquaredScoring {
    language: &'static LanguageProperties,
}

impl ChiSquaredScoring {
    pub fn r#for(language: &'static LanguageProperties) -> Self {
        Self {language}
    }
}

impl Score for ChiSquaredScoring {
    fn score_string(&self, value: &str) -> f64 {
        match value.len() {
            0 => 0.0,
            length => {
                let characters: Vec<_> = self.language.mapping().keys().copied().collect();
                let mut frequencies: HashMap<char, u32> =
                    characters.iter().map(|ch| (*ch, 0)).collect();

                for ch in value.chars() {
                    *frequencies.entry(ch).or_default() += 1;
                }

                chi_squared_test(
                    self.language.mapping(),
                    &frequencies,
                    u32::try_from(length).unwrap(),
                )
            }
        }
    }
}

fn chi_squared_test(expected: &HashMap<char, f64>, measured: &HashMap<char, u32>, n: u32) -> f64 {
    debug_assert!(expected.values().sum::<f64>() <= 1.0);
    debug_assert!(measured.values().sum::<u32>() <= n);
    const SPECIAL_CHARACTERS_PROBABILITY: f64 = 0.01;

    // we have m+1 classes: m different classes with expected characters,
    // and one class for all unexpected characters (with p=0)
    let _m = expected.len() + 1;
    let expected_characters: u32 = expected.keys().map(|ch| measured[ch]).sum();

    let x_squared: f64 = expected
        .iter()
        .map(|(ch, p0j)| {
            let n0j = p0j * f64::from(n);
            (f64::from(measured[ch]) - n0j).powi(2) / n0j
        })
        .sum();

    x_squared + {
        let n0m = SPECIAL_CHARACTERS_PROBABILITY * f64::from(n);
        (f64::from(n - expected_characters) - n0m).powi(2) / n0m
    }
}
