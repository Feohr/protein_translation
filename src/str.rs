use crate::{ProteinError, ProteinTranslate, NUCLTD, CODON_CHUNK};
use anyhow::Result;

impl<'a> ProteinTranslate for &'a str {
    type Item = &'a str;
    fn protein_translate(self) -> Result<Vec<Self::Item>> {
        // Vector to hold the resulting proteins.
        let mut protein_vec = Vec::<Self::Item>::new();
        // To get the codon vector from a &str.
        let mut codon_iter = codon(self)?.into_iter();

        // Matching and entering the specific protein.
        while let Some(ref codon) = codon_iter.next() {
            match codon.to_uppercase().as_ref() {
                "AUG" => protein_vec.push("Methionine"),
                "UUU" | "UUC" => protein_vec.push("Phenylalanine"),
                "UUA" | "UUG" => protein_vec.push("Leucine"),
                "UCU" | "UCC" | "UCA" | "UCG" => protein_vec.push("Serine"),
                "UAU" | "UAC" => protein_vec.push("Tyrosine"),
                "UGU" | "UGC" => protein_vec.push("Cysteine"),
                "UGG"  => protein_vec.push("Tryptophan"),
                "UAA" | "UAG" | "UGA" => break,
                _ => continue,
            }
        }

        Ok(protein_vec)
    }
}

/// Program to take a stream of nucleotides and return vector of `&str` with valid codon length
/// and nucleotides. The input needs to be `&str` to avoid issues with `String` conversions
/// that is likely to happen with generics.
pub fn codon(rna: &str) -> Result<Vec<&str>> {
    let mut codon_vec = Vec::<&str>::new();
    // Iterating through chunks of codons, validating and pushing.
    for chunk in rna.as_bytes().chunks(CODON_CHUNK) {
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

#[cfg(test)]
mod unit_str_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_codon_err() {
        let rna = "UUUAUGUUE";
        codon(rna).unwrap();
    }

    #[test]
    fn valid_codon() {
        let rna = "AUGUUUUCUUAAAUG";
        assert_eq!(vec!["AUG", "UUU", "UCU", "UAA", "AUG"], codon(rna).unwrap());
    }

    #[test]
    #[should_panic]
    fn invalid_nucleotide() {
        let rna = "AUGUUUUIUUAOAUG";
        codon(rna).unwrap();
    }

    #[test]
    fn valid_nucleotide() {
        let rna = "AUGUGUUCUUAAAUT";
        codon(rna).unwrap();
    }

    #[test]
    fn nucleotide_uppercase() {
        let rna = "AUGuGUUcUUAaAUT";
        codon(rna).unwrap();
    }
}
