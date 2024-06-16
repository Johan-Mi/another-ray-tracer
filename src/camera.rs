use crate::{ScreenSpace, WorldSpace};
use euclid::{Angle, RigidTransform3D};

pub struct Camera {
    pub transform: RigidTransform3D<f32, ScreenSpace, WorldSpace>,
    pub vertical_field_of_view: Angle<f32>,
}
