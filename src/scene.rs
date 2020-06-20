use super::light::*;
use super::object::geometry::GeometryGroup;
use super::object::Object;
use super::object::ObjectGroup;
use super::renderer::*;
use super::camera::Camera;
use super::Args;
use nalgebra::Vector3;
use std::convert::Into;

pub struct Scene {
    pub objs: ObjectGroup<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

impl Scene {
    pub fn box_scene(args: &Args) -> Scene {
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
        )
        .into();

        let box_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(10f64, 10f64, -10f64),
            Vector3::new(30f64, 30f64, 10f64),
        )
        .into();

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
        )
        .into();

        let sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(20f64, 20f64, 0f64), 8f64);

        let metal_sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(20f64, 5f64, 20f64), 5f64);

        let glass_sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(35f64, 5f64, 20f64), 7f64);

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

        let metal_sphere_mat = super::material::general::General::new(
            0f64,
            1f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1f64,
        );

        let glass_sphere_mat = super::material::general::General::new(
            0f64,
            0f64,
            1f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1.2f64,
        );

        let room_obj = super::object::geometry::GeometryObject::new(room_geo, room_mat);
        let sphere_obj = super::object::geometry::GeometryObject::new(sphere_geo, sphere_mat);
        let box_obj = super::object::geometry::GeometryObject::new(box_geo, box_mat);
        let inner_box_obj =
            super::object::geometry::GeometryObject::new(inner_box_geo, inner_box_mat);
        let metal_sphere_obj =
            super::object::geometry::GeometryObject::new(metal_sphere_geo, metal_sphere_mat);
        let glass_sphere_obj =
            super::object::geometry::GeometryObject::new(glass_sphere_geo, glass_sphere_mat);

        let objs: Vec<Box<dyn Object>> = vec![
            Box::new(room_obj),
            Box::new(sphere_obj),
            Box::new(box_obj),
            Box::new(inner_box_obj),
            Box::new(metal_sphere_obj),
            Box::new(glass_sphere_obj),
        ];

        let camera = super::camera::Camera::new(
            Point::new(-40f64, 50f64, 30f64),
            Dir::new(1f64, -0.5f64, -0.5f64).normalize(),
            Dir::new(0f64, 1f64, 0f64),
            args.width,
            args.height,
            50f64 * std::f64::consts::PI / 180f64,
            args.lens_radius,
            args.depth,
        );

        Scene {
            objs: objs.into(),
            lights: vec![Box::new(light)],
            camera,
        }
    }
    
    /*
    pub fn final_scene(args: Args) -> Scene {
    }
    */

    pub fn intersect(&self, ray: &Ray) -> Option<super::object::Intersect> {
        self.objs.intersect(ray, None)
    }
}
