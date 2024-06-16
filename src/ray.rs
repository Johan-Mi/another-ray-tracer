use crate::{ScreenSpace, WorldPoint, WorldSpace, WorldVector};
use euclid::RigidTransform3D;

pub struct Ray {
    transform: RigidTransform3D<f32, ScreenSpace, WorldSpace>,
}

pub struct Hit {
    point: WorldPoint,
    normal: WorldVector,
}
