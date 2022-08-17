use std::f64::consts::PI;

use crate::canvas::new_canvas;
use crate::color::new_color;
use crate::environment::new_environment;
use crate::matrix_transformations::rotation_y;
use crate::projectile::new_projectile;
use crate::tuple::{Point, Vector};

mod canvas;
mod color;
mod environment;
mod matrix;
mod matrix_transformations;
mod projectile;
mod rays;
mod tuple;
mod utils;

fn main() {
    analog_clock();
    create_test_image();
    simulate_projectile();
}

fn create_test_image() {
    let width = 500;
    let height = 500;
    let mut canvas = new_canvas(width, height);
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
    canvas.to_ppm("test_ppm.ppm").expect("while creating ppm");
}

fn simulate_projectile() {
    let start = Point::new_point(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0) * 11.25;
    let velocity = velocity.normalize();
    let mut p = new_projectile(start, velocity);
    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(0.01, 0.0, 0.0);
    let mut c = new_canvas(900, 550);
    let env = new_environment(gravity, wind);
    let white = new_color(1.0, 1.0, 1.0);

    let alpha = 40.0;
    c.write_pixel(
        (p.position.x * alpha) as usize,
        (p.position.y * alpha) as usize,
        white,
    );
    while p.position.x >= 0.0 && p.position.y >= 0.0 {
        p = environment::tick(env, p);
        println!(
            "projectile now at:\n\t{}\n\tvelocity {}",
            p.position, p.velocity
        );
        c.write_pixel(
            (p.position.x * alpha) as usize,
            (p.position.y * alpha) as usize,
            white,
        );
    }
    c.to_ppm("rocket_shot.ppm").expect("while creating ppm");
}

fn analog_clock() {
    let width = 100;
    let height = 100;
    let rad = width as f64 * 0.45;
    let mut c = new_canvas(width, height);
    let white = new_color(1.0, 1.0, 1.0);

    let origin = Point::new_point(width as f64 / 2.0_f64, 0.0, height as f64 / 2.0_f64);
    let noon = Point::new_point(0.0, 0.0, 1.0);

    const HOUR: f64 = PI / 6.0_f64;

    for i in 0..12 {
        let r = rotation_y(i as f64 * HOUR);
        let clock_hand = r * noon * rad;
        let final_pos = Point::new_point(origin.x + clock_hand.x, 0.0, origin.z + clock_hand.z);
        c.write_pixel(final_pos.x as usize, final_pos.z as usize, white);
    }

    c.to_ppm("analog_clock.ppm").expect("while creating ppm");
}
