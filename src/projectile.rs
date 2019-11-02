use crate::tuple::Tuple;
use std::fmt::{Display, Formatter, Result};

pub struct Projectile {
    pub position: Tuple, // point
    pub velocity: Tuple, // vector
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "position: {}", self.position)
    }
}

pub fn new_projectile(position: Tuple, velocity: Tuple) -> Projectile {
    Projectile { position, velocity }
}
