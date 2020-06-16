pub mod triangle;
pub mod sphere;
pub mod util;

use crate::material::Material;
use crate::renderer::*;

#[derive(Clone, Debug)]
pub struct GeometryIntersect {
    pub norm: Dir,
    pub dist: f64,
}

impl GeometryIntersect {
    fn convert<'a>(self, material: &'a dyn Material) -> super::Intersect<'a> {
        super::Intersect {
            norm: self.norm,
            dist: self.dist,
            material,
        }
    }
}

pub trait Geometry : Sync + Send {
    fn intersect(&self, ray: &crate::renderer::Ray, upper: Option<f64>) -> Option<GeometryIntersect>;
    fn bounding_box(&self) -> super::BoundingBox;
}

pub struct GeometryObject<G: Geometry, M: Material> {
    geometry: G,
    material: M,
}

impl<G, M> GeometryObject<G, M> where G: Geometry, M: Material {
    pub fn new(g: G, m: M) -> Self {
        Self {
            geometry: g,
            material: m,
        }
    }
}

impl<G, M> super::Object for GeometryObject<G, M> where G: Geometry, M: Material {
    fn intersect(&self, ray: &crate::renderer::Ray, upper: Option<f64>) -> Option<super::Intersect> {
        self.geometry.intersect(ray, upper).map(|gi| gi.convert(&self.material))
    }
    fn bounding_box(&self) -> super::BoundingBox {
        self.geometry.bounding_box()
    }
}

pub struct GeometryGroup<G: Geometry> {
    content: Vec<(G, super::BoundingBox)>,
}

impl<G> From<Vec<G>> for GeometryGroup<G> where G: Geometry {
    fn from(input: Vec<G>) -> Self {
        Self {
            content: input.into_iter().map(|g| {
                let bb = g.bounding_box();
                (g, bb)
            }).collect()
        }
    }
}

impl<G> Geometry for GeometryGroup<G> where G: Geometry {
    fn intersect(&self, ray: &Ray, mut upper: Option<f64>) -> Option<GeometryIntersect> {
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

    fn bounding_box(&self) -> super::BoundingBox {
        self.content.iter().fold(super::BoundingBox::unbounded(), |acc, (_, bb)| acc.merge(bb))
    }
}
