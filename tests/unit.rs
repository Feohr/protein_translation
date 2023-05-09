use protein_translation::*;

#[test]
fn protein_translate() {
    let rna = "AUGUUUUCUUAAAUG";
    assert!(rna.protein_translate().is_ok());
    let rna = "AUGUUUUCUUAAAUG".to_string();
    assert!(rna.protein_translate().is_ok());
}
