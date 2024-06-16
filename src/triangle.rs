//! Doug Baldwin and Michael Weber, Fast Ray-Triangle Intersections by
//! Coordinate Transformation, Journal of Computer Graphics Techniques
//! (JCGT), vol. 5, no. 3, 39-49, 2016
//! Available online <http://jcgt.org/published/0005/03/03/>

use crate::{Hit, Ray, WorldLength, WorldPoint, WorldSpace, WorldVector};
use std::ops::Range;

pub struct Triangle {
    normal: WorldVector,
    inverse_transformation: [f32; 9],
    free_vector: FreeVector,
}

impl Triangle {
    pub fn new(vertices: [WorldPoint; 3]) -> Self {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        let normal = e1.cross(e2).normalize();

        let normal_abs = normal.abs();

        Self {
            normal,
            inverse_transformation: [
                e2.z,
                -e2.y,
                vertices[2].to_vector().cross(vertices[0].to_vector()).x,
                -e1.z,
                e1.y,
                -vertices[1].to_vector().cross(vertices[0].to_vector()).x,
                normal.y,
                normal.z,
                normal.dot(vertices[0].to_vector()),
            ]
            .map(|n| n / normal.x),
            free_vector: if normal_abs.y < normal_abs.x
                && normal_abs.z < normal_abs.x
            {
                FreeVector::A
            } else if normal_abs.x < normal_abs.y && normal_abs.z < normal_abs.y
            {
                FreeVector::B
            } else {
                FreeVector::C
            },
        }
    }

    pub fn hit(&self, ray: &Ray, range: Range<WorldLength>) -> Option<Hit> {
        let q = self.inverse_transformation;
        let world_to_barycentric: euclid::Transform3D<
            f32,
            WorldSpace,
            BarycentricSpace,
        > = match self.free_vector {
            #[rustfmt::skip]
            FreeVector::A => euclid::Transform3D::new(
                0.0, q[0], q[1], q[2],
                0.0, q[3], q[4], q[5],
                1.0, q[6], q[7], q[8],
                0.0, 0.0,  0.0,  1.0,
            ),
            #[rustfmt::skip]
            FreeVector::B => euclid::Transform3D::new(
                q[0], 0.0, q[1], q[2],
                q[3], 0.0, q[4], q[5],
                q[6], 1.0, q[7], q[8],
                0.0,  0.0, 0.0,  1.0,
            ),
            #[rustfmt::skip]
            FreeVector::C => euclid::Transform3D::new(
                q[0], q[1], 0.0, q[2],
                q[3], q[4], 0.0, q[5],
                q[6], q[7], 1.0, q[8],
                0.0,  0.0,  0.0, 1.0,
            ),
        };

        let transformed_origin =
            world_to_barycentric.transform_point3d(ray.origin).unwrap();
        let transformed_direction =
            world_to_barycentric.transform_vector3d(ray.direction);
        let ray_length = -transformed_origin.z / transformed_direction.z;

        let intersection =
            transformed_origin.xy() + transformed_direction.xy() * ray_length;

        let ray_length = WorldLength::new(ray_length);
        if (0.0..=1.0).contains(&intersection.x)
            && (0.0..=1.0).contains(&intersection.y)
            && intersection.x + intersection.y <= 1.0
            && range.contains(&ray_length)
        {
            Some(Hit {
                point: ray.at(ray_length),
                normal: self.normal,
            })
        } else {
            None
        }
    }
}

enum BarycentricSpace {}

enum FreeVector {
    A,
    B,
    C,
}
