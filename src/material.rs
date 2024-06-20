use crate::color;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Material {
    pub albedo: color::Hdr,
    #[serde(default)]
    pub emissivity: color::Hdr,
}
