use super::layer::Layer;

pub struct Nucleotide<'a> {
    pub layer: &'a Layer,
    pub variant: usize,
}
