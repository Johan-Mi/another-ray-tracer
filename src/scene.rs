use crate::{Camera, Material, WorldPoint};
use euclid::{RigidTransform3D, Rotation3D, Vector2D};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub skybox: String,
    pub mesh: String,
    pub material: Material,
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Camera {
            position: WorldPoint,
            look_at: WorldPoint,
            vertical_field_of_view: Angle,
        }

        Camera::deserialize(deserializer).map(|it| {
            let direction = (it.look_at - it.position).normalize();
            let pitch = euclid::Angle::radians(direction.y.asin());
            let forward = Vector2D::new(0.0, 1.0);
            let yaw = direction.xz().angle_to(forward);
            Self {
                transform: RigidTransform3D {
                    rotation: Rotation3D::euler(euclid::Angle::default(), pitch, yaw),
                    translation: it.position.to_vector(),
                },
                vertical_field_of_view: it.vertical_field_of_view.into(),
            }
        })
    }
}

#[derive(Deserialize)]
enum Angle {
    Radians(f32),
    Degrees(f32),
}

impl From<Angle> for euclid::Angle<f32> {
    fn from(value: Angle) -> Self {
        match value {
            Angle::Radians(n) => Self::radians(n),
            Angle::Degrees(n) => Self::degrees(n),
        }
    }
}
