use crate::consts::*;
use crate::renderer::*;
use nalgebra::*;
use crate::renderer::Point;

#[derive(Clone)]
pub struct General {
    diffusion_ratio: f64,
    pure_reflection_ratio: f64,
    refraction_ratio: f64,

    refraction_throughput: Vector3<f64>,
    diffusion_throughput: Vector3<f64>,

    specular_ratio: f64,
    nratio: f64,
    r0: f64,
}

impl General {
    pub fn new(
        diffusion_ratio: f64,
        pure_reflection_ratio: f64,
        refraction_ratio: f64,
        refraction_throughput: Vector3<f64>,
        diffusion_throughput: Vector3<f64>,
        n1: f64,
        n2: f64,
    ) -> Self {
        Self {
            diffusion_ratio, pure_reflection_ratio, refraction_ratio,
            refraction_throughput, diffusion_throughput,

            specular_ratio: refraction_ratio + pure_reflection_ratio,

            nratio: n1 / n2,
            r0: ((n1 - n2) / (n1 + n2)).powi(2),
        }
    }

    fn generate_reflection_ray(&self, at: &Point, inc: &Dir, norm: &Dir, _theta_i: f64) -> super::Reflection {
        let inc = -inc;
        let projected = inc.dot(norm);
        let scaled = norm * projected;
        let out = inc + (scaled - inc) * 2f64;

        super::Reflection {
            out: Ray::new(at.clone(), out.clone()),
            throughput: self.specular_ratio * self.refraction_throughput
        }
    }

    fn generate_refraction_ray(&self, at: &Point, inc: &Dir, norm: &Dir, theta_i: f64) -> super::Reflection {
        let theta_t_sin = if theta_i > std::f64::consts::PI / 2f64 { // Outgoing
            theta_i.sin() / self.nratio
        } else {
            theta_i.sin() * self.nratio
        };
        let theta_t = theta_t_sin.asin();

        assert!(!theta_t.is_nan());

        let rotation_axis = inc.cross(norm);
        let rotation_angle = theta_t - theta_i;
        let rotation = Rotation3::new(rotation_axis * rotation_angle);
        let out = rotation * inc;

        super::Reflection {
            out: Ray::new(at.clone(), out.clone()),
            throughput: self.specular_ratio * self.refraction_throughput
        }
    }
}

impl super::Material for General {
    fn is_lambertian(&self) -> bool {
        self.diffusion_ratio > EPS
    }
    fn get_lambertian_ratio(&self) -> Vector3<f64> {
        self.diffusion_throughput.clone_owned() * self.diffusion_ratio
    }

    // Normal is n2 -> n1
    fn get_vision_reflection(&self, at: &Point, inc: &Dir, norm: &Dir) -> super::Reflection {
        let theta_i: f64 = inc.angle(&-norm).into();
        let reflection_coeff = self.r0 + (1f64 - self.r0) * (1f64 - theta_i.cos()).powi(5);
        let reflected_ratio = reflection_coeff * self.refraction_ratio + self.pure_reflection_ratio;

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let is_reflection = rng.gen_bool(reflected_ratio / self.specular_ratio);

        if is_reflection {
            self.generate_reflection_ray(at, inc, norm, theta_i)
        } else {
            self.generate_refraction_ray(at, inc, norm, theta_i)
        }
    }

    fn get_photon_reflection(&self, at: &Point, inc: &Dir, norm: &Dir) -> crate::light::Photon {
        // Identical to vision reflection
        let reflection = self.get_vision_reflection(at, inc, norm);

        crate::light::Photon {
            ray: reflection.out,
            flux: reflection.throughput,
        }
    }
}
