use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("the input string has an invalid number of characters")]
    IllegalNumberOfCharacters,

    #[error("found an illegal character: {0}")]
    IllegalCharacter(char)
}