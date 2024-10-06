use rtlib::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Copy, Clone, Debug)]
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

fn main() -> Result<(), std::io::Error> {
    let mut projectile = Projectile {
        position: Tuple::point_from_underlying(0.0, 1.0, 0.0),
        velocity: Tuple::vector_from_underlying(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let environment = Environment {
        gravity: Tuple::vector_from_underlying(0.0, -0.1, 0.0),
        wind: Tuple::vector_from_underlying(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(Coord2D::new(900, 550));
    let y_max = canvas.get_dim().y - 1;

    while projectile.position.get_y() > Float(0.0) {
        projectile = tick(environment, projectile);
        canvas[Coord2D::new(
            projectile.position.get_x().0 as usize,
            y_max - projectile.position.get_y().0 as usize,
        )] = Color::new_from_underlying(1.0, 0.0, 0.0);
    }

    let file = std::fs::File::create("chapter02.ppm")?;
    let mut buffered_file = std::io::BufWriter::new(file);

    canvas.write_ppm(&mut buffered_file)?;

    let file = std::fs::File::create("chapter02_binary.ppm")?;
    let mut buffered_file = std::io::BufWriter::new(file);

    canvas.write_binary_ppm(&mut buffered_file)
}
