use crate::{Camera, ScreenSize, Triangle};
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
    for _ in 0..screen_size.area() {
        writer.write_all(&[255, 0, 0])?;
    }

    Ok(())
}
