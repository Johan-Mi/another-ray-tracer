#![forbid(unsafe_code)]
#![deny(clippy::allow_attributes, clippy::allow_attributes_without_reason)]
#![warn(clippy::nursery, clippy::pedantic)]
#![expect(clippy::option_if_let_else, reason = "less readable")]

mod camera;
use camera::Camera;
mod color;
mod image;
mod material;
use material::Material;
mod mesh;
use image::Image;
mod ray;
use ray::{Hit, Ray};
mod renderer;
mod scene;
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
    let mut args = std::env::args().skip(1);
    let screen_size = ScreenSize::from(
        args.next()
            .and_then(|arg| {
                arg.split_once('x').and_then(|(width, height)| {
                    width.parse().ok().zip(height.parse().ok())
                })
            })
            .unwrap_or((480, 360)),
    );
    let samples_per_pixel =
        args.next().and_then(|arg| arg.parse().ok()).unwrap_or(1);

    let scene =
        ron::from_str::<scene::Scene>(&std::fs::read_to_string("scene.ron")?)?;

    renderer::render(
        &mesh::load(Path::new(&scene.mesh))?,
        &scene.camera,
        &scene.material,
        screen_size,
        samples_per_pixel,
        &Image::open(Path::new(&scene.skybox))?,
        Path::new("image.ppm"),
    )?;

    Ok(())
}
