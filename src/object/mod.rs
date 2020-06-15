pub mod geometry;

use super::renderer::*;
use super::material::Material;
use std::rc::Rc;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BoundingBox {
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
}

#[derive(Clone, Debug)]
pub struct Intersect {
    pub norm: Dir,
    pub dist: f64,
}

pub trait Object {
    fn intersect(&self, ray: &Ray, upper: Option<f64>) -> Option<Intersect>;
    fn bounding_box(&self) -> BoundingBox;
    fn material(&self) -> Rc<dyn Material>;
}
