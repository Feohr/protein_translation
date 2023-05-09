use crate::ProteinTranslate;

impl<'a> ProteinTranslate<'a> for String {}

#[cfg(test)]
mod unit_string_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_codon_err() {
        let rna = "UUUAUGUUE".to_string();
        rna.codon().unwrap();
    }

    #[test]
    fn valid_codon() {
        let rna = "AUGUUUUCUUAAAUG".to_string();
        assert_eq!(vec![
            "AUG".to_string(),
            "UUU".to_string(),
            "UCU".to_string(),
            "UAA".to_string(),
            "AUG".to_string(),
        ], rna.codon().unwrap());
    }

    #[test]
    #[should_panic]
    fn invalid_nucleotide() {
        let rna = "AUGUUUUIUUAOAUG".to_string();
        rna.codon().unwrap();
    }

    #[test]
    fn valid_nucleotide() {
        let rna = "AUGUGUUCUUAAAUT".to_string();
        rna.codon().unwrap();
    }

    #[test]
    fn nucleotide_uppercase() {
        let rna = "AUGuGUUcUUAaAUT".to_string();
        rna.codon().unwrap();
    }
}
