use crate::{Hit, Ray, WorldLength, WorldPoint, WorldVector};
use std::ops::Range;

pub struct Triangle {
    vertices: [WorldPoint; 3],
    normal: WorldVector,
}

impl Triangle {
    pub fn new(vertices: [WorldPoint; 3]) -> Self {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        let normal = e1.cross(e2).normalize();
        Self { vertices, normal }
    }

    pub fn hit(&self, ray: &Ray, range: Range<WorldLength>) -> Option<Hit> {
        const EPSILON: f32 = 0.000_000_1;

        let [v0, v1, v2] = self.vertices;

        let edge_1 = v1 - v0;
        let edge_2 = v2 - v0;

        let dir_cross_e2 = ray.direction.cross(edge_2);
        let a = edge_1.dot(dir_cross_e2);
        if a.abs() < EPSILON {
            return None;
        }

        let inv_a = 1.0 / a;
        let offset = ray.origin - v0;
        let u = inv_a * offset.dot(dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let s_cross_e1 = offset.cross(edge_1);
        let v = inv_a * ray.direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = WorldLength::new(inv_a * edge_2.dot(s_cross_e1));
        if range.contains(&t) {
            Some(Hit {
                point: ray.at(t),
                normal: self.normal,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::{Ray, Triangle, WorldLength, WorldPoint, WorldVector};

        let triangle = Triangle::new([
            WorldPoint::new(1.0, -0.5, 00.5),
            WorldPoint::new(1.0, 00.5, 00.0),
            WorldPoint::new(1.0, -0.5, -0.5),
        ]);
        let range = WorldLength::new(0.0)..WorldLength::new(f32::INFINITY);

        let ray = Ray {
            origin: WorldPoint::zero(),
            direction: WorldVector::new(1.0, 0.0, 0.0),
        };
        assert!(triangle.hit(&ray, range.clone()).is_some());

        let ray = Ray {
            origin: WorldPoint::zero(),
            direction: WorldVector::new(-1.0, 0.0, 0.0),
        };
        assert!(triangle.hit(&ray, range.clone()).is_none());

        let ray = Ray {
            origin: WorldPoint::zero(),
            direction: WorldVector::new(1.0, 4.0, 0.0).normalize(),
        };
        assert!(triangle.hit(&ray, range).is_none());
    }
}
