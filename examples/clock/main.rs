use env_logger::Env;
use glam::{Affine3A, Vec3A};
use log::{debug, info};
use raytracer::{canvas::Canvas, color::Color};
use std::f32::consts::PI;

const CANVAS_WIDTH: u16 = 800;
const CANVAS_HEIGHT: u16 = 800;
const CLOCK_RADIUS_PX: u16 = 300;
const PPM_FILE_PATH: &str = "clock.ppm";
const PNG_FILE_PATH: &str = "clock.png";

fn main() {
    // cargo run --release --example clock

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Running clock example");

    if (CLOCK_RADIUS_PX * 2) >= CANVAS_HEIGHT || (CLOCK_RADIUS_PX * 2) >= CANVAS_WIDTH {
        panic!("Clock radius cannot be greater than or equal to canvas height or width");
    }

    let mut canvas = Canvas::new_black_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    let center_point_width = ((CANVAS_WIDTH / 2) - 1) as f32;
    let center_point_height = ((CANVAS_HEIGHT / 2) - 1) as f32;
    let center_point = Vec3A::new(center_point_width, 0.0, center_point_height);

    let twelve = Vec3A::new(0_f32, 0_f32, 1_f32);
    debug!(
        "12 o'clock coordinates, X: {}, Y: {}, Z: {}",
        twelve.x, twelve.y, twelve.z
    );

    let mut clock_hours = vec![twelve];

    for i in 1..12 {
        debug!("Getting coordinates for {i} o'clock");

        // There are 2π radians in a circle. Each hour is rotated 2π/12 (or π/6) radians
        let rotation_y = i as f32 * PI / 6_f32;
        let rotation = Affine3A::from_rotation_y(rotation_y);
        let hour = rotation.transform_point3a(twelve);
        debug!(
            "{i} o'clock coordinates, X: {}, Y: {}, Z: {}",
            hour.x, hour.y, hour.z
        );
        clock_hours.push(hour)
    }

    clock_hours
        .iter()
        .map(|hour| {
            // We will discard y values
            let radius_adjusted_x = hour.x * CLOCK_RADIUS_PX as f32;
            let radius_adjusted_z = hour.z * CLOCK_RADIUS_PX as f32;
            let radius_adjusted_hour = Vec3A::new(radius_adjusted_x, hour.y, radius_adjusted_z);
            radius_adjusted_hour + center_point
        })
        .for_each(|canvas_hour| {
            debug!(
                "Writing pixel to canvas, X: {}, Y: {}",
                canvas_hour.x, canvas_hour.z
            );
            canvas.write_pixel(
                canvas_hour.x.round() as u16,
                canvas_hour.z.round() as u16,
                Color::new_white(),
            )
        });

    canvas.export_as_ppm_and_save_to_file(PPM_FILE_PATH);
    canvas.export_to_desired_format_based_on_extension(PNG_FILE_PATH);
}
