#[cfg(test)]
mod codon_tests {
    use protein_translation::*;

    #[test]
    fn call_codon() {
        let rna = "AUGUUUUCUUAAAUG";
        assert!(rna.codon().is_ok());
        assert!(rna.to_string().codon().is_ok());
    }

    #[test]
    fn invalid_codon_err() {
        let rna = "UUUAUGUUE";
        assert!(rna.codon().is_err());
    }

    #[test]
    fn valid_codon() {
        let rna = "AUGUUUUCUUAAAUG";
        assert_eq!(vec!["AUG", "UUU", "UCU", "UAA", "AUG"], rna.codon().unwrap());
    }

    #[test]
    fn invalid_nucleotide() {
        let rna = "AUGUUUUIUUAOAUG";
        assert!(rna.codon().is_err());
    }

    #[test]
    fn valid_nucleotide() {
        let rna = "AUGUGUUCUUAAAUT";
        assert!(rna.codon().is_ok());
    }

    #[test]
    fn fail_nucleotide_uppercase() {
        let rna = "AUGuGUUcUUAaAUT";
        assert!(rna.codon().is_ok())
    }
}
