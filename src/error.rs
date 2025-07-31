#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duplicate parent sequence found: {0}")]
    DuplicateParent(String),
    #[error("Invalid nucleotide sequence: {0}")]
    InvalidSequence(String),
    #[error("Empty sequence")]
    EmptySequence,
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}
