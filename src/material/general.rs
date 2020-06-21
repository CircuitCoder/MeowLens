use crate::consts::*;
use crate::renderer::*;
use nalgebra::*;
use crate::renderer::Point;
use rand::rngs::ThreadRng;
use rand_distr::Normal;
use rand::Rng;

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

    glossy_dist: Option<Normal<f64>>,
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
        glossy_stddev: f64,
    ) -> Self {
        let dist = if glossy_stddev < EPS {
            None
        } else {
            Some(Normal::new(0f64, glossy_stddev * glossy_stddev).unwrap())
        };

        Self {
            diffusion_ratio, pure_reflection_ratio, refraction_ratio,
            refraction_throughput, diffusion_throughput,

            specular_ratio: refraction_ratio + pure_reflection_ratio,

            nratio: n1 / n2,
            r0: ((n1 - n2) / (n1 + n2)).powi(2),

            glossy_dist: dist,
        }
    }

    fn generate_reflection_ray(&self, at: &Point, inc: &Dir, norm: &Dir, _theta_i: f64, rng : &mut ThreadRng) -> super::Reflection {
        let inc = -inc;
        let projected = inc.dot(norm);
        let mut scaled = norm.clone();
        scaled.set_magnitude(projected);
        let out = (scaled * 2f64 - inc).normalize();

        let out = if let Some(dist) = self.glossy_dist {
            let dx = rng.sample(dist);
            let dy = rng.sample(dist);
            let dz = rng.sample(dist);

            let d = Vector3::new(dx, dy, dz);
            let pd = d - d.dot(&out) * out;
            (out + pd).normalize()
        } else {
            out
        };

        /*
        if out.normalize()[0].is_nan() {
            log::error!("Incorrect norm: {}", out);
        }
        */

        super::Reflection {
            out: Ray::new(at.clone(), out.clone()),
            throughput: self.specular_ratio * self.refraction_throughput
        }
    }

    fn generate_refraction_ray(&self, at: &Point, inc: &Dir, norm: &Dir, theta_i: f64, rng: &mut ThreadRng) -> super::Reflection {
        let (theta_t_sin, starting_norm, negate) = if theta_i < std::f64::consts::PI / 2f64 { // Outgoing
            (theta_i.sin() / self.nratio, -norm, false)
        } else {
            (theta_i.sin() * self.nratio, norm.clone(), true)
        };

        let theta_t = theta_t_sin.asin();

        /*
        if (at[2] + 10f64).abs() < EPS && at[1] > EPS {
            log::info!("On left face, {} = {} -> {}", theta_i, theta_i.sin(), theta_t_sin);
        }
        */

        if theta_t.is_nan() {
            log::debug!("Forced reflection: {} -> {}", theta_i.sin(), theta_t_sin);
            /*
            return super::Reflection {
                out: Ray::new(at.clone(), -inc),
                throughput: Vector3::new(0f64, 0f64, 0f64),
            };
            */
            return self.generate_reflection_ray(at, inc, norm, theta_i, rng);
        }

        if theta_t <= 0f64 {
            log::error!("Unexpected theta_t {} from sin {}", theta_t, theta_t_sin);
            loop {}
        }

        let angle = if negate { -theta_t } else { theta_t };
        let out = if angle.abs() < EPS {
            starting_norm
        } else {
            let rotation_axis = nalgebra::base::Unit::new_normalize(inc.cross(norm));
            let rotation = Rotation3::from_axis_angle(&rotation_axis, angle);
            let result = rotation * starting_norm;

            /*
            let cross = inc.cross(&result);
            let diff = cross.normalize() - rotation_axis.normalize();
            let diff = diff.norm();

            if !(diff < EPS || (diff - 2f64).abs() < EPS) && (inc - result).norm() > EPS {
                log::error!("Unexpected diff: {}, {}, {}, {}, {}, {}", diff, cross.normalize(), rotation_axis.normalize(), inc, norm, result);
                loop {}
            }
            */

            result
        };
        
        /*
        if theta_i < std::f64::consts::PI / 2f64 {
            log::debug!("Outgoing!");
        } else {
            log::debug!("Incoming!");
        }
        log::debug!("Refraction, inc: {:?}, starting_norm: {:?}, out: {:?}", inc, starting_norm, out);
        */

        /*
        if out.normalize()[0].is_nan() {
            log::error!("Incorrect norm: {}", out);
        }
        */

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
    fn get_vision_reflection(&self, at: &Point, inc: &Dir, norm: &Dir, rng: &mut ThreadRng) -> super::Reflection {
        let theta_i: f64 = inc.angle(&-norm).into();
        // let reflection_coeff = self.r0 + (1f64 - self.r0) * (1f64 - theta_i.cos().abs()).powi(5);
        let reflection_coeff = 0f64;
        let reflected_ratio = reflection_coeff * self.refraction_ratio + self.pure_reflection_ratio;

        if self.specular_ratio < EPS {
            return super::Reflection {
                out: Ray::new(at.clone(), -inc),
                throughput: Vector3::new(0f64, 0f64, 0f64),
            }
        }

        // log::debug!("{} / {}", reflected_ratio, self.specular_ratio);
        let is_reflection = rng.gen_bool(reflected_ratio / self.specular_ratio);

        if is_reflection {
            self.generate_reflection_ray(at, inc, norm, theta_i, rng)
        } else {
            self.generate_refraction_ray(at, inc, norm, theta_i, rng)
        }
    }

    fn get_photon_reflection(&self, at: &Point, inc: &Dir, norm: &Dir, rng: &mut ThreadRng) -> crate::light::Photon {
        // Identical to vision reflection
        let reflection = self.get_vision_reflection(at, inc, norm, rng);

        crate::light::Photon {
            ray: reflection.out,
            flux: reflection.throughput,
        }
    }
}
