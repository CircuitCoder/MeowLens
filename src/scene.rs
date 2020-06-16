use super::light::*;
use super::object::ObjectGroup;
use super::object::Object;
use super::object::geometry::GeometryGroup;
use super::renderer::*;
use nalgebra::Vector3;
use std::convert::Into;

pub struct Scene {
    pub objs: ObjectGroup<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn box_scene() -> Scene {
        let light = SemisphereLight::new(
            Point::new(10f64, 60f64, 20f64),
            Color::new(10f64, 10f64, 10f64),
            16f64 * 1024f64,
            Dir::new(0f64, -1f64, 0f64),
        );

        let room_mat = super::material::general::General::new(
            0.2f64,
            0f64, // TODO: why does this even matters?
            0.5f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.4f64, 0.4f64, 0.4f64),
            1f64,
            1f64,
        );

        let room_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(-50f64, 0f64, -50f64),
            Vector3::new(50f64, 100f64, 50f64),
        ).into();

        let box_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(10f64, 10f64, -10f64),
            Vector3::new(30f64, 30f64, 10f64),
        ).into();

        let box_mat = super::material::general::General::new(
            0f64,
            0f64,
            1f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0f64, 0f64, 1f64),
            1f64,
            1.5f64,
        );

        let inner_box_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(11f64, 11f64, -9f64),
            Vector3::new(29f64, 12f64, -8f64),
        ).into();

        let sphere_geo = super::object::geometry::sphere::Sphere::new(
            Vector3::new(20f64, 20f64, 0f64),
            8f64,
        );

        let sphere_mat = super::material::general::General::new(
            1f64,
            0f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1f64,
        );

        let inner_box_mat = super::material::general::General::new(
            1f64,
            0f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 0.7f64, 0.7f64),
            1f64,
            1f64,
        );

        let room_obj = super::object::geometry::GeometryObject::new(room_geo, room_mat);
        let sphere_obj = super::object::geometry::GeometryObject::new(sphere_geo, sphere_mat);
        let box_obj = super::object::geometry::GeometryObject::new(box_geo, box_mat);
        let inner_box_obj = super::object::geometry::GeometryObject::new(inner_box_geo, inner_box_mat);
        
        let objs: Vec<Box<dyn Object>> = vec![Box::new(room_obj), Box::new(sphere_obj), Box::new(box_obj), Box::new(inner_box_obj)];

        Scene {
            objs: objs.into(),
            lights: vec![Box::new(light)],

        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<super::object::Intersect> {
        self.objs.intersect(ray, None)
    }
}
