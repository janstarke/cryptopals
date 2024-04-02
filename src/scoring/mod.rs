use encoding_rs::Encoding;

use crate::Bytes;

pub trait Score: Copy {
    fn score_string(&self, value: &str) -> f64;
    fn score_bytes(&self, value: &Bytes, encoding: &'static Encoding) -> f64 {
        let (str_value, _) = value.to_string(encoding);
        self.score_string(&str_value)
    }
}

mod language_properties;
pub use language_properties::*;

mod simple_scoring;
pub use simple_scoring::*;

mod chi_squared_scoring;
pub use chi_squared_scoring::*;