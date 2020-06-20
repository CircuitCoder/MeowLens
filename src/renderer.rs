use super::consts::*;
use super::scene::Scene;
use itertools::iproduct;
use log::*;
use nalgebra::Vector3;
use rand::Rng;
use std::sync::Arc;
use std::borrow::Cow;
use serde::{Deserialize, Serialize};

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

enum Event {
    Halt,
    Process {
        iter: usize,
        radius: f64,
    }
}

#[derive(Serialize, Deserialize)]
struct Checkpoint<'a> {
    iter: usize,
    data: Cow<'a, Vec<Vec<Color>>>,
}

pub fn render(args: super::Args, scene: Scene) {
    let mut buffers: Vec<Vec<Vec<Color>>> = Vec::with_capacity(args.threads);
    for _t in 0..args.threads {
        buffers.push(vec![vec![Default::default(); args.height as usize]; args.width as usize]);
    }

    let cp_cnt = args.iter / args.checkpoint;
    let iter_per_cp_per_thread = args.checkpoint / args.threads;

    use super::light::*;
    use rand::seq::SliceRandom;

    let vol_lambda = 1f64 / args.mean_dist;
    let vol_r = args.volumetric_radius_ratio;
    let vol_r3 = vol_r.powi(3);

    let mut radius = args.radius_0;

    // Main loop
    for cp in 0..cp_cnt {
        info!("Checkpoint: {}", cp);

        let handle = crossbeam_utils::thread::scope(|s| {
            let (dispatcher, consumer) = crossbeam_channel::bounded(args.threads*2);

            {
                // Dispatcher 
                let args = &args;
                s.spawn(move |_| {
                    for local_iter in 0..args.checkpoint {
                        // Update radius
                        radius *= ((local_iter as f64 + args.alpha) / (local_iter + 1) as f64).sqrt();
                        let iter = args.checkpoint * cp + local_iter;
                        info!("[Dispatcher] Dispatching: {}, {}", iter, radius);
                        dispatcher.send(Event::Process { iter, radius }).unwrap();
                    }

                    for _hald_tid in 0..args.threads {
                        dispatcher.send(Event::Halt).unwrap();
                    }

                    drop(dispatcher);
                });
            }

            // Consumer
            for (tid, buf) in buffers.iter_mut().enumerate() {
                let consumer = consumer.clone();
                let args = &args;
                let scene = &scene;
                s.spawn(move |_| {
                    let mut rng = rand::thread_rng();

                    loop {
                        let ev = consumer.recv().unwrap();

                        let (iter, radius) = match ev {
                            Event::Halt => break,
                            Event::Process { iter, radius } => (iter, radius),
                        };

                        let mut kdtree = kdtree::KdTree::new(3);

                        info!("[Worker {}] Iter {}, radius {}", tid, iter, radius);

                        // Photon pass
                        for _pc in 0..args.photon_per_iter {
                            let light: &Box<dyn Light> = scene.lights.as_slice().choose(&mut rng).unwrap();
                            let mut photon: Photon = light.emit_photon(args.photon_per_iter, &mut rng);

                            for _bounce in 0..BOUNCE_HARD_BOUND {
                                // Breaks if photon has no flux
                                if photon.flux.max() <= EPS {
                                    break;
                                }

                                // Find intersect and break if no hit
                                let int = if let Some(r) = scene.intersect(&photon.ray) {
                                    r
                                } else {
                                    // TODO: volumetric light for this portion of photons
                                    break;
                                };

                                // Volumetric lights
                                let vol_dist = rand_distr::Exp::new(vol_lambda).unwrap();
                                let vol_step = rng.sample(vol_dist);
                                if vol_step < int.dist {
                                    let saved = photon.clone();
                                    kdtree.add(
                                        photon.ray.interpolate(vol_step).as_ref().clone(),
                                        saved
                                    ).unwrap();
                                    break;
                                }

                                let material = int.material;

                                if material.is_lambertian() {
                                    let mut saved = photon.clone();
                                    saved
                                        .flux
                                        .component_mul_assign(&material.get_lambertian_ratio());
                                    kdtree.add(photon.ray.interpolate(int.dist).as_ref().clone(), saved).unwrap();
                                }

                                let original_flux = photon.flux;
                                photon = material.get_photon_reflection(
                                    &photon.ray.interpolate(int.dist),
                                    &photon.ray.dir,
                                    &int.norm,
                                    &mut rng,
                                );
                                photon.flux.component_mul_assign(&original_flux);

                                // Russian roulette
                                let avgflux = photon.flux.mean();
                                if rng.gen::<f64>() > avgflux {
                                    // TODO: compensate lost flux
                                    break;
                                }
                            }
                        }

                        info!("[Worker {}] Total recorded photons: {}", tid, kdtree.size());

                        // RT Pass
                        // let radius = args.radius_0 * ((iter as f64 + args.alpha) / (iter + 1) as f64).powf(iter as f64 / 2f64);
                        let radius3 = radius.powi(3);
                        for (x, row) in buf.iter_mut().enumerate() {
                            for (y, pixel) in row.iter_mut().enumerate() {
                                let mut accum: Color = Default::default();

                                for _ss in 0..args.supersampling {
                                    let mut ray = scene.camera.generate_ray(x, y, &mut rng);
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

                                        let mut batch_flux: Color = Default::default();

                                        // Volumetric lights
                                        let vol_dist = rand_distr::Exp::new(vol_lambda).unwrap();
                                        let mut vol_cnt = 0;
                                        let mut traveled = 0f64;
                                        loop {
                                            traveled += rng.sample(vol_dist);
                                            if traveled > int.dist {
                                                break;
                                            }
                                            if vol_cnt > 10 {
                                                break;
                                            }
                                            vol_cnt += 1;

                                            use kdtree::distance::squared_euclidean;
                                            let photons = kdtree
                                                .within(
                                                    ray.interpolate(traveled).as_ref(),
                                                    radius3 * vol_r3,
                                                    &squared_euclidean,
                                                )
                                                .unwrap();

                                            if photons.len() > 0 {
                                                for (dist, photon) in photons {
                                                    let weight = 1f64 - dist / (args.k * radius * vol_r);
                                                    if weight <= EPS {
                                                        continue;
                                                    }
                                                    let inc: Vector3<f64> = photon.flux * weight;
                                                    batch_flux += inc;
                                                }

                                                let batch_flux = batch_flux
                                                    / (1f64 - (3f64 / 4f64) * args.k)
                                                    / (radius3 * vol_r3 * core::f64::consts::PI);

                                                color += batch_flux;
                                            }
                                        }

                                        // Apply material
                                        let material = int.material;

                                        if material.is_lambertian() {
                                            use kdtree::distance::squared_euclidean;
                                            let photons = kdtree
                                                .within(
                                                    ray.interpolate(int.dist).as_ref(),
                                                    radius3,
                                                    &squared_euclidean,
                                                )
                                                .unwrap();

                                            if photons.len() > 0 {
                                                let mut batch_flux: Color = Default::default();

                                                for (dist, photon) in photons {
                                                    let weight = 1f64 - dist / (args.k * radius);
                                                    if weight <= EPS {
                                                        continue;
                                                    }
                                                    let inc: Vector3<f64> = photon.flux
                                                        * (weight
                                                            * ray.dir.angle(&int.norm).cos().abs());
                                                    batch_flux += inc;
                                                }

                                                let batch_flux = batch_flux
                                                    / (1f64 - (2f64 / 3f64) * args.k)
                                                    / (radius * radius * core::f64::consts::PI);

                                                color += batch_flux;
                                            }
                                        }

                                        let reflection = material.get_vision_reflection(
                                            &ray.interpolate(int.dist),
                                            &ray.dir,
                                            &int.norm,
                                            &mut rng,
                                        );
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
                                *pixel += accum / args.supersampling as f64;
                            }
                        }
                    }
                });
            }
        }).unwrap();

        drop(handle);

        // Accumulate results
        let mut pixels: Vec<Vec<Color>> =
            vec![vec![Default::default(); args.height as usize]; args.width as usize];

        for buf in buffers.iter() {
            for (x, col) in buf.iter().enumerate() {
                for (y, elem) in col.iter().enumerate() {
                    pixels[x][y] += elem;
                }
            }
        }

        info!("Saving checkpoint: {}", cp);

        let cps = Checkpoint {
            iter: (cp + 1) * args.checkpoint,
            data: Cow::Borrowed(&pixels),
        };

        let cpf = std::fs::File::create(format!("./checkpoint.{}.json", (cp + 1) * args.checkpoint)).unwrap();
        serde_json::to_writer(cpf, &cps).unwrap();

        result_to_image(&pixels, (cp + 1) * args.checkpoint)
            .save(format!("./output.{}.png", (cp + 1) * args.checkpoint))
            .unwrap();
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

            let color = [r.round() as u8, g.round() as u8, b.round() as u8];
            img.put_pixel(x as u32, y as u32, image::Rgb(color));
        }
    }
    img
}
