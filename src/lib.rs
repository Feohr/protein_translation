pub mod str;
pub mod string;

use thiserror::Error;
use anyhow::Result;

// Size of codon chunk.
pub(crate) const CODON_CHUNK: usize = 3_usize;
// To hold valid nucleotides.
pub(crate) const NUCLTD: [char; 5_usize] = ['A', 'G', 'C', 'T', 'U'];

/// Trait is handled for both `&str` and `String` for ease of use. Could've used generics but it
/// would be really tough to maintain and hard to manage. Moreover it can reduce code performance
/// as well. Instead, the function can take both '&str' and `String` to return a `Vec<String>`.
pub trait ProteinTranslate<'a> where Self: Sized + From<&'a str> + ToString + AsRef<str> {
    /// Function that takes a `&str` or `String` and returns a `Vec<String>` with the appropriate
    /// protein names.
    fn protein_translate(self) -> Result<Vec<String>> {
        // Vector to hold the resulting proteins.
        let mut protein_vec = Vec::<String>::new();
        // To get the codon vector from a &str.
        let mut codon_iter = self.codon()?.into_iter();
        // Matching and entering the specific protein.
        while let Some(ref codon) = codon_iter.next() {
            match codon.to_string().to_uppercase().as_ref() {
                "AUG" => protein_vec.push("Methionine".into()),
                "UUU" | "UUC" => protein_vec.push("Phenylalanine".into()),
                "UUA" | "UUG" => protein_vec.push("Leucine".into()),
                "UCU" | "UCC" | "UCA" | "UCG" => protein_vec.push("Serine".into()),
                "UAU" | "UAC" => protein_vec.push("Tyrosine".into()),
                "UGU" | "UGC" => protein_vec.push("Cysteine".into()),
                "UGG"  => protein_vec.push("Tryptophan".into()),
                "UAA" | "UAG" | "UGA" => break,
                _ => return Err(ProteinError::InvalidCodon(codon.into()).into()),
            }
        }
        Ok(protein_vec)
    }

    /// Program to take a stream of nucleotides and return vector of `string` with valid codon
    /// length and nucleotides.
    fn codon(self) -> Result<Vec<String>> {
        let mut codon_vec = Vec::<String>::new();
        // Iterating through chunks of codons, validating and pushing.
        for chunk in self.as_ref().as_bytes().chunks(CODON_CHUNK) {
            // Converting the given chunk to utf8. Techniacally you won't be getting this error as
            // you are converting the chunks from a &str in the first place which is made up of
            // valid utf8 characters. Still it is handled for reassurance.
            let codon = String::from_utf8(chunk.to_vec())?;
            // If codon is not in a pair of 3.
            if codon.len() < CODON_CHUNK {
                return Err(ProteinError::InvalidCodonLen(codon).into());
            }
            // Checking if the given codon chunk is valid.
            for nucleotide in codon.to_uppercase().chars() {
                if !NUCLTD.contains(&nucleotide) {
                    return Err(ProteinError::InvalidNucleotide(nucleotide).into());
                }
            }
            // Push if all satisfies.
            codon_vec.push(codon.into());
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
