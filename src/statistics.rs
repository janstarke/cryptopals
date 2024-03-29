use std::collections::HashMap;
//use lazy_static::lazy_static;

/*
lazy_static!{
    static ref CHI_SQUARED_QUANTILES: HashMap<u8, u8> = {
        let alphas = vec![0.9, 0.95, 0.975, 0.99, 0.995, 0.999];
        vec![

        ]
    };
}
 */

pub fn chi_squared_test(
    expected: &HashMap<char, f64>,
    measured: &HashMap<char, u32>,
    n: u32,
) -> f64 {
    debug_assert!(expected.values().sum::<f64>() <= 1.0);
    debug_assert!(measured.values().sum::<u32>() <= n);
    const SPECIAL_CHARACTERS_PROBABILITY: f64 = 0.01;

    // we have m+1 classes: m different classes with expected characters,
    // and one class for all unexpected characters (with p=0)
    let _m = expected.len() + 1;
    let expected_characters: u32 = expected.keys().map(|ch|measured[ch]).sum();

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
