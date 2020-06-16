use nalgebra::Vector3;
use crate::consts::*;

pub struct Sphere {
    radius: f64,
    center: Vector3<f64>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl super::Geometry for Sphere {
    fn intersect(&self, ray: &crate::renderer::Ray, upper: Option<f64>) -> Option<super::GeometryIntersect> {
        let sight = self.center - ray.origin;
        let projected = ray.dir.dot(&sight); // Dir should be normalized
        let tan = ray.dir * projected;
        let distv: Vector3<f64> = tan - sight;
        let distsq = distv.norm_squared();

        let tanlensq = self.radius * self.radius - distsq;
        if tanlensq <= EPS {
            return None;
        }

        let len = projected - tanlensq.sqrt();

        if len < EPS {
            return None;
        }

        if let Some(upper) = upper {
            if upper < len {
                return None;
            }
        }

        let hitpoint = ray.interpolate(len);

        return Some(super::GeometryIntersect {
            norm: (self.center - hitpoint).normalize(), // Reverted
            dist: len,
        });
    }

    fn bounding_box(&self) -> crate::object::BoundingBox {
        crate::object::BoundingBox {
            x: (self.center[0] - self.radius, self.center[0] + self.radius),
            y: (self.center[1] - self.radius, self.center[1] + self.radius),
            z: (self.center[2] - self.radius, self.center[2] + self.radius),
        }
    }
}
