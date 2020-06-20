use super::renderer::*;

#[derive(Clone, Debug)]
pub struct Photon {
    pub ray: Ray,
    pub flux: Color,
}

pub trait Light: Send + Sync {
    fn emit_photon(&self, total_photon_number: usize) -> Photon;
}

pub struct SemisphereLight {
    at: Point,
    color: Color,
    total_flux: f64,
    towards: Dir,
}

impl SemisphereLight {
    pub fn new(at: Point, color: Color, total_flux: f64, towards: Dir) -> Self {
        Self { at, color, total_flux, towards }
    }
}

impl Light for SemisphereLight {
    fn emit_photon(&self, total_photon_number: usize) -> Photon {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let flux = self.color.clone_owned() * (self.total_flux / total_photon_number as f64);
        let mut dir: Dir = rng.gen();
        dir *= 2f64;
        dir.add_scalar_mut(-1f64);
        dir.normalize_mut();

        /*
        let angle = dir.angle(&self.towards);
        if angle > std::f64::consts::FRAC_PI_2 {
            dir = -dir;
        }
        */

        Photon {
            ray: Ray::new(self.at.clone_owned(), dir),
            flux,
        }
    }
}
