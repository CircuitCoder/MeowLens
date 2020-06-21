use super::camera::Camera;
use super::light::*;
use super::object::geometry::GeometryGroup;
use super::object::geometry::GeometryObject;
use super::object::Object;
use super::object::ObjectGroup;
use super::renderer::*;
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
            8f64 * 1024f64,
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
            0f64,
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
            0f64,
        );

        let inner_box_1_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(11f64, 11f64, -9f64),
            Vector3::new(29f64, 12f64, -8f64),
        )
        .into();

        let inner_box_2_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(11f64, 28f64, -9f64),
            Vector3::new(29f64, 29f64, -8f64),
        )
        .into();

        let inner_box_3_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(11f64, 11f64, 8f64),
            Vector3::new(29f64, 12f64, 9f64),
        )
        .into();

        let inner_box_4_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(11f64, 28f64, 8f64),
            Vector3::new(29f64, 29f64, 9f64),
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
            0f64,
        );

        let inner_box_mat = super::material::general::General::new(
            1f64,
            0f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 0.7f64, 0.7f64),
            1f64,
            1f64,
            0f64,
        );

        let metal_sphere_mat = super::material::general::General::new(
            0f64,
            1f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1f64,
            0f64,
        );

        let glass_sphere_mat = super::material::general::General::new(
            0f64,
            0f64,
            1f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1.2f64,
            0f64,
        );

        let room_obj = super::object::geometry::GeometryObject::new(room_geo, room_mat);
        let sphere_obj = super::object::geometry::GeometryObject::new(sphere_geo, sphere_mat);
        let box_obj = super::object::geometry::GeometryObject::new(box_geo, box_mat);
        let inner_box_1_obj =
            super::object::geometry::GeometryObject::new(inner_box_1_geo, inner_box_mat.clone());
        let inner_box_2_obj =
            super::object::geometry::GeometryObject::new(inner_box_2_geo, inner_box_mat.clone());
        let inner_box_3_obj =
            super::object::geometry::GeometryObject::new(inner_box_3_geo, inner_box_mat.clone());
        let inner_box_4_obj =
            super::object::geometry::GeometryObject::new(inner_box_4_geo, inner_box_mat.clone());
        let metal_sphere_obj =
            super::object::geometry::GeometryObject::new(metal_sphere_geo, metal_sphere_mat);
        let glass_sphere_obj =
            super::object::geometry::GeometryObject::new(glass_sphere_geo, glass_sphere_mat);

        let objs: Vec<Box<dyn Object>> = vec![
            Box::new(room_obj),
            Box::new(sphere_obj),
            Box::new(box_obj),
            Box::new(inner_box_1_obj),
            Box::new(inner_box_2_obj),
            Box::new(inner_box_3_obj),
            Box::new(inner_box_4_obj),
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

    pub fn focus_scene(args: &Args) -> Scene {
        let light = SemisphereLight::new(
            Point::new(0f64, 50f64, 120f64),
            Color::new(10f64, 10f64, 10f64),
            8f64 * 1024f64,
            Dir::new(0f64, 0f64, -1f64),
        );

        let room_mat = super::material::general::General::new(
            0.6f64,
            0.4f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 1f64, 1f64),
            1f64,
            1f64,
            0.5f64,
        );

        let room_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(-80f64, 0f64, -20f64),
            Vector3::new(20f64, 60f64, 200f64),
        )
        .into();

        let sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(0f64, 10f64, 30f64), 10f64);

        let metal_sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(0f64, 10f64, 0f64), 10f64);

        let glass_sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(0f64, 10f64, 60f64), 10f64);

        let sphere_mat = super::material::general::General::new(
            1f64,
            0f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.5f64, 1f64, 0.5f64),
            1f64,
            1f64,
            0f64,
        );

        let metal_sphere_mat = super::material::general::General::new(
            0f64,
            1f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1f64,
            0f64,
        );

        let glass_sphere_mat = super::material::general::General::new(
            0f64,
            0f64,
            1f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1.4f64,
            0f64,
        );

        let room_obj = super::object::geometry::GeometryObject::new(room_geo, room_mat);
        let sphere_obj = super::object::geometry::GeometryObject::new(sphere_geo, sphere_mat);
        let metal_sphere_obj =
            super::object::geometry::GeometryObject::new(metal_sphere_geo, metal_sphere_mat);
        let glass_sphere_obj =
            super::object::geometry::GeometryObject::new(glass_sphere_geo, glass_sphere_mat);

        let objs: Vec<Box<dyn Object>> = vec![
            Box::new(room_obj),
            Box::new(sphere_obj),
            Box::new(metal_sphere_obj),
            Box::new(glass_sphere_obj),
        ];

        let camera = super::camera::Camera::new(
            Point::new(-60f64, 30f64, 80f64),
            Dir::new(60f64, -20f64, -50f64).normalize(),
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

    pub fn volumetric_scene(args: &Args) -> Scene {
        let room_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(-1e20f64, 0f64, 0f64),
            Vector3::new(1e20f64, 1e20f64, 1e20f64),
        )
        .into();

        let room_mat = super::material::general::General::new(
            1f64,
            0f64, // TODO: why does this even matters?
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.1f64, 0.1f64, 0.1f64),
            1f64,
            1f64,
            0f64,
        );

        let table_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(-100f64, 0f64, 200f64),
            Vector3::new(100f64, 20f64, 300f64),
        )
        .into();

        let table_mat = super::material::general::General::new(
            0.8f64,
            0.2f64, // TODO: why does this even matters?
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 1f64, 1f64),
            1f64,
            1f64,
            0.2f64,
        );

        let water_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(-1e20f64, -10f64, 0f64),
            Vector3::new(1e20f64, 10f64, 1e20f64),
        )
        .into();

        let water_mat = super::material::general::General::new(
            0f64,
            0.6f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 1f64, 1f64),
            1f64,
            1.333f64,
            0f64,
        );

        let base_1_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(0f64, 10f64, 225f64),
            Vector3::new(50f64, 40f64, 275f64),
        )
        .into();

        let base_1_mat = super::material::general::General::new(
            1f64,
            0f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 0.5f64, 0.5f64),
            1f64,
            1f64,
            0f64,
        );

        let glass_1_geo: GeometryGroup<_> = super::object::geometry::util::create_box(
            Vector3::new(0f64, 40f64, 225f64),
            Vector3::new(50f64, 120f64, 275f64),
        )
        .into();

        let glass_mat = super::material::general::General::new(
            0f64,
            0f64,
            0.8f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.5f64, 0.5f64, 1f64),
            1f64,
            1.4f64,
            0f64,
        );

        let metal_mat = super::material::general::General::new(
            0f64,
            1f64,
            0f64,
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0.7f64, 0.7f64, 1f64),
            1f64,
            1f64,
            0f64,
        );


        let sphere_geo =
            super::object::geometry::sphere::Sphere::new(Vector3::new(30f64, 60f64, 250f64), 20f64);

        let room = GeometryObject::new(room_geo, room_mat);
        let table = GeometryObject::new(table_geo, table_mat);
        let water = GeometryObject::new(water_geo, water_mat);
        /*
        let base_1 = GeometryObject::new(base_1_geo, base_1_mat);
        let glass_1 = GeometryObject::new(glass_1_geo, glass_mat.clone());
        */
        let sphere = GeometryObject::new(sphere_geo, metal_mat.clone());

        let objs: Vec<Box<dyn Object>> = vec![
            Box::new(room),
            Box::new(table),
            Box::new(water),
            /*
            Box::new(base_1),
            Box::new(glass_1),
            */
            Box::new(sphere),
        ];

        let camera = super::camera::Camera::new(
            Point::new(-50f64, 60f64, 500f64),
            Dir::new(50f64, 0f64, -250f64).normalize(),
            Dir::new(0f64, 1f64, 0f64),
            args.width,
            args.height,
            50f64 * std::f64::consts::PI / 180f64,
            args.lens_radius,
            args.depth,
        );

        let env_light = SemisphereLight::new(
            Point::new(100f64, 300f64, 500f64),
            Color::new(10f64, 10f64, 10f64),
            64f64 * 1024f64,
            Dir::new(0f64, -1f64, -1f64),
        );

        let beam_light = BeamLight::new(
            Vector3::new(150f64, 200f64, 250f64),
            10f64,
            Dir::new(-1f64, -1f64, 0f64).normalize(),
            Dir::new(0f64, -1f64, 0f64),
            1024f64 * 1024f64,
            Color::new(10f64, 10f64, 10f64),
        );

        let lights: Vec<Box<dyn Light>> = vec![Box::new(env_light), Box::new(beam_light)];

        Scene {
            objs: objs.into(),
            camera,
            lights,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<super::object::Intersect> {
        self.objs.intersect(ray, None)
    }
}
