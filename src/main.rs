use crate::canvas::new;
use crate::color::new_color;
use std::{thread, time};

mod canvas;
mod color;
mod environment;
mod projectile;
mod tuple;

fn main() {
    simulate_projectile();
    create_test_image();
}

fn create_test_image() {
    let width = 500;
    let height = 500;
    let mut canvas = new(width, height);
    let red = new_color(1.0, 0.0, 0.0);
    let blue = new_color(0.0, 0.0, 1.0);
    for x in 0..width {
        canvas.write_pixel(x, 0, red);
        canvas.write_pixel(x, 1, red);
        canvas.write_pixel(x, 2, red);

    }
    for y in 1..height {
        canvas.write_pixel(0, y, blue);
    }
    canvas.to_ppm("test_ppm.ppm");
}

fn simulate_projectile() {
    let gravity = tuple::new_vector(0.0, -1.0, 0.0);
    let wind = tuple::new_vector(0.0, 0.0, 0.0);
    let env = environment::new_environment(gravity, wind);
    let mut canvas = canvas::new(200, 200);
    let red = color::new_color(1.0, 0.0, 0.0);

    let proj_pos = tuple::new_point(0.0, 0.0, 0.0);
    let proj_vel = tuple::new_vector(10.0, 10.0, 0.0);
    let mut projectile = projectile::new_projectile(proj_pos, proj_vel);

    println!("env + project created, simulation starting...");
    println!(
        "projectile starting at:\n\t{}\n\tvelocity {}",
        projectile.position, projectile.velocity
    );

    canvas.write_pixel(projectile.position.x as usize, projectile.position.y as usize, red);
    while projectile.position.x >= 0.0 && projectile.position.y >= 0.0 {
        projectile = environment::tick(env, projectile);
        println!(
            "projectile now at:\n\t{}\n\tvelocity {}",
            projectile.position, projectile.velocity
        );
        canvas.write_pixel(projectile.position.x as usize, projectile.position.y as usize, red);
        thread::sleep(time::Duration::from_millis(1));
    }
    canvas.to_ppm("rocket_shot.ppm");
}
