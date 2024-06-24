use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Variant {
    pub asset: String,
    pub weight: Option<u32>,
}
