use crate::{Hit, Ray, WorldPoint};

pub struct Triangle {
    vertices: [WorldPoint; 3],
}

impl Triangle {
    pub const fn new(vertices: [WorldPoint; 3]) -> Self {
        Self { vertices }
    }

    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        todo!()
    }
}
