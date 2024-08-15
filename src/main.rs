use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::environment::new_environment;
use crate::matrix_transformations::rotation_y;
use crate::projectile::new_projectile;
use crate::rays::{hit, intersect, Ray, Sphere};
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
    cast_ray_onto_sphere();
}

fn create_test_image() {
    let width = 500;
    let height = 500;
    let mut canvas = Canvas::new(width, height);
    let red = Color::new(1.0, 0.0, 0.0);
    let blue = Color::new(0.0, 0.0, 1.0);
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
    let width = 500;
    let height = 250;
    let start = Point::new_point(0.0, 0.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0) * 11.25;
    let velocity = velocity.normalize();
    let mut p = new_projectile(start, velocity);
    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(0.01, 0.0, 0.0);
    let mut c = Canvas::new(width, height);
    let env = new_environment(gravity, wind);
    let white = Color::new(1.0, 1.0, 1.0);

    let alpha = 40.0;
    c.write_pixel(
        (p.position.x * alpha) as usize,
        height - 1 - (p.position.y * alpha) as usize,
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
            height - 1 - (p.position.y * alpha) as usize,
            white,
        );
    }
    c.to_ppm("rocket_shot.ppm").expect("while creating ppm");
}

fn analog_clock() {
    let width = 100;
    let height = 100;
    let rad = width as f64 * 0.45;
    let mut c = Canvas::new(width, height);
    let white = Color::new(1.0, 1.0, 1.0);

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

fn cast_ray_onto_sphere() {
    // Canvas setup
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0); // red color

    // Sphere setup
    let shape = Sphere::new();

    // Define the wall dimensions
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    // Ray origin
    let ray_origin = Point::new_point(0.0, 0.0, -5.0);

    // For each row of pixels in the canvas
    for y in 0..canvas_pixels {
        // Compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;

        // For each pixel in the row
        for x in 0..canvas_pixels {
            // Compute the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f64;

            // Describe the point on the wall that the ray will target
            let position = Point::new_point(world_x, world_y, wall_z);

            // Create the ray from the origin to the position on the wall
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());

            // Check for intersection with the sphere
            let xs = intersect(&r, shape);

            // If there's a hit, color the pixel
            if hit(xs).is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    // Save the canvas to a PPM file
    canvas.to_ppm("sphere.ppm").unwrap();
}
