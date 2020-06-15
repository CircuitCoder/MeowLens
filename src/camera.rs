use super::renderer::*;
use nalgebra::*;

pub struct Camera {
    origin: super::renderer::Point,

    width: usize,
    height: usize,
    fx: f64,
    fy: f64,
    view: Matrix3<f64>,
}

impl Camera {
    pub fn new(origin: super::renderer::Point, dir: Dir, up: Dir, width: usize, height: usize, fovy: f64) -> Self {
        let dir = dir.normalize();
        let up = up.normalize();
        let horizontal = dir.cross(&up);
        log::info!("Horizontal: {:?}", horizontal);
        let f = height as f64 / (2f64 * (fovy / 2f64).tan());
        Camera {
            origin,
            width,
            height,
            fx: f,
            fy: f,
            view: Matrix3::from_columns(&[horizontal.clone_owned(), -up.clone_owned(), dir.clone_owned()])
        }
    }

    pub fn generate_ray(&self, x: usize, y: usize) -> Ray {
        // TODO: subpixel supersampling
        let csx = (x as f64 - self.width as f64 / 2f64) / self.fx;
        let csy = (y as f64 - self.height as f64 / 2f64) / self.fy;
        let dir = Vector3::new(csx, csy, 1f64);
        let dir = self.view * dir;

        Ray::new(self.origin.clone_owned(), dir)
    }
}
