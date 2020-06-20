use super::renderer::*;
use nalgebra::Vector3;

use rand::rngs::ThreadRng;

pub mod general;

/**
 * Both reflection and refrection
 */
pub struct Reflection {
    pub out: Ray,
    pub throughput: Vector3<f64>,
}

pub trait Material : Sync + Send {
    fn is_lambertian(&self) -> bool;
    fn get_lambertian_ratio(&self) -> Vector3<f64>;

    // Specular
    fn get_vision_reflection(&self, at: &Point, inc: &Dir, norm: &Dir, rng: &mut ThreadRng) -> Reflection;

    // Specular
    fn get_photon_reflection(&self, at: &Point, inc: &Dir, norm: &Dir, rng: &mut ThreadRng) -> super::light::Photon;
}
