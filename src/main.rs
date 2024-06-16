#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod camera;
use camera::Camera;
mod ray;
use ray::{Hit, Ray};
mod triangle;
use triangle::Triangle;

enum ScreenSpace {}
enum WorldSpace {}

type WorldLength = euclid::Length<f32, WorldSpace>;
type WorldPoint = euclid::Point3D<f32, WorldSpace>;
type WorldVector = euclid::Vector3D<f32, WorldSpace>;

fn main() {
    let camera = Camera {
        transform: euclid::RigidTransform3D {
            rotation: euclid::Rotation3D::identity(),
            translation: euclid::Vector3D::zero(),
        },
        // https://en.wikipedia.org/wiki/Field_of_view
        vertical_field_of_view: euclid::Angle::degrees(150.0),
    };

    println!("Hello, world!");
}
