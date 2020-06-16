use super::renderer::*;
use nalgebra::Vector3;

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
    fn get_vision_reflection(&self, at: &Point, inc: &Dir, norm: &Dir) -> Reflection;

    // Specular
    fn get_photon_reflection(&self, at: &Point, inc: &Dir, norm: &Dir) -> super::light::Photon;
}
