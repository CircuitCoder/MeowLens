use super::renderer::*;

pub struct Light {
    at: Point,
    color: Color,
}

#[derive(Clone, Debug)]
pub struct Photon {
    pub ray: Ray,
    pub flux: Color,
}

impl Light {
    pub fn new(at: Point, color: Color) -> Light {
        Light { at, color }
    }

    pub fn emit_photon(&self) -> Photon {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let flux = self.color.clone_owned();
        let mut dir: Dir = rng.gen();
        dir *= 2f64;
        dir.add_scalar_mut(-1f64);
        dir.normalize_mut();

        Photon {
            ray: Ray::new(self.at.clone_owned(), dir),
            flux,
        }
    }
}
