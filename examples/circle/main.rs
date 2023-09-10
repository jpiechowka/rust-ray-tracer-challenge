use env_logger::Env;
use log::info;

const CANVAS_WIDTH: u16 = 800;
const CANVAS_HEIGHT: u16 = 800;
const PPM_FILE_PATH: &str = "circle.ppm";
const PNG_FILE_PATH: &str = "circle.png";

fn main() {
    // cargo run --release --example circle

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Running circle example");
}
