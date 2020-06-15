#![deny(warnings)]

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

    #[structopt(short, long, default_value="16")]
    iter: usize,

    #[structopt(short, long, default_value="262144")]
    photon_per_iter: usize,

    #[structopt(short, long, default_value="5")]
    radius_0: f64,

    #[structopt(short, long, default_value="0.7")]
    alpha: f64,

    #[structopt(short, long, default_value="1.1")]
    k: f64,
}

#[paw::main]
fn main(args: Args) {
    env_logger::init();
    info!("Starting with parameters: {:?}", args);

    let scene = scene::Scene::box_scene();
    renderer::render(args, scene);
}
