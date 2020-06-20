use super::renderer::*;
use rand::rngs::ThreadRng;
use nalgebra::Vector3;
use rand::Rng;
use nalgebra::Rotation3;
use crate::consts::*;

#[derive(Clone, Debug)]
pub struct Photon {
    pub ray: Ray,
    pub flux: Color,
}

pub trait Light: Send + Sync {
    fn emit_photon(&self, total_photon_number: usize, rng: &mut ThreadRng) -> Photon;
}

pub struct SemisphereLight {
    at: Point,
    color: Color,
    total_flux: f64,
    towards: Dir,
}

impl SemisphereLight {
    pub fn new(at: Point, color: Color, total_flux: f64, towards: Dir) -> Self {
        Self { at, color, total_flux, towards: towards.normalize() }
    }
}

impl Light for SemisphereLight {
    fn emit_photon(&self, total_photon_number: usize, rng: &mut ThreadRng) -> Photon {
        let flux = self.color.clone_owned() * (self.total_flux / total_photon_number as f64);
        let mut dir: Dir = rng.gen();
        dir *= 2f64;
        dir.add_scalar_mut(-1f64);
        dir.normalize_mut();

        let angle = dir.angle(&self.towards);
        if angle > std::f64::consts::FRAC_PI_2 {
            dir = -dir;
        }

        Photon {
            ray: Ray::new(self.at.clone_owned(), dir),
            flux,
        }
    }
}

pub struct BeamLight {
    origin: Point,
    radius: f64,
    dir: Dir,
    horizontal: Dir,

    total_flux: f64,
    color: Color,
}

impl BeamLight {
    pub fn new(origin: Point, radius: f64, dir: Dir, reference: Dir, total_flux: f64, color: Color) -> BeamLight {
        BeamLight {
            origin, radius, dir, total_flux, color,
            horizontal: dir.cross(&reference).normalize()
        }
    }
}

impl Light for BeamLight {
    fn emit_photon(&self, total_photon_number: usize, rng: &mut ThreadRng) -> Photon {
        let flux = self.color.clone_owned() * (self.total_flux / total_photon_number as f64);

        let radius = rng.gen::<f64>().sqrt() * self.radius;
        let theta = rng.gen_range(0f64, std::f64::consts::PI * 2f64);
        let shift = Rotation3::new(self.dir * theta) * (self.horizontal * radius);

        Photon {
            ray: Ray::new(self.origin + shift, self.dir),
            flux,
        }
    }
}
