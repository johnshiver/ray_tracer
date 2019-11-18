use uuid::Uuid;

use crate::tuple::{Tuple, TupleTypeError, new_point, dot};
pub const SPHERE_ORIGIN: Tuple = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }; // is a point

pub struct Ray {
    origin: Tuple,    // point
    direction: Tuple, // vector
}

pub fn new_ray(origin: Tuple, direction: Tuple) -> Result<Ray, TupleTypeError> {
    if !origin.is_point() {
        return Err(TupleTypeError::new(
            "origin: is not a a point",
        ));
    }
    if !direction.is_vector() {
        return Err(TupleTypeError::new(
            "direction: is not a vector",
        ));
    }
    Ok(Ray{origin, direction})
}

pub fn position(r: &Ray, time: f64) -> Tuple {
    r.origin + r.direction * time
}

pub struct Sphere {
    pub id: Uuid,
}

pub fn new_sphere() -> Sphere {
    Sphere{
        id: Uuid::new_v4()
    }

}

pub struct Intersection {
    count: i64,
    pub positions: [f64; 2],
}

pub fn discriminant(r: &Ray) -> f64  {
    let sphere_to_ray = r.origin - SPHERE_ORIGIN;
    let a = dot(r.direction, r.direction).unwrap();
    let b = 2.0 * dot(r.direction, sphere_to_ray).unwrap();
    let c = dot(sphere_to_ray, sphere_to_ray).unwrap() - 1.0;
    b.powf(2.0) - (4.0 *a*c)
}

// returns set of t values, where ray intersects sphere
pub fn intersect(r: &Ray, s: Sphere) -> ([f64; 2], i16) {
    let d = discriminant(r);
    if d < 0.0 {
        return ([0.0; 2], 0)
    }
    let sphere_to_ray = r.origin - SPHERE_ORIGIN;
    let a = dot(r.direction, r.direction).unwrap();
    let b = 2.0 * dot(r.direction, sphere_to_ray).unwrap();

    let t1 = (-b - d.sqrt()) / (2.0 * a);
    let t2 = (-b + d.sqrt()) / (2.0 * a);

    let mut count = 0;
    if t1 == t2 {
        return ([t1, t2], 1)
    }
    ([t1, t2], 2)
}


#[cfg(test)]
mod tests {
use crate::tuple::{new_point, new_vector};
    use crate::rays::{new_ray, position, new_sphere, intersect};
    use std::borrow::Borrow;

    #[test]
    fn create_ray() {
        let origin = new_point(1.0, 2.0, 3.0);
        let direction = new_vector(4.0, 5.0, 6.0);
        let r = new_ray(origin, direction).unwrap();
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_pt_from_distance() {
        let r = new_ray(new_point(2.0, 3.0, 4.0), new_vector(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(position(r.borrow(), 0.0), new_point(2.0, 3.0, 4.0));
        assert_eq!(position(r.borrow(), 1.0), new_point(3.0, 3.0, 4.0));
        assert_eq!(position(r.borrow(), -1.0), new_point(1.0, 3.0, 4.0));
        assert_eq!(position(r.borrow(), 2.5), new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_two_pts() {
        let r = new_ray(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let (xs, count) = intersect(r.borrow(), s);
        assert_eq!(count, 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = new_ray(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let (xs, count) = intersect(r.borrow(), s);
        assert_eq!(count, 1);
        // assuming two intersections for simplicity
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = new_ray(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let (xs, count) = intersect(r.borrow(), s);
        assert_eq!(count, 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = new_ray(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let (xs, count) = intersect(r.borrow(), s);
        assert_eq!(count, 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = new_ray(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let (xs, count) = intersect(r.borrow(), s);
        assert_eq!(count, 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0], -6.0);
        assert_eq!(xst[1], -4.0);
    }

}