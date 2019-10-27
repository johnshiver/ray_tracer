use std::borrow::Borrow;
use std::{thread, time};

mod color;
mod environment;
mod projectile;
mod tuple;

fn main() {
    simulate_projectile();
}

fn simulate_projectile() {
    let gravity = tuple::new_vector(0.0, -9.8, 0.0);
    let wind = tuple::new_vector(-1.0, 0.0, 0.0);
    let env = environment::new_environment(gravity, wind);

    let proj_pos = tuple::new_point(0.0, 0.0, 0.0);
    let proj_vel = tuple::new_vector(10.0, 150.0, 0.0);
    let mut projectile = projectile::new_projectile(proj_pos, proj_vel);

    println!("env + project created, simulation starting...");
    println!(
        "projectile starting at:\n\t{}\n\tvelocity {}",
        projectile.position, projectile.velocity
    );
    for i in 1..101 {
        projectile = environment::tick(env.clone(), projectile);

        println!(
            "projectile now at:\n\t{}\n\tvelocity {}",
            projectile.position, projectile.velocity
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}
