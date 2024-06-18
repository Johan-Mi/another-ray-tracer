use euclid::Point3D;

pub enum HdrSpace {}

pub type Hdr = Point3D<f32, HdrSpace>;

pub enum SrgbSpace {}

pub type Srgb = Point3D<u8, SrgbSpace>;

pub fn hdr_to_srgb(color: Hdr) -> Srgb {
    // TODO: implement a proper tone mapping operator
    (color * 255.0).cast().cast_unit()
}
