mod codon;

pub use codon::ProteinTranslate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProteinError {
    #[error("The given nucleotide is {0} invalid")]
    InvalidNucleotide(char),
    #[error("Codon need to have at least 3 nucleotides")]
    InvalidCodonLen(String),
}
