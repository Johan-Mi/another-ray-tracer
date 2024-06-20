use euclid::{Point3D, Vector3D};

pub enum HdrSpace {}

pub type Hdr = Vector3D<f32, HdrSpace>;

pub enum SrgbSpace {}

pub type Srgb = Point3D<u8, SrgbSpace>;

pub fn hdr_to_srgb(color: Hdr) -> Srgb {
    // TODO: implement a proper tone mapping operator
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    (color * 255.0)
        .map(|channel| channel as u8)
        .to_point()
        .cast_unit()
}

pub fn srgb_to_hdr(color: Srgb) -> Hdr {
    (color.cast::<f32>() / 255.0).to_vector().cast_unit()
}
