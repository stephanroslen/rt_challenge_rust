use rtlib::prelude::*;

#[derive(Copy,Clone,Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Copy,Clone,Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

fn main() {
    let mut projectile = Projectile{position: Tuple::point_from_underlying(0.0, 1.0, 0.0), velocity: Tuple::vector_from_underlying(1.0, 1.0, 0.0).normalize()};
    let environment = Environment{gravity: Tuple::vector_from_underlying(0.0, -0.1, 0.0), wind: Tuple::vector_from_underlying(-0.01, 0.0, 0.0)};

    while projectile.position.get_y() > Float(0.0) {
        projectile = tick(environment, projectile);
    }
    println!("projectile: {}", projectile.position);
    println!("velocity: {}", projectile.velocity);
}
