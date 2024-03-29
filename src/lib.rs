//! # Protein Translation
//!
//! A simple library to parse `&str` and `String` types to create a `Vec<String>` with protein
//! names.
//!
//! ## Example:
//!
//! ```rust
//! use protein_translation::*;
//!
//! # fn main() {
//! let rna = "AUGUUUUCUUAAAUG".to_string();
//! let protein_vec = rna.protein_translate().unwrap();
//! assert_eq!(
//!     vec!["Methionine", "Phenylalanine", "Serine"],
//!     protein_vec,
//! );
//! # }
//! ```

pub mod str;
pub mod string;

use anyhow::Result;
use thiserror::Error;

// Size of codon chunk.
pub(crate) const CODON_CHUNK: usize = 3_usize;
// To hold valid nucleotides.
pub(crate) const NUCLTD: [char; 5_usize] = ['A', 'G', 'C', 'T', 'U'];

/// Trait is handled for both `&str` and `String` for ease of use. Instead, the function can take
/// both `&str` and `String` to return a `Vec<&str>`.
pub trait ProteinTranslate<'a>
where
    Self: Sized + From<&'a str> + ToString + AsRef<str>,
{
    /// Function that takes a `&str` or `String` and returns a `Vec<&str>` with the appropriate
    /// protein names.
    fn protein_translate(&'a self) -> Result<Vec<&'a str>> {
        // Vector to hold the resulting proteins.
        let mut protein_vec = Vec::<&'a str>::new();
        // To get the codon vector from a &str.
        let mut codon_iter = self.codon()?.into_iter();
        // Matching and entering the specific protein.
        while let Some(ref codon) = codon_iter.next() {
            match codon.to_string().to_uppercase().as_ref() {
                "AUG" => protein_vec.push("Methionine"),
                "UUU" | "UUC" => protein_vec.push("Phenylalanine"),
                "UUA" | "UUG" => protein_vec.push("Leucine"),
                "UCU" | "UCC" | "UCA" | "UCG" => protein_vec.push("Serine"),
                "UAU" | "UAC" => protein_vec.push("Tyrosine"),
                "UGU" | "UGC" => protein_vec.push("Cysteine"),
                "UGG" => protein_vec.push("Tryptophan"),
                "UAA" | "UAG" | "UGA" => break,
                _ => return Err(ProteinError::InvalidCodon(codon.to_string()).into()),
            }
        }
        Ok(protein_vec)
    }

    /// Program to take a stream of nucleotides and return `Vec<&str>` with valid codon length
    /// and nucleotides.
    fn codon(&'a self) -> Result<Vec<&'a str>> {
        let mut codon_vec = Vec::<&'a str>::new();
        // Iterating through chunks of codons, validating and pushing.
        for chunk in self.as_ref().as_bytes().chunks(CODON_CHUNK) {
            // Converting the given chunk to utf8. Techniacally you won't be getting this error as
            // you are converting the chunks from a &str in the first place which is made up of
            // valid utf8 characters. Still it is handled for reassurance.
            let codon = std::str::from_utf8(chunk)?;
            // If codon is not in a pair of 3.
            if codon.len() < CODON_CHUNK {
                return Err(ProteinError::InvalidCodonLen(codon.to_string()).into());
            }
            // Checking if the given codon chunk is valid.
            for nucleotide in codon.to_uppercase().chars() {
                if !NUCLTD.contains(&nucleotide) {
                    return Err(ProteinError::InvalidNucleotide(nucleotide).into());
                }
            }
            // Push if all satisfies.
            codon_vec.push(&codon[..]);
        }
        Ok(codon_vec)
    }
}

/// Error type.
#[derive(Debug, Error)]
pub enum ProteinError {
    /// If the nucleotide type is invalid.
    #[error("The given nucleotide is {0} invalid")]
    InvalidNucleotide(char),
    /// If Codon is not exactly character length.
    #[error("Codon need to have at least 3 nucleotides. {0}")]
    InvalidCodonLen(String),
    /// If the codon is not valid.
    #[error("Invalid Codon type {0}")]
    InvalidCodon(String),
}
