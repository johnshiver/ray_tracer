use crate::projectile::{new_projectile, Projectile};
use crate::tuple::Tuple;

#[derive(Copy, Clone)]
pub struct Environment {
    gravity: Tuple, // vector
    wind: Tuple,    // vector
}

pub fn new_environment(gravity: Tuple, wind: Tuple) -> Environment {
    Environment { gravity, wind }
}

pub fn tick(env: Environment, projectile: Projectile) -> Projectile {
    let new_pos = projectile.position + projectile.velocity;
    let new_vel = projectile.velocity + env.gravity + env.wind;
    return new_projectile(new_pos, new_vel);
}
