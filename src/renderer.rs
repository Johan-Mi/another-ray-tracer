use crate::{
    color, image::UvPoint, Camera, Hit, Image, Ray, ScreenPoint, ScreenSize,
    Triangle, WorldLength, WorldVector,
};
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn render(
    triangles: &[Triangle],
    camera: &Camera,
    screen_size: ScreenSize,
    skybox: &Image,
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
            let color = color_of_ray(&ray, triangles, skybox);
            writer.write_all(&color::hdr_to_srgb(color).to_array())?;
        }
    }

    Ok(())
}

fn color_of_ray(
    ray: &Ray,
    triangles: &[Triangle],
    skybox: &Image,
) -> color::Hdr {
    let mut range = WorldLength::new(0.0)..WorldLength::new(f32::INFINITY);
    let mut closest_hit = None::<Hit>;
    for triangle in triangles {
        let Some(hit) = triangle.hit(ray, range.clone()) else {
            continue;
        };
        if !closest_hit
            .as_ref()
            .is_some_and(|it| it.ray_length < hit.ray_length)
        {
            range.end = hit.ray_length;
            closest_hit = Some(hit);
        }
    }

    if let Some(hit) = closest_hit {
        color::Hdr::new(0.0, 1.0, 0.0)
    } else {
        sky(skybox, ray.direction)
    }
}

fn sky(skybox: &Image, direction: WorldVector) -> color::Hdr {
    let pitch = direction.y.mul_add(0.5, 0.5);
    let yaw = direction.xz().angle_from_x_axis().positive().radians
        / std::f32::consts::TAU;
    color::srgb_to_hdr(skybox.sample_uv(UvPoint::new(yaw, pitch)))
}
