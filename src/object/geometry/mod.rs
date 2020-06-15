pub mod triangle;

use crate::material::Material;

pub trait Geometry {
    fn intersect(&self, ray: &crate::renderer::Ray, upper: Option<f64>) -> Option<super::Intersect>;
    fn bounding_box(&self) -> super::BoundingBox;
}

pub struct GeometryObject<G: Geometry> {
    geometry: G,
    material: std::rc::Rc<dyn Material>,
}

impl<G> GeometryObject<G> where G: Geometry {
    pub fn new<M: Material + 'static>(g: G, m: M) -> Self {
        Self {
            geometry: g,
            material: std::rc::Rc::new(m),
        }
    }
}

impl<G> super::Object for GeometryObject<G> where G: Geometry {
    fn intersect(&self, ray: &crate::renderer::Ray, upper: Option<f64>) -> Option<super::Intersect> {
        self.geometry.intersect(ray, upper)
    }
    fn bounding_box(&self) -> super::BoundingBox {
        self.geometry.bounding_box()
    }

    fn material(&self) -> std::rc::Rc<dyn Material> {
        self.material.clone()
    }
}
