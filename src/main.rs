use crate::canvas::Canvas;
use crate::color::Color;
use crate::environment::new_environment;
use crate::light::{lighting, Material, PointLight};
use crate::matrix_transformations::rotation_y;
use crate::projectile::new_projectile;
use crate::rays::{hit, intersect, Ray, Sphere};
use crate::tuple::{Point, Vector};
use rayon::prelude::*;
use std::f64::consts::PI;
use std::sync::Mutex;

mod canvas;
mod color;
mod environment;
mod light;
mod matrix;
mod matrix_transformations;
mod projectile;
mod rays;
mod tuple;
mod utils;

fn main() {
    // analog_clock();
    // create_test_image();
    // simulate_projectile();
    // cast_ray_onto_sphere();
    cast_ray_onto_sphere_par();
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
    let canvas_pixels = 400;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut color = Color::new(1.0, 0.0, 0.0); // red color

    let mut shape = Sphere::new();
    shape.set_material(Material::new());
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    // can mess around with various transformations here
    // shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));

    let light_pos = Point::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_pos, light_color);

    let wall_z = 10.0;
    let wall_size = 7.0;

    let ray_origin = Point::new_point(0.0, 0.0, -5.0);

    for y in 0..canvas_pixels {
        for x in 0..canvas_pixels {
            let (world_x, world_y, world_z) =
                compute_world_coordinates(canvas_pixels, wall_size, wall_z, x, y);

            let pos = Point::new_point(world_x, world_y, world_z);
            // create a new ray that originates from the camera (or eye)
            // and points toward a specific position on the wall in the 3D scene
            // The subtraction gives you the direction in which the ray needs to travel to hit
            // the exact point on the wall that corresponds to the pixel you’re currently processing.
            // and make sure to normalize direction
            let r = Ray::new(ray_origin, (pos - ray_origin).normalize());

            // if our ray intersects our shape at this point, color in the canvas
            let xs = intersect(&r, shape);
            if hit(&xs).is_some() {
                let closest_hit = xs[0];
                let point = r.position(closest_hit.t);
                let norm = closest_hit.object.normal_at(point);
                let eye = -r.direction;

                // apply lighting to color
                color = lighting(closest_hit.object.material, light, point, eye, norm);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas.to_ppm("sphere.ppm").unwrap();
}

fn cast_ray_onto_sphere_par() {
    let canvas_pixels = 1000;
    let canvas = Mutex::new(Canvas::new(canvas_pixels, canvas_pixels)); // Wrap the canvas in a Mutex
    let color = Color::new(1.0, 0.0, 0.0); // Initial color (will be updated)

    let mut shape = Sphere::new();
    shape.set_material(Material::new());
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    let light_pos = Point::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_pos, light_color);

    let wall_z = 10.0;
    let wall_size = 7.0;

    let ray_origin = Point::new_point(0.0, 0.0, -5.0);

    (0..canvas_pixels).into_par_iter().for_each(|y| {
        for x in 0..canvas_pixels {
            let (world_x, world_y, world_z) =
                compute_world_coordinates(canvas_pixels, wall_size, wall_z, x, y);

            let pos = Point::new_point(world_x, world_y, world_z);
            let r = Ray::new(ray_origin, (pos - ray_origin).normalize());

            let xs = intersect(&r, shape);
            if let Some(closest_hit) = hit(&xs) {
                let point = r.position(closest_hit.t);
                let norm = closest_hit.object.normal_at(point);
                let eye = -r.direction;

                // Apply lighting to determine color
                let pixel_color = lighting(closest_hit.object.material, light, point, eye, norm);

                // Safely write to the canvas
                canvas.lock().unwrap().write_pixel(x, y, pixel_color);
            }
        }
    });

    canvas.lock().unwrap().to_ppm("sphere2.ppm").unwrap();
}

/// Computes the world coordinates on a 3D wall for a given pixel on a 2D canvas.
///
/// This function converts the 2D pixel coordinates on a canvas to 3D world coordinates on a wall
/// in the scene. The wall is centered on the z-axis, and the function takes into account the size
/// of the canvas and the wall to ensure correct mapping. The wall's z-coordinate remains constant.
///
/// # Parameters
///
/// - `canvas_size`: The size of the canvas (assumed to be square, so both width and height are the same).
/// - `wall_size`: The physical size of the wall in the 3D scene (assumed to be square).
/// - `wall_z`: The z-coordinate of the wall, representing its distance from the ray origin.
/// - `pixel_x`: The x-coordinate of the pixel on the canvas (0 to canvas_size - 1).
/// - `pixel_y`: The y-coordinate of the pixel on the canvas (0 to canvas_size - 1).
///
/// # Returns
///
/// A tuple `(world_x, world_y, wall_z)` representing the 3D world coordinates corresponding to the given pixel.
/// - `world_x`: The x-coordinate in the world space.
/// - `world_y`: The y-coordinate in the world space.
/// - `wall_z`: The z-coordinate (which is the same as the input `wall_z`).
///
/// # Example
///
/// ```rust
/// let canvas_size = 100;
/// let wall_size = 7.0;
/// let wall_z = 10.0;
/// let pixel_x = 50;
/// let pixel_y = 50;
/// let (world_x, world_y, world_z) = compute_world_coordinates(canvas_size, wall_size, wall_z, pixel_x, pixel_y);
/// println!("World coordinates: ({}, {}, {})", world_x, world_y, world_z);
/// ```
///
/// This will output the world coordinates for the center pixel of a 100x100 canvas.
fn compute_world_coordinates(
    canvas_size: usize,
    wall_size: f64,
    wall_z: f64,
    pixel_x: usize,
    pixel_y: usize,
) -> (f64, f64, f64) {
    // Calculate half of the wall size to determine the range of coordinates
    // The wall's x and y coordinates will range from -half to +half
    let half = wall_size / 2.0;

    // Determine the size of each pixel on the canvas in world units
    // This tells us how much space each pixel on the canvas corresponds to on the wall
    let pixel_size = wall_size / canvas_size as f64;

    // Calculate the x-coordinate in the 3D world corresponding to the pixel's x-coordinate on the canvas
    // -half is the leftmost edge of the wall, and we add the offset for the specific pixel
    let world_x = -half + pixel_size * pixel_x as f64;

    // Calculate the y-coordinate in the 3D world corresponding to the pixel's y-coordinate on the canvas
    // half is the topmost edge of the wall, and we subtract the offset for the specific pixel
    let world_y = half - pixel_size * pixel_y as f64;

    (world_x, world_y, wall_z)
}
