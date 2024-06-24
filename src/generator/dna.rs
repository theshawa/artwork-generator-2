use super::nucleotide::Nucleotide;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub struct Dna<'a> {
    pub nucleotides: Vec<Nucleotide<'a>>,
}

impl<'a> Dna<'a> {
    pub fn new(nucleotides: Vec<Nucleotide<'a>>) -> Dna<'a> {
        Dna { nucleotides }
    }

    pub fn get_id(&self) -> u64 {
        let mut id = String::new();
        for nucleotide in &self.nucleotides {
            if id.len() > 0 {
                id.push_str(".");
            }
            id.push_str(&nucleotide.layer.name.as_str());
            id.push_str(":");
            id.push_str(&nucleotide.variant.to_string());
        }
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }
}
