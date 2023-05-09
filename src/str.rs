use crate::{ProteinError, ProteinTranslate, NUCLTD, CODON_CHUNK};
use anyhow::Result;

impl<'a> ProteinTranslate for &'a str {
    type Item = &'a str;
    /// Program to take a stream of nucleotides and return vector of `&str` with valid codon length
    /// and nucleotides. The input needs to be `&str` to avoid issues with `String` conversions
    /// that is likely to happen with generics.
    fn codon(&self) -> Result<Vec<Self::Item>> {
        let mut codon_vec = Vec::<Self::Item>::new();
        // Iterating through chunks of codons, validating and pushing.
        for chunk in self.as_bytes().chunks(CODON_CHUNK) {
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
            codon_vec.push(codon);
        }
        Ok(codon_vec)
    }
}
