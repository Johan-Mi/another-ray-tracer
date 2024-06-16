#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod camera;
use camera::Camera;

enum ScreenSpace {}
enum WorldSpace {}

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
