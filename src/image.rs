use crate::color;
use euclid::{Point2D, Vector2D};
use std::{fs::File, path::Path};

pub enum UvSpace {}

pub type UvPoint = Point2D<f32, UvSpace>;

pub struct Image {
    width: u32,
    height: u32,
    bytes: Box<[u8]>,
}

impl Image {
    pub fn open(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let decoder = png::Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        if info.color_type != png::ColorType::Rgb {
            return Err(format!("unsupported color type: {:?}", info.color_type).into());
        }
        let bytes = &buf[..info.buffer_size()];
        Ok(Self {
            width: info.width,
            height: info.height,
            bytes: bytes.into(),
        })
    }

    pub fn sample_uv(&self, uv: UvPoint) -> color::Srgb {
        let xy = uv
            .to_vector()
            .component_mul(Vector2D::new(self.width, self.height).cast())
            .cast::<usize>();
        let index = (xy.y * self.width as usize + xy.x) * 3;
        color::Srgb::new(
            self.bytes[index],
            self.bytes[index + 1],
            self.bytes[index + 2],
        )
    }
}
