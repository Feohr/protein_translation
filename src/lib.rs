mod str;
mod string;

use thiserror::Error;
use anyhow::Result;

// Size of codon chunkk
pub const CODON_CHUNK: usize = 3_usize;
// To hold valid nucleotides
pub const NUCLTD: [char; 5_usize] = ['A', 'G', 'C', 'T', 'U'];

/// Trait is handled for both `&str` and `String` for ease of use. Could've used generics but it
/// would be really tough to maintain and hard to manage. Moreover it can reduce code performance
/// as well.
pub trait ProteinTranslate {
    type Item;
    fn codon(&self) -> Result<Vec<Self::Item>>;
}

#[derive(Debug, Error)]
pub enum ProteinError {
    #[error("The given nucleotide is {0} invalid")]
    InvalidNucleotide(char),
    #[error("Codon need to have at least 3 nucleotides")]
    InvalidCodonLen(String),
}
