use config::Config;
use dna::Dna;
use dna_tree::DnaTree;
use error::GenError;
use nucleotide::Nucleotide;
use rand::Rng;

pub mod config;
pub mod dna;
pub mod dna_tree;
pub mod error;
pub mod layer;
pub mod nucleotide;
pub mod variant;

#[derive(Debug)]
pub struct Generator {
    pub config: Config,
    dna_tree: DnaTree,
}

impl Generator {
    pub fn new(config_path: &str) -> Result<Generator, GenError> {
        let config = Config::read_from_file(config_path)?;
        Ok(Generator {
            config,
            dna_tree: DnaTree::new(),
        })
    }

    pub fn dummy() -> Generator {
        let config = Config::new_dummy();
        Generator {
            config,
            dna_tree: DnaTree::new(),
        }
    }

    pub fn generate(&mut self, count: usize) -> Result<usize, GenError> {
        if count < 1 {
            return Err(String::from("count must be greater than 0").into());
        }

        if count > self.config.get_maximum_generations_count() {
            return Err(GenError::new(
                "error: count must be less than or equal to the maximum number of generations"
                    .to_string(),
            ));
        }

        let mut dna_list: Vec<Dna> = Vec::new();

        for _ in 0..count {
            loop {
                let mut nucleotides: Vec<Nucleotide> = Vec::new();
                for layer in &self.config.layers {
                    let total_weight: u32 = layer
                        .variants
                        .iter()
                        .map(|var| var.weight.unwrap_or(1))
                        .sum();
                    let mut rng: i32 = rand::thread_rng()
                        .gen_range(0..total_weight)
                        .try_into()
                        .unwrap();
                    for (i, variant) in layer.variants.iter().enumerate() {
                        let weight: i32 = variant.weight.unwrap_or(1).try_into().unwrap();
                        rng -= weight;
                        if rng < 0 {
                            let nucleotide = Nucleotide { layer, variant: i };
                            nucleotides.push(nucleotide);
                            break;
                        }
                    }
                }
                let dna = Dna::new(nucleotides);
                let id = dna.get_id();
                if !self.dna_tree.search(id) {
                    self.dna_tree.insert(id);
                    dna_list.push(dna);
                    println!("dna generated: {id}");
                    break;
                }
            }
        }

        Ok(dna_list.len())
    }
}
