use env_logger::Env;
use glam::Vec3A;
use log::info;

use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;

const CANVAS_PIXELS: u16 = 800;
const PPM_FILE_PATH: &str = "circle.ppm";
const PNG_FILE_PATH: &str = "circle.png";

fn main() {
    // cargo run --release --example circle

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Running circle example");

    let wall_z = 10_f32;
    let wall_size = 7_f32;
    let pixel_size = wall_size / CANVAS_PIXELS as f32;
    let middle = wall_size / 2.0;

    let mut canvas = Canvas::new_with_initial_color(
        CANVAS_PIXELS,
        CANVAS_PIXELS,
        Color::new_color(0.2, 0.2, 0.2),
    );
    let ray_origin = Vec3A::new(0.0, 0.0, -5.0);
    let sphere_color = Color::new_red();
    let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));

    for y in 0..CANVAS_PIXELS {
        let world_y = middle - pixel_size * y as f32;

        for x in 0..CANVAS_PIXELS {
            let world_x = -middle + pixel_size * x as f32;

            // describe the point on the wall that the ray will target
            let position = Vec3A::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersection = sphere.intersect(ray);

            if intersection.hit().is_some() {
                canvas.write_pixel(x, y, sphere_color);
            }
        }
    }

    canvas.export_as_ppm_and_save_to_file(PPM_FILE_PATH);
    canvas.export_to_desired_format_based_on_extension(PNG_FILE_PATH);
}
