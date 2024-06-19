#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::option_if_let_else)]

mod camera;
use camera::Camera;
mod color;
mod image;
mod mesh;
use image::Image;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Camera {
        transform: euclid::RigidTransform3D {
            rotation: euclid::Rotation3D::identity(),
            translation: euclid::Vector3D::zero(),
        },
        // https://en.wikipedia.org/wiki/Field_of_view
        vertical_field_of_view: euclid::Angle::degrees(150.0),
    };

    renderer::render(
        &mesh::load(Path::new("mesh.obj"))?,
        &camera,
        ScreenSize::new(480, 360),
        &Image::open(Path::new("skybox.png"))?,
        Path::new("image.ppm"),
    )?;

    Ok(())
}
