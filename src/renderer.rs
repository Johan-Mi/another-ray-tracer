use crate::{
    color, image::UvPoint, Camera, Hit, Image, Ray, ScreenPoint, ScreenSize,
    Triangle, WorldLength, WorldVector,
};
use euclid::Vector3D;
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

    let print_stats = std::env::var_os("STATS").is_some();
    let start_time = std::time::Instant::now();
    let total_pixels = screen_size.area();
    let mut rendered_pixels = 0;

    writeln!(
        writer,
        "P6 {} {} 255",
        screen_size.width, screen_size.height
    )?;
    for y in 0..screen_size.height {
        for x in 0..screen_size.width {
            let ray = camera.ray_for_pixel(ScreenPoint::new(x, y), screen_size);
            let color = color_of_ray(&ray, triangles, skybox, 5);
            writer.write_all(&color::hdr_to_srgb(color).to_array())?;

            #[allow(clippy::cast_precision_loss)]
            if print_stats {
                rendered_pixels += 1;
                if rendered_pixels % 10000 == 0 {
                    eprint!(
                        "\r{:02.3}%",
                        rendered_pixels as f32 / total_pixels as f32 * 100.0
                    );
                }
            }
        }
    }

    if print_stats {
        println!("\nfinished rendering in {:?}", start_time.elapsed());
    }

    Ok(())
}

fn color_of_ray(
    ray: &Ray,
    triangles: &[Triangle],
    skybox: &Image,
    max_bounces: usize,
) -> color::Hdr {
    if max_bounces == 0 {
        return color::Hdr::zero();
    }

    let mut range =
        WorldLength::new(0.000_000_1)..WorldLength::new(f32::INFINITY);
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
        let reflected_ray = Ray {
            origin: hit.point,
            direction: ray.direction.reflect(hit.normal),
        };
        color_of_ray(&reflected_ray, triangles, skybox, max_bounces - 1)
            .to_vector()
            .component_mul(Vector3D::new(0.1, 1.0, 0.3))
            .to_point()
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
