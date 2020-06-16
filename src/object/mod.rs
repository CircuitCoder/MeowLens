pub mod geometry;

use super::renderer::*;
use crate::consts::*;
use super::material::Material;
use std::sync::Arc;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BoundingBox {
    pub x: (f64, f64),
    pub y: (f64, f64),
    pub z: (f64, f64),
}

impl BoundingBox {
    fn unbounded() -> Self {
        Self {
            x: (std::f64::NEG_INFINITY, std::f64::INFINITY),
            y: (std::f64::NEG_INFINITY, std::f64::INFINITY),
            z: (std::f64::NEG_INFINITY, std::f64::INFINITY),
        }
    }

    fn merge(&self, another: &BoundingBox) -> BoundingBox {
        BoundingBox {
            x: (self.x.0.min(another.x.0), self.x.1.max(another.x.1)),
            y: (self.y.0.min(another.y.0), self.y.1.max(another.y.1)),
            z: (self.z.0.min(another.z.0), self.z.1.max(another.z.1)),
        }
    }

    fn hit(&self, ray: &Ray, upper: Option<f64>) -> bool {
        let bound = if let Some(u) = upper { u } else { std::f64::INFINITY };

        let tx1 = if ray.dir[0].abs() < EPS { std::f64::INFINITY } else { (self.x.0 - ray.origin[0]) * ray.invdir[0] };
        let tx2 = if ray.dir[0].abs() < EPS { std::f64::NEG_INFINITY } else { (self.x.1 - ray.origin[0]) * ray.invdir[0] };

        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2).min(bound);

        if tmin > bound { return false; }

        if ray.dir[1].abs() > EPS {
            let ty1 = (self.y.0 - ray.origin[1]) * ray.invdir[1];
            let ty2 = (self.y.1 - ray.origin[1]) * ray.invdir[1];

            let tymin = ty1.min(ty2);
            let tymax = ty1.max(ty2);

            if tmin > tymax || tmax < tymin { return false; }
            tmin = tmin.max(tymin);
            tmax = tmax.min(tymax);
        }

        if ray.dir[2].abs() > EPS {
            let tz1 = (self.z.0 - ray.origin[2]) * ray.invdir[2];
            let tz2 = (self.z.1 - ray.origin[2]) * ray.invdir[2];

            let tzmin = tz1.min(tz2);
            let tzmax = tz1.max(tz2);

            if tmin > tzmax || tmax < tzmin { return false; }
        }

        true
    }
}

#[derive(Clone)]
pub struct Intersect<'a> {
    pub norm: Dir,
    pub dist: f64,
    pub material: &'a dyn Material,
}

pub trait Object: Sync + Send {
    fn intersect(&self, ray: &Ray, upper: Option<f64>) -> Option<Intersect>;
    fn bounding_box(&self) -> BoundingBox;
}

impl<T> Object for T where T: AsRef<dyn Object> + Send + Sync {
    fn intersect(&self, ray: &Ray, upper: Option<f64>) -> Option<Intersect> {
        self.as_ref().intersect(ray, upper)
    }
    fn bounding_box(&self) -> BoundingBox {
        self.as_ref().bounding_box()
    }
}

pub struct ObjectGroup<O: Object> {
    content: Vec<(O, BoundingBox)>,
}

impl<O> From<Vec<O>> for ObjectGroup<O> where O: Object {
    fn from(input: Vec<O>) -> Self {
        Self {
            content: input.into_iter().map(|g| {
                let bb = g.bounding_box();
                (g, bb)
            }).collect()
        }
    }
}

impl<O> Object for ObjectGroup<O> where O: Object {
    fn intersect(&self, ray: &Ray, mut upper: Option<f64>) -> Option<Intersect> {
        let mut result = None;
        for (obj, bb) in self.content.iter() {
            if !bb.hit(ray, upper) {
                continue
            }

            let hit = obj.intersect(ray, upper);

            if let Some(int) = hit {
                upper = Some(int.dist);
                result = Some(int);
            }
        }
        result
    }

    fn bounding_box(&self) -> BoundingBox {
        self.content.iter().fold(BoundingBox::unbounded(), |acc, (_, bb)| acc.merge(bb))
    }
}
