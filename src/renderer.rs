use nalgebra::Vector3;
use itertools::iproduct;
use log::*;
use super::consts::*;
use super::scene::Scene;
use rand::Rng;
use std::sync::Arc;

pub type Point = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Dir = Vector3<f64>;
pub type RenderBuffer = Vec<Vec<Color>>;

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Dir,
    pub invdir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point, dir: Dir) -> Ray {
        let dir = dir.normalize();
        Ray {
            origin,
            dir,
            invdir: dir.map(|r| 1f64 / r),
        }
    }

    pub fn interpolate(&self, dist: f64) -> Point {
        self.origin + self.dir * dist
    }
}

pub fn render(args: super::Args, scene: Scene) {
    let mut pixels: Vec<Vec<Color>> = vec![vec![Default::default(); args.height as usize]; args.width as usize];

    /*
    let camera = super::camera::Camera::new(
        Point::new(-60f64, 30f64, 0f64),
        Dir::new(1f64, 0f64, 0f64),
        Dir::new(0f64, 1f64, 0f64), 
        args.width,
        args.height,
        50f64 * std::f64::consts::PI / 180f64,
    );
    */

    /*
    let camera = super::camera::Camera::new(
        Point::new(30f64, 60f64, 0f64),
        Dir::new(0f64, -1f64, 0f64),
        Dir::new(1f64, 0f64, 0f64), 
        args.width,
        args.height,
        50f64 * std::f64::consts::PI / 180f64,
    );
    */

    let camera = super::camera::Camera::new(
        Point::new(-40f64, 50f64, 30f64),
        Dir::new(1f64, -0.5f64, -0.5f64).normalize(),
        Dir::new(0f64, 1f64, 0f64), 
        args.width,
        args.height,
        50f64 * std::f64::consts::PI / 180f64,
    );

    let camera = Arc::new(camera);

    let scene = Arc::new(scene);
    
    use super::light::*;
    use rand::seq::SliceRandom;

    let kdtree = Arc::new(std::sync::RwLock::new(kdtree::KdTree::new(3)));
    let mut radius = args.radius_0;

    // Main loop
    for iter in 0..args.iter {
        info!("Iteration: {}, radius = {}", iter, radius);
        let mut handles = Vec::with_capacity(args.threads);

        // Generate photon map
        for photon_stash in 0..args.threads {
            let scene = scene.clone();
            let kdtree = kdtree.clone();
            let cnt = args.photon_per_iter / args.threads;
            let photon_cnt = args.photon_per_iter;
            let handle= std::thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut stash = Vec::new();
                for _pc in 0..cnt {
                    let light: &Box<dyn Light> = scene.lights.as_slice().choose(&mut rng).unwrap();
                    let mut photon: Photon = light.emit_photon(photon_cnt);

                    for _bounce in 0..BOUNCE_HARD_BOUND {
                        // Breaks if photon has no flux
                        if photon.flux.max() <= EPS {
                            break
                        }

                        // Find intersect and break if no hit
                        let int = if let Some(r) = scene.intersect(&photon.ray) {
                            r
                        } else {
                            break;
                        };

                        let material = int.material;

                        if material.is_lambertian() {
                            let mut saved = photon.clone();
                            saved.flux.component_mul_assign(&material.get_lambertian_ratio());
                            stash.push((photon.ray.interpolate(int.dist).as_ref().clone(), saved));
                        }

                        let original_flux = photon.flux;
                        photon = material.get_photon_reflection(&photon.ray.interpolate(int.dist), &photon.ray.dir, &int.norm);
                        photon.flux.component_mul_assign(&original_flux);

                        // Russian roulette
                        let avgflux = photon.flux.mean();
                        if rng.gen::<f64>() > avgflux {
                            // TODO: compensate lost flux
                            break;
                        }
                    }
                }

                let mut guard = kdtree.write().unwrap();
                for (pt, photon) in stash.drain(..) {
                    guard.add(pt, photon).unwrap();
                }
            });

            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles.drain(..) {
            handle.join().unwrap();
        }

        info!("Total recorded photons: {}", kdtree.read().unwrap().size());
        info!("RT Pass");

        // Generate hitpoints
        // TODO: muiltithreaded
        let rows = pixels.as_mut_slice();
        let mut chunk_size = rows.len() / args.threads;
        if chunk_size * args.threads < rows.len() { chunk_size += 1; }
        let chunks = rows.chunks_mut(chunk_size);
        let mut base = 0;
        crossbeam_utils::thread::scope(|s| {
            let mut handles = Vec::with_capacity(args.threads);
            for chunk in chunks {
                let xbase = base;
                base += chunk.len();

                let kdtree = kdtree.clone();
                let camera = camera.clone();
                let scene = scene.clone();
                let k = args.k;
                let supersampling = args.supersampling;
                let handle = s.spawn(move |_| {
                    let radius3 = radius.powi(3);
                    let guard = kdtree.read().unwrap();
                    let mut rng = rand::thread_rng();
                    for (x, row) in chunk.iter_mut().enumerate() {
                        for (y, pixel) in row.iter_mut().enumerate() {
                            let mut accum: Color = Default::default();

                            for _ss in 0..supersampling {
                                let mut ray = camera.generate_ray(x + xbase, y);
                                let mut throughput = Vector3::new(1f64, 1f64, 1f64);
                                let mut color: Color = Default::default();

                                // debug!("{:#?}", ray);

                                for _bounce in 0..BOUNCE_HARD_BOUND {
                                    let int = if let Some(r) = scene.intersect(&ray) {
                                        r
                                    } else {
                                        // TODO: add background here?
                                        break;
                                    };

                                    // debug!("Found intersection: {:#?}, {:#?}", ray, int);

                                    // Apply material
                                    let material = int.material;

                                    if material.is_lambertian() {
                                        use kdtree::distance::squared_euclidean;
                                        let photons = guard.within(ray.interpolate(int.dist).as_ref(), radius3, &squared_euclidean).unwrap();

                                        if photons.len() > 0 {
                                            if x == 0 && y == 0 && xbase == 0 {
                                                info!("Photon count: {}", photons.len());
                                            }

                                            let mut batch_flux: Color = Default::default();

                                            for (dist, photon) in photons {
                                                let weight = 1f64 - dist / (k * radius);
                                                if weight <= EPS { continue; }
                                                let inc: Vector3<f64> = photon.flux * (weight * ray.dir.angle(&int.norm).cos().abs());
                                                batch_flux += inc;
                                            }

                                            let batch_flux = batch_flux / (1f64 - (2f64 / 3f64) * k) / (radius * radius * core::f64::consts::PI);

                                            color += batch_flux;
                                        }
                                    }

                                    let reflection = material.get_vision_reflection(&ray.interpolate(int.dist), &ray.dir, &int.norm);
                                    ray = reflection.out;
                                    throughput.component_mul_assign(&reflection.throughput);

                                    // Russian roulette
                                    let max_flux = throughput.max();
                                    if rng.gen::<f64>() > max_flux {
                                        // Compensate lost flux
                                        color.component_mul_assign(&throughput.add_scalar(1f64));
                                        
                                        break;
                                    }
                                }

                                accum += color;
                            }

                            // Update color
                            *pixel += accum / supersampling as f64;
                        }
                    }
                });
                handles.push(handle);
            }

            for handle in handles.drain(..) {
                handle.join().unwrap();
            }
        }).unwrap();

        // Update radius
        radius *= ((iter as f64 + args.alpha) / (iter + 1) as f64).sqrt();

        *kdtree.write().unwrap() = kdtree::KdTree::new(3);

        if iter == args.iter - 1 || iter % args.checkpoint == args.checkpoint - 1 {
            result_to_image(&pixels, iter).save(format!("./output.{}.png", iter)).unwrap();
        }
    }
}

fn result_to_image(buffer: &RenderBuffer, rounds: usize) -> image::RgbImage {
    let mut img = image::RgbImage::new(buffer.len() as u32, buffer[0].len() as u32);
    for (x, col) in buffer.iter().enumerate() {
        for (y, elem) in col.iter().enumerate() {
            // Gamma correction
            let r = (elem[0] / rounds as f64).powf(1f64 / 2.2f64) * 255f64;
            let g = (elem[1] / rounds as f64).powf(1f64 / 2.2f64) * 255f64;
            let b = (elem[2] / rounds as f64).powf(1f64 / 2.2f64) * 255f64;

            let color = [
                r.round() as u8,
                g.round() as u8,
                b.round() as u8,
            ];
            img.put_pixel(x as u32, y as u32, image::Rgb(color));
        }
    }
    img
}
