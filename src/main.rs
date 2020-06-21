// #![deny(warnings)]

mod renderer;
mod object;
mod material;
mod camera;
mod light;
mod consts;
mod scene;

use structopt::StructOpt;
use log::info;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short, long, default_value="720")]
    width: usize,

    #[structopt(short, long, default_value="480")]
    height: usize,

    #[structopt(short, long, default_value="128")]
    iter: usize,

    #[structopt(short, long, default_value="1048576")]
    photon_per_iter: usize,

    #[structopt(short, long, default_value="1")]
    radius_0: f64,

    #[structopt(short, long, default_value="0.7")]
    alpha: f64,

    #[structopt(short, long, default_value="1.1")]
    k: f64,

    #[structopt(short, long, default_value="16")]
    threads: usize,

    // TODO: impl
    #[structopt(short, long, default_value="1")]
    supersampling: usize,

    #[structopt(short, long, default_value="16")]
    checkpoint: usize,

    #[structopt(short, long, default_value="2")]
    lens_radius: f64,

    #[structopt(short, long, default_value="20")]
    depth: f64,

    #[structopt(short, long, default_value="1000")]
    mean_dist: f64,

    #[structopt(short, long, default_value="2")]
    volumetric_radius_ratio: f64,
}

#[paw::main]
fn main(args: Args) {
    env_logger::init();
    info!("Starting with parameters: {:?}", args);

    let scene = scene::Scene::focus_scene(&args);
    renderer::render(args, scene);
}
