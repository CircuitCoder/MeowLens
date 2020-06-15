use super::object::Object;
use super::light::Light;
use super::renderer::*;
use nalgebra::Vector3;

pub struct Scene {
    pub objs: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn box_scene() -> Scene {
        let light = Light::new(Point::new(20f64, 20f64, 0f64), Color::new(10f64, 10f64, 10f64));

        let floor_a_geo = super::object::geometry::triangle::Triangle::new([
            Vector3::new(0f64, 0f64, -50f64),
            Vector3::new(100f64, 0f64, -50f64),
            Vector3::new(0f64, 0f64, 50f64),
        ]);

        let floor_b_geo = super::object::geometry::triangle::Triangle::new([
            Vector3::new(100f64, 0f64, -50f64),
            Vector3::new(100f64, 0f64, 50f64),
            Vector3::new(0f64, 0f64, 50f64),
        ]);

        let floor_mat = super::material::general::General::new(
            0.1f64,
            0.9f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64), 
            Vector3::new(0.4f64, 0.4f64, 0.4f64),
            1f64, 1.4f64,
        );

        let floor_a = super::object::geometry::GeometryObject::new(floor_a_geo, floor_mat.clone());
        let floor_b = super::object::geometry::GeometryObject::new(floor_b_geo, floor_mat.clone());

        Scene {
            objs: vec![Box::new(floor_a), Box::new(floor_b)],
            lights: vec![light],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(super::object::Intersect, &dyn Object)> {
        let mut upper = None;
        let mut hit = None;
        for obj in self.objs.iter() {
            let int = obj.intersect(&ray, upper);
            let int = if let Some(i) = int { i } else {
                continue;
            };

            upper = Some(int.dist);
            hit = Some((int, obj.as_ref()));
        }

        hit
    }
}
