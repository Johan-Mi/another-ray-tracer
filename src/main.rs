#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod camera;
use camera::Camera;
mod color;
mod ray;
use ray::{Hit, Ray};
mod renderer;
mod triangle;
use triangle::Triangle;

use std::path::Path;

enum ScreenSpace {}
enum WorldSpace {}

type ScreenPoint = euclid::Point2D<usize, ScreenSpace>;
type ScreenSize = euclid::Size2D<usize, ScreenSpace>;
type WorldLength = euclid::Length<f32, WorldSpace>;
type WorldPoint = euclid::Point3D<f32, WorldSpace>;
type WorldVector = euclid::Vector3D<f32, WorldSpace>;

fn main() -> std::io::Result<()> {
    let camera = Camera {
        transform: euclid::RigidTransform3D {
            rotation: euclid::Rotation3D::identity(),
            translation: euclid::Vector3D::zero(),
        },
        // https://en.wikipedia.org/wiki/Field_of_view
        vertical_field_of_view: euclid::Angle::degrees(150.0),
    };

    let triangle = Triangle::new([
        WorldPoint::new(-0.5, 00.5, 1.0),
        WorldPoint::new(00.5, 00.0, 1.0),
        WorldPoint::new(-0.5, -0.5, 1.0),
    ]);

    let screen_size = ScreenSize::new(480, 360);

    renderer::render(&triangle, &camera, screen_size, Path::new("image.ppm"))
}
