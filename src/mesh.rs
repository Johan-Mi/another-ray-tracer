use crate::{Triangle, WorldPoint};
use obj::IndexTuple;
use std::{error::Error, path::Path};

pub fn load(path: &Path) -> Result<Vec<Triangle>, Box<dyn Error>> {
    let mesh = obj::Obj::load(path)?.data;

    mesh.objects
        .into_iter()
        .flat_map(|object| object.groups)
        .flat_map(|group| group.polys)
        .map(|poly| {
            let vertices = <[IndexTuple; 3]>::try_from(poly.0)
                .map_err(|_| "mesh contains non-triangular polygons")?
                .map(|vertex| mesh.position[vertex.0]);
            Ok(Triangle::new(vertices.map(WorldPoint::from)))
        })
        .collect()
}
