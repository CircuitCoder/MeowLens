use nalgebra::*;
use crate::consts::EPS;

pub struct Triangle {
    vertices: [Vector3<f64>; 3],
    normal: Vector3<f64>,
}

impl Triangle {
    pub fn new(vertices: [Vector3<f64>; 3]) -> Self {
        let leg_a = vertices[1] - vertices[0];
        let leg_b = vertices[2] - vertices[0];

        let normal = leg_a.cross(&leg_b).normalize();

        Triangle {
            vertices,
            normal,
        }
    }
}

impl super::Geometry for Triangle {
    fn intersect(&self, ray: &crate::renderer::Ray, upper: Option<f64>) -> Option<super::GeometryIntersect> {
        let e1 = self.vertices[0] - self.vertices[1];
        let e2 = self.vertices[0] - self.vertices[2];
        let s = self.vertices[0] - ray.origin;

        if s.norm() < EPS { return None };

        let d = Matrix3::from_columns(&[ray.dir, e1, e2]).determinant();
        if d.abs() < EPS {
            return None;
        }

        let t = Matrix3::from_columns(&[s, e1, e2]).determinant() / d;
        let beta = Matrix3::from_columns(&[ray.dir, s, e2]).determinant() / d;
        let gamma= Matrix3::from_columns(&[ray.dir, e1, s]).determinant() / d;

        if beta < 0f64 || gamma < 0f64 || beta + gamma > 1f64{
            return None;
        }

        if t < EPS {
            return None;
        }

        if let Some(upper) = upper {
            if upper < t {
                return None;
            }
        }

        Some(
            super::GeometryIntersect {
                dist: t,
                norm: self.normal.clone(),
            }
        )
    }

    fn bounding_box(&self) -> crate::object::BoundingBox {
        crate::object::BoundingBox {
            x: self.vertices.iter().fold((std::f64::NEG_INFINITY, std::f64::INFINITY), |(lower, upper), elem| {
                (
                    lower.min(elem[0]),
                    upper.max(elem[0]),
                )
            }),
            y: self.vertices.iter().fold((std::f64::NEG_INFINITY, std::f64::INFINITY), |(lower, upper), elem| {
                (
                    lower.min(elem[1]),
                    upper.max(elem[1]),
                )
            }),
            z: self.vertices.iter().fold((std::f64::NEG_INFINITY, std::f64::INFINITY), |(lower, upper), elem| {
                (
                    lower.min(elem[2]),
                    upper.max(elem[2]),
                )
            }),
        }
    }
}
