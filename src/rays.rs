use uuid::Uuid;

use crate::tuple::{dot, Tuple, TupleTypeError};
use std::borrow::Borrow;
use std::ops::{Deref, Index};

pub const SPHERE_ORIGIN: Tuple = Tuple {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 1.0,
}; // is a point

#[derive(Debug)]
pub struct Ray {
    origin: Tuple,    // point
    direction: Tuple, // vector
}

pub fn new_ray(origin: Tuple, direction: Tuple) -> Result<Ray, TupleTypeError> {
    if !origin.is_point() {
        return Err(TupleTypeError::new("origin: is not a a point"));
    }
    if !direction.is_vector() {
        return Err(TupleTypeError::new("direction: is not a vector"));
    }
    Ok(Ray { origin, direction })
}

pub fn position(r: &Ray, time: f64) -> Tuple {
    r.origin + r.direction * time
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub id: Uuid,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn new_sphere() -> Sphere {
    Sphere { id: Uuid::new_v4() }
}

#[derive(Debug)]
pub struct Intersection<T> {
    pub t: f64,    // value of intersection
    pub object: T, // object that was intersected
}
impl PartialEq for Intersection<Sphere> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object
    }
}

impl Copy for Intersection<Sphere> {}
impl Clone for Intersection<Sphere> {
    fn clone(&self) -> Self {
        Intersection {
            t: self.t,
            object: self.object,
        }
    }
}

pub fn new_intersection(t: f64, object: Sphere) -> Intersection<Sphere> {
    Intersection { t, object }
}

pub struct Intersections<T> {
    items: Vec<Intersection<T>>,
    count: i64,
}

impl Index<usize> for Intersections<Sphere> {
    type Output = Intersection<Sphere>;
    fn index(&self, index: usize) -> &Self::Output {
        self.items[index].borrow()
    }
}

pub fn intersections(items: Vec<Intersection<Sphere>>, count: i64) -> Intersections<Sphere> {
    Intersections { items, count }
}

pub fn discriminant(r: &Ray) -> f64 {
    let sphere_to_ray = r.origin - SPHERE_ORIGIN;
    let a = dot(r.direction, r.direction).unwrap();
    let b = 2.0 * dot(r.direction, sphere_to_ray).unwrap();
    let c = dot(sphere_to_ray, sphere_to_ray).unwrap() - 1.0;
    b.powf(2.0) - (4.0 * a * c)
}

// returns set of t values, where ray intersects sphere
pub fn intersect(r: &Ray, s: Sphere) -> Intersections<Sphere> {
    let d = discriminant(r);
    if d < 0.0 {
        return intersections(vec![], 0);
    }
    let sphere_to_ray = r.origin - SPHERE_ORIGIN;
    let a = dot(r.direction, r.direction).unwrap();
    let b = 2.0 * dot(r.direction, sphere_to_ray).unwrap();

    let t1 = (-b - d.sqrt()) / (2.0 * a);
    let t2 = (-b + d.sqrt()) / (2.0 * a);

    // TODO: copying here because I dont understand lifetime stuff :/
    let i1 = new_intersection(t1, s.clone());
    let i2 = new_intersection(t2, s.clone());

    if t1 == t2 {
        return intersections(vec![i1, i2], 1);
    }
    intersections(vec![i1, i2], 2)
}

pub fn hit(xs: Intersections<Sphere>) -> Option<Intersection<Sphere>> {
    if xs.count < 1 {
        return None;
    }
    const FLOAT_MAX: f64 = 999_999_999.0;
    let mut smallest = Intersection {
        t: FLOAT_MAX, // TODO: get a proper max
        object: Sphere {
            id: Default::default(),
        },
    };

    for i in xs.items {
        if i.t < smallest.t && i.t >= 0.0 {
            smallest = i;
        }
    }
    if smallest.t == FLOAT_MAX {
        return None;
    }
    Some(smallest)
}

#[cfg(test)]
mod tests {
    use crate::rays::{
        hit, intersect, intersections, new_intersection, new_ray, new_sphere, position,
    };
    use crate::tuple::{new_point, new_vector};
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
        let xs = intersect(r.borrow(), s);
        assert_eq!(xs.count, 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = new_ray(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let xs = intersect(r.borrow(), s);
        assert_eq!(xs.count, 1);
        // assuming two intersections for simplicity
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = new_ray(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let xs = intersect(r.borrow(), s);
        assert_eq!(xs.count, 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = new_ray(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let xs = intersect(r.borrow(), s);
        assert_eq!(xs.count, 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = new_ray(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0)).unwrap();
        let s = new_sphere();
        let xs = intersect(r.borrow(), s);
        assert_eq!(xs.count, 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersection_encapsulates_t_object() {
        let s = new_sphere();
        let i = new_intersection(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = new_sphere();
        let i1 = new_intersection(1.0, s.clone());
        let i2 = new_intersection(2.0, s.clone());

        let xs = intersections(vec![i1, i2], 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
        assert_eq!(xs.count, 2);
    }

    #[test]
    fn hit_all_intersections_positive() {
        let s = new_sphere();
        let i1 = new_intersection(1.0, s.clone());
        let i2 = new_intersection(2.0, s.clone());

        let xs = intersections(vec![i1, i2], 2);
        let i = hit(xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn hit_some_intersections_negative() {
        let s = new_sphere();
        let i1 = new_intersection(-1.0, s);
        let i2 = new_intersection(1.0, s);

        let xs = intersections(vec![i2, i1], 2);
        let i = hit(xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn hit_all_intersections_negative() {
        let s = new_sphere();
        let i1 = new_intersection(-2.0, s);
        let i2 = new_intersection(-1.0, s);

        let xs = intersections(vec![i2, i1], 2);
        // i should be none, implement with option
        let i = hit(xs);
        assert_eq!(i, None);
    }
}
