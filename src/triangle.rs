use crate::{Hit, Ray, WorldLength, WorldPoint};
use std::ops::Range;

pub struct Triangle {
    vertices: [WorldPoint; 3],
}

impl Triangle {
    pub const fn new(vertices: [WorldPoint; 3]) -> Self {
        Self { vertices }
    }

    pub fn hit(&self, ray: &Ray, range: Range<WorldLength>) -> Option<Hit> {
        todo!()
    }
}
