use super::renderer::*;
use nalgebra::*;
use crate::consts::*;
use rand::rngs::ThreadRng;

pub struct Camera {
    origin: super::renderer::Point,

    width: usize,
    height: usize,
    fx: f64,
    fy: f64,
    view: Matrix3<f64>,

    horizontal: Vector3<f64>,
    dir: Vector3<f64>,

    lens_radius: f64,
    depth: f64,
}

impl Camera {
    pub fn new(
        origin: super::renderer::Point,
        dir: Dir,
        up: Dir,
        width: usize,
        height: usize,
        fovy: f64,
        lens_radius: f64,
        depth: f64,
    ) -> Self {
        let dir = dir.normalize();
        let horizontal = dir.cross(&up).normalize();
        let up = horizontal.cross(&dir);
        log::info!("Horizontal: {:?}", horizontal);
        let f = height as f64 / (2f64 * (fovy / 2f64).tan());
        Camera {
            origin,
            width,
            height,
            fx: f,
            fy: f,
            view: Matrix3::from_columns(&[
                horizontal.clone_owned(),
                -up.clone_owned(),
                dir.clone_owned(),
            ]),
            horizontal,
            dir,
            lens_radius,
            depth,
        }
    }

    pub fn generate_ray(&self, x: usize, y: usize, rng: &mut ThreadRng) -> Ray {
        use rand::Rng;
        let xdelta = rng.gen_range(-0.5f64, 0.5f64);
        let ydelta = rng.gen_range(-0.5f64, 0.5f64);

        let csx = (xdelta + x as f64 - self.width as f64 / 2f64) / self.fx;
        let csy = (ydelta + y as f64 - self.height as f64 / 2f64) / self.fy;
        let dir = Vector3::new(csx, csy, 1f64);
        let dir = self.view * dir;

        let shift = if self.lens_radius < EPS {
            Vector3::new(0f64, 0f64, 0f64)
        } else {
            let radius = rng.gen::<f64>().sqrt() * self.lens_radius;
            let theta = rng.gen_range(0f64, std::f64::consts::PI * 2f64);
            Rotation3::new(self.dir * theta) * (self.horizontal * radius)
        };

        let pointing_dir = dir * self.depth - shift;

        Ray::new(self.origin.clone_owned() + shift, pointing_dir.normalize())
    }
}
