use nalgebra::Vector3;
use itertools::iproduct;
use log::*;
use super::consts::*;
use super::scene::Scene;
use rand::Rng;

pub type Point = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Dir = Vector3<f64>;
pub type RenderBuffer = Vec<Vec<Color>>;

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Dir,
}

impl Ray {
    pub fn new(origin: Point, dir: Dir) -> Ray {
        Ray {
            origin,
            dir: dir.normalize(),
        }
    }

    pub fn interpolate(&self, dist: f64) -> Point {
        self.origin + self.dir * dist
    }
}

pub fn render(args: super::Args, scene: Scene) {
    let mut pixels: Vec<Vec<Color>> = vec![vec![Default::default(); args.height as usize]; args.width as usize];

    let camera = super::camera::Camera::new(
        Point::new(-40f64, 10f64, 0f64),
        Dir::new(1f64, 0f64, 0f64),
        Dir::new(0f64, 1f64, 0f64), 
        args.width,
        args.height,
        50f64 * std::f64::consts::PI / 180f64,
    );
    
    use super::light::*;
    use rand::seq::SliceRandom;

    let mut kdtree = kdtree::KdTree::new(3);
    let mut radius = args.radius_0;

    // Main loop
    for iter in 0..args.iter {
        info!("Iteration: {}", iter);

        let mut rng = rand::thread_rng();

        // Generate photon map
        // TODO: multithreaded
        for _pc in 0..args.photon_per_iter {
            let light: &Light = scene.lights.as_slice().choose(&mut rng).unwrap();
            let mut photon: Photon = light.emit_photon();

            for _bounce in 0..BOUNCE_HARD_BOUND {
                // Breaks if photon has no flux
                if photon.flux.max() <= EPS {
                    break
                }

                // Find intersect and break if no hit
                let (int, obj) = if let Some(r) = scene.intersect(&photon.ray) {
                    r
                } else {
                    break;
                };

                let material = obj.material();

                if material.is_lambertian() {
                    let mut saved = photon.clone();
                    saved.flux.component_mul_assign(&material.get_lambertian_ratio());
                    kdtree.add(photon.ray.interpolate(int.dist).as_ref().clone(), saved).unwrap();
                }

                photon = material.get_photon_reflection(&photon.ray.interpolate(int.dist), &photon.ray.dir, &int.norm);
                // Russian roulette
                let avgflux = photon.flux.mean();
                if rng.gen::<f64>() < avgflux {
                    // TODO: compensate lost flux
                    break;
                }
            }
        }

        // Generate hitpoints
        // TODO: muiltithreaded
        for (x, y) in iproduct!(0..args.width, 0..args.height) {
            // TODO: generate multiple ray
            let mut ray = camera.generate_ray(x, y);
            let mut throughput = Vector3::new(1f64, 1f64, 1f64);
            let mut color: Color = Default::default();

            // debug!("{:#?}", ray);

            for _bounce in 0..BOUNCE_HARD_BOUND {
                let (int, obj) = if let Some(r) = scene.intersect(&ray) {
                    r
                } else {
                    // TODO: add background here?
                    break;
                };

                // debug!("Found intersection: {:#?}, {:#?}", ray, int);

                // Apply material
                let material = obj.material();

                if material.is_lambertian() {
                    use kdtree::distance::squared_euclidean;
                    let photons = kdtree.within(ray.interpolate(int.dist).as_ref(), radius, &squared_euclidean).unwrap();

                    if photons.len() > 0 {
                        let mut batch_flux: Color = Default::default();
                        let len = photons.len();

                        for (dist, photon) in photons {
                            let weight = 1f64 - dist / (args.k * radius);
                            if weight <= EPS { continue; }
                            batch_flux += photon.flux * weight * ray.dir.angle(&int.norm).cos();
                        }

                        let batch_flux = batch_flux / (1f64 - (2f64 / 3f64) * args.k) / (radius * radius * core::f64::consts::PI);
                        color += batch_flux;
                    }
                }

                let reflection = material.get_vision_reflection(&ray.interpolate(int.dist), &ray.dir, &int.norm);
                ray = reflection.out;
                throughput.component_mul_assign(&reflection.throughput);

                // Russian roulette
                let avgflux = throughput.mean();
                if rng.gen::<f64>() < avgflux {
                    // Compensate lost flux
                    color.component_mul_assign(&throughput.add_scalar(1f64));
                    
                    break;
                }
            }

            // Update color
            pixels[x as usize][y as usize] += color;
        }

        // Update radius
        radius *= ((iter as f64 + args.alpha) / (iter + 1) as f64).sqrt();

        kdtree = kdtree::KdTree::new(3);

        result_to_image(&pixels, iter).save(format!("./output-{}.png", iter)).unwrap();
    }
}

fn result_to_image(buffer: &RenderBuffer, rounds: usize) -> image::RgbImage {
    let mut img = image::RgbImage::new(buffer.len() as u32, buffer[0].len() as u32);
    for (x, col) in buffer.iter().enumerate() {
        for (y, elem) in col.iter().enumerate() {
            let color = elem * (255f64 / rounds as f64);
            let color = [
                color[0].round() as u8,
                color[1].round() as u8,
                color[2].round() as u8,
            ];
            img.put_pixel(x as u32, y as u32, image::Rgb(color));
        }
    }
    img
}
