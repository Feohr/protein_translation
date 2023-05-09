# protein_translation

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![crates.io Version](https://img.shields.io/crates/v/protein_translation.svg)](https://crates.io/crates/protein_translation)

A crate to translate `&str` or `String` of `RNA` sequence with nucleotide into a `Vec<String>` of
their appropriate protein names.

## Explaination
A codon is a DNA or RNA sequence of three nucleotides (a trinucleotide) that forms a unit of
genomic information encoding a particular amino acid or signaling the termination of protein
synthesis (stop signals). DNA and the corresponding messenger RNA are made up of a series of bases
(nucleotides). In RNA, these bases are often labeled with the letters A, U, C, and G. A set of
three bases makes up a codon.

## Example

```rust
use protein_translation::*;

    fn main() {
        let rna = "AUGUUUUCUUAAAUG".to_string();
        let protein_vec = rna.protein_translate().unwrap();
        assert_eq!(
        vec![
            "Methionine".to_string(), "Phenylalanine".to_string(), "Serine".to_string()],
            protein_vec,
        );
    }
```
