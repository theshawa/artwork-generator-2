use serde::Deserialize;
use std::fs::File;

use super::{error::GenError, layer::Layer, variant::Variant};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub layers: Vec<Layer>,
}

impl Config {
    pub fn read_from_file(path: &str) -> Result<Config, GenError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(GenError::new_config(e.to_string()));
            }
        };

        let config: Config = match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(e) => {
                return Err(GenError::new_config(e.to_string()));
            }
        };

        config.validate()?;

        Ok(config)
    }

    pub fn get_maximum_generations_count(&self) -> usize {
        self.layers
            .iter()
            .map(|layer| layer.variants.len())
            .product()
    }

    pub fn validate(&self) -> Result<(), GenError> {
        if self.layers.is_empty() {
            return Err(GenError::new_config(
                "must have at least one layer".to_string(),
            ));
        }

        for layer in &self.layers {
            if layer.variants.len() < 1 {
                return Err(GenError::new_config(format!(
                    "layer \"{}\" must have at least one variant",
                    layer.name
                )));
            }
        }

        Ok(())
    }

    pub fn new_dummy() -> Config {
        Config {
            layers: vec![
                Layer {
                    name: String::from("l1"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l2"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v5"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v6"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l3"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v5"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l4"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l5"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l6"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l7"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l8"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v5"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v6"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l9"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v5"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l10"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l11"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                    ],
                },
                Layer {
                    name: String::from("l12"),
                    opacity: Some(1.0),
                    variants: vec![
                        Variant {
                            asset: String::from("v1"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v2"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v3"),
                            weight: Some(1),
                        },
                        Variant {
                            asset: String::from("v4"),
                            weight: Some(1),
                        },
                    ],
                },
            ],
        }
    }
}
