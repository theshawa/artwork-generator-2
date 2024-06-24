use serde::Deserialize;

use super::variant::Variant;

#[derive(Deserialize, Debug)]
pub struct Layer {
    pub name: String,
    pub opacity: Option<f32>,
    pub variants: Vec<Variant>,
}
