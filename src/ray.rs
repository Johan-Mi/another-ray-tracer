use crate::{WorldLength, WorldPoint, WorldVector};

pub struct Ray {
    pub origin: WorldPoint,
    pub direction: WorldVector,
}

impl Ray {
    pub fn at(&self, length: WorldLength) -> WorldPoint {
        self.origin + self.direction * length.get()
    }
}

pub struct Hit {
    pub ray_length: WorldLength,
    pub point: WorldPoint,
    pub normal: WorldVector,
}
