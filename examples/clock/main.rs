use env_logger::Env;
use glam::{Vec3A, Vec4};
use log::{debug, info};
use raytracer::{canvas::Canvas, color::Color, tuple::Tuple};

const CANVAS_WIDTH: u16 = 1200;
const CANVAS_HEIGHT: u16 = 1200;
const PPM_FILE_PATH: &str = "clock.ppm";
const PNG_FILE_PATH: &str = "clock.png";

fn main() {
    // cargo run --release --example clock

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Running clock example");

    let mut canvas =
        Canvas::new_with_initial_color(CANVAS_WIDTH, CANVAS_HEIGHT, Vec3A::new_white());
    let starting_point_x = (CANVAS_WIDTH / 2) - 1; // 0 indexed
    let starting_point_y = (CANVAS_HEIGHT / 2) - 1; // 0 indexed
    let starting_point =
        Vec4::new_point_tuple(starting_point_x as f32, starting_point_y as f32, 0.0);

    canvas.write_pixel(starting_point_x, starting_point_y, Vec3A::new_black());
    canvas.export_as_ppm_and_save_to_file(PPM_FILE_PATH);
    canvas.export_to_desired_format_based_on_extension(PNG_FILE_PATH);
}
