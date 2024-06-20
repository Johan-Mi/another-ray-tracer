use crate::{Ray, ScreenPoint, ScreenSize, ScreenSpace, WorldSpace};
use euclid::{Angle, RigidTransform3D, Vector2D};

pub struct Camera {
    pub transform: RigidTransform3D<f32, ScreenSpace, WorldSpace>,
    pub vertical_field_of_view: Angle<f32>,
}

impl Camera {
    pub fn ray_for_pixel(
        &self,
        pixel: ScreenPoint,
        screen_size: ScreenSize,
    ) -> Ray {
        let pixel =
            pixel.cast() + Vector2D::new(fastrand::f32(), fastrand::f32());
        let screen_size = screen_size.cast();
        let point = (pixel * 2.0f32 - screen_size) / screen_size.height;
        let angle_offset = (self.vertical_field_of_view * 0.5).radians.atan();
        let direction = (point.to_vector() * angle_offset)
            .extend(1.0)
            .normalize()
            .cast_unit();
        Ray {
            origin: self.transform.translation.to_point(),
            direction: self.transform.rotation.transform_vector3d(direction),
        }
    }
}
