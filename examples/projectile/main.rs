use env_logger::Env;
use glam::Vec4;
use log::{debug, info};
use raytracer::{canvas::Canvas, color::Color, tuple::Tuple};

const CANVAS_WIDTH: u16 = 900;
const CANVAS_HEIGHT: u16 = 550;
const PPM_FILE_PATH: &str = "projectile.ppm";
const PNG_FILE_PATH: &str = "projectile.png";

#[derive(Debug)]
struct Environment {
    gravity: Vec4, // A vector
    wind: Vec4,    // A vector
}

#[derive(Debug)]
struct Projectile {
    position: Vec4, // A point
    velocity: Vec4, // A vector
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let new_position = proj.position + proj.velocity;
    let new_velocity = proj.velocity + env.gravity + env.wind;
    Projectile {
        position: new_position,
        velocity: new_velocity,
    }
}

fn main() {
    // cargo run --release --example projectile

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Running projectile example");

    let mut canvas = Canvas::new_black_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    let mut projectile = Projectile {
        position: Vec4::new_point_tuple(0.0, 1.0, 0.0),
        velocity: Vec4::new_vector_tuple(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let environment = Environment {
        gravity: Vec4::new_vector_tuple(0.0, -0.1, 0.0),
        wind: Vec4::new_vector_tuple(-0.01, 0.0, 0.0),
    };

    // Canvas coordinates are 0 indexed (so for 550 height we use 0-549 inclusive)
    // Start point
    debug!("Writing starting point on canvas");
    let mut proj_x = projectile
        .position
        .x
        .round()
        .clamp(0.0, (CANVAS_WIDTH - 1) as f32) as u16;

    // Y coordinate is upside-down on canvas
    let mut proj_y = projectile
        .position
        .y
        .round()
        .clamp(1.0, CANVAS_HEIGHT as f32) as u16;
    let mut proj_y_flipped = CANVAS_HEIGHT - proj_y;

    canvas.write_pixel(proj_x, proj_y_flipped, Color::new_red());

    debug!("Writing subsequent points on canvas");
    while projectile.position.y > 0.0 {
        projectile = tick(&environment, &projectile);
        proj_x = projectile
            .position
            .x
            .round()
            .clamp(0.0, (CANVAS_WIDTH - 1) as f32) as u16;
        proj_y = projectile
            .position
            .y
            .round()
            .clamp(1.0, CANVAS_HEIGHT as f32) as u16;
        proj_y_flipped = CANVAS_HEIGHT - proj_y;

        canvas.write_pixel(proj_x, proj_y_flipped, Color::new_green());
    }

    canvas.export_as_ppm_and_save_to_file(PPM_FILE_PATH);
    canvas.export_to_desired_format_based_on_extension(PNG_FILE_PATH);
}
