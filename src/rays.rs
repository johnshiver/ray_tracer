use std::borrow::Borrow;
use std::ops::Index;

use uuid::Uuid;

use crate::tuple::{Point, Tuple, Vector};

pub const SPHERE_ORIGIN: Tuple = Tuple {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 1.0,
}; // is a point

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    pub fn discriminant(&self) -> f64 {
        let sphere_to_ray = self.origin - SPHERE_ORIGIN;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        b.powf(2.0) - (4.0 * a * c)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub id: Uuid,
}

impl Sphere {
    // Factory method to create a new Intersection
    pub fn new() -> Self {
        Sphere { id: Uuid::new_v4() }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
pub struct Intersection<T> {
    pub t: f64,
    // value of intersection
    pub object: T, // object that was intersected
}

impl<T> Intersection<T> {
    // Factory method to create a new Intersection
    pub fn new(t: f64, object: T) -> Self {
        Intersection { t, object }
    }
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

pub struct Intersections<T> {
    items: Vec<Intersection<T>>,
}

impl<T> Intersections<T> {
    fn size(&self) -> usize {
        self.items.len()
    }
}

impl Index<usize> for Intersections<Sphere> {
    type Output = Intersection<Sphere>;
    fn index(&self, index: usize) -> &Self::Output {
        self.items[index].borrow()
    }
}

impl<T> From<Vec<Intersection<T>>> for Intersections<T> {
    fn from(items: Vec<Intersection<T>>) -> Self {
        Intersections { items }
    }
}

/// Computes the intersection points between a ray and a sphere.
///
/// This function calculates the intersection points, if any, between a ray and a sphere
/// using the quadratic formula. The ray is defined by its origin and direction, and the sphere
/// is assumed to be centered at `SPHERE_ORIGIN` with a radius of 1.0.
///
/// The quadratic equation used is derived from the formula for a sphere and a parametric
/// equation for a ray:
///
/// - Sphere equation: `(x - cx)^2 + (y - cy)^2 + (z - cz)^2 = r^2`
/// - Ray equation: `P(t) = O + tD`, where `O` is the origin, `D` is the direction, and `t` is the parameter
///
/// By substituting the ray equation into the sphere equation and rearranging terms,
/// we get a quadratic equation of the form `at^2 + bt + c = 0`, where:
///
/// - `a` is the dot product of the direction vector with itself.
/// - `b` is 2 times the dot product of the direction vector and the vector from the sphere's center to the ray's origin.
/// - `c` is the dot product of the vector from the sphere's center to the ray's origin with itself, minus the radius squared (1.0 in this case).
///
/// The discriminant `d = b^2 - 4ac` determines the nature of the intersection:
///
/// - If `d < 0`, the ray does not intersect the sphere.
/// - If `d = 0`, the ray touches the sphere at exactly one point (tangent).
/// - If `d > 0`, the ray intersects the sphere at two points (entering and exiting).
///
/// The function returns an `Intersections<Sphere>` object containing the `t` values where the intersections occur.
///
/// # Arguments
///
/// * `r` - A reference to the `Ray` that might intersect the sphere.
/// * `s` - The `Sphere` that the ray might intersect.
///
/// # Returns
///
/// An `Intersections<Sphere>` object containing the intersection points, if any.
pub fn intersect(r: &Ray, s: Sphere) -> Intersections<Sphere> {
    // Calculate the discriminant, which determines the number of intersection points
    let d = r.discriminant();

    // If the discriminant is negative, there are no real intersections (ray misses the sphere)
    if d < 0.0 {
        return Intersections::from(vec![]); // Return an empty list of intersections
    }

    // Vector from the sphere's origin (assumed to be the origin in this case) to the ray's origin
    let sphere_to_ray = r.origin - SPHERE_ORIGIN;

    // Calculate the coefficients of the quadratic equation
    let a = r.direction.dot(&r.direction); // Coefficient 'a' (direction vector dot product with itself)
    let b = 2.0 * r.direction.dot(&sphere_to_ray); // Coefficient 'b' (2 times direction dot product with sphere_to_ray vector)

    // The discriminant is zero, meaning the ray is tangent to the sphere.
    // This results in exactly one intersection point (the ray just touches the sphere).
    if d == 0.0 {
        let t = -b / (2.0 * a); // Calculate the single intersection point
        let i = Intersection::new(t, s.clone()); // Create the Intersection object for this point
        return Intersections::from(vec![i, i]); // Return the single intersection as a list with two elements, but they are the same
    }

    // Calculate the two possible values of t (parameter along the ray) where intersections occur
    let t1 = (-b - d.sqrt()) / (2.0 * a); // First intersection point (entering the sphere)
    let t2 = (-b + d.sqrt()) / (2.0 * a); // Second intersection point (exiting the sphere)

    // Create Intersection objects for each intersection point with the sphere
    let i1 = Intersection::new(t1, s.clone()); // Intersection at t1
    let i2 = Intersection::new(t2, s.clone()); // Intersection at t2

    // Return a list of the intersections
    Intersections::from(vec![i1, i2])
}

pub fn hit(xs: Intersections<Sphere>) -> Option<Intersection<Sphere>> {
    if xs.size() < 1 {
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
    use crate::rays::{hit, intersect, Intersection, Intersections, Ray, Sphere};
    use crate::tuple::{Point, Vector};

    #[test]
    fn create_ray() {
        let origin = Point::new_point(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_pt_from_distance() {
        let r = Ray::new(Point::new_point(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Point::new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_two_pts() {
        let r = Ray::new(Point::new_point(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new_point(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new_point(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new_point(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new_point(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 2);
        // assuming two intersections for simplicity
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersection_encapsulates_t_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = Intersections::from(vec![i1, i2]);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
        assert_eq!(xs.size(), 2);
    }

    #[test]
    fn hit_all_intersections_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = Intersections::from(vec![i1, i2]);
        let i = hit(xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn hit_some_intersections_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);

        let xs = Intersections::from(vec![i2, i1]);
        let i = hit(xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn hit_all_intersections_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);

        let xs = Intersections::from(vec![i2, i1]);
        // i should be none, implement with option
        let i = hit(xs);
        assert_eq!(i, None);
    }
}
