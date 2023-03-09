use crate::tuple::{Point, Tuple, Vector};
use std::fmt::{Display, Formatter, Result};

pub struct Projectile {
    pub position: Point,  // point
    pub velocity: Vector, // vector
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "position: {}", self.position)
    }
}

pub fn new_projectile(position: Point, velocity: Vector) -> Projectile {
    Projectile { position, velocity }
}
