use thiserror::Error;

// To check for codons are valid
const CODONS: [&'static str; 3_usize] = ["AUG", "UUU", "UCU"];


#[derive(Debug, Error)]
pub enum ProteinError {}
