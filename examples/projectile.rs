use raytracer::tuple::Tuple;

#[derive(Debug)]
struct Environment {
    gravity: Tuple, // A vector
    wind: Tuple,    // A vector
}

#[derive(Debug)]
struct Projectile {
    position: Tuple, // A point
    velocity: Tuple, // A vector
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
    let mut projectile = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    };

    let environment = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        println!("{:#?}", projectile);
        projectile = tick(&environment, &projectile);
    }
}
