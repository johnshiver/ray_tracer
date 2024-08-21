use std::fmt::{Display, Formatter, Result};

use crate::tuple::{Point, Vector};

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "position: {}", self.position)
    }
}

pub fn new_projectile(position: Point, velocity: Vector) -> Projectile {
    Projectile { position, velocity }
}
