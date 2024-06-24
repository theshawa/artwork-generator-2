#[cfg(test)]
#[test]
fn dna_tree() {
    use crate::generator::dna_tree::DnaTree;

    let mut tree = DnaTree::new();
    tree.insert(10);
    tree.insert(20);
    tree.insert(5);

    tree.print_inorder();

    assert!(tree.search(10));
    assert!(tree.search(5));
    assert!(tree.search(20));
}
