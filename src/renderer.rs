use crate::{Camera, ScreenPoint, ScreenSize, Triangle, WorldLength};
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn render(
    triangle: &Triangle,
    camera: &Camera,
    screen_size: ScreenSize,
    image_path: &Path,
) -> io::Result<()> {
    let file = File::create(image_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(
        writer,
        "P6 {} {} 255",
        screen_size.width, screen_size.height
    )?;
    for y in 0..screen_size.height {
        for x in 0..screen_size.width {
            let ray = camera.ray_for_pixel(ScreenPoint::new(x, y), screen_size);
            let range = WorldLength::new(0.0)..WorldLength::new(f32::INFINITY);
            let color = if triangle.hit(&ray, range).is_some() {
                [0, 255, 0]
            } else {
                [255, 0, 0]
            };
            writer.write_all(&color)?;
        }
    }

    Ok(())
}
