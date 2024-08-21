use std::borrow::Borrow;
use std::ops::Index;

use crate::light::Material;
use crate::matrix::{invert_4x4, transpose, M4x4, IDENTITY_MATRIX_4X4};
use crate::tuple::{Point, Tuple, Vector};
use uuid::Uuid;

pub const SPHERE_ORIGIN: Tuple = Point {
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

    /// Calculates the discriminant of the quadratic equation that arises from the
    /// intersection of a ray with a sphere centered at the origin (0, 0, 0) with a unit radius.
    ///
    /// This discriminant is derived by substituting the ray's equation into the sphere's equation,
    /// resulting in a quadratic equation in terms of the parameter `t`. The discriminant determines
    /// the nature of the intersection:
    /// - A positive discriminant indicates two intersection points, meaning the ray enters and exits the sphere.
    /// - A zero discriminant indicates one intersection point, meaning the ray is tangent to the sphere (touching it at one point).
    /// - A negative discriminant indicates no intersection, meaning the ray does not intersect the sphere.
    ///
    /// This method assumes the sphere is centered at the origin with a radius of 1, simplifying the
    /// calculations by focusing on the specific case where the sphere is at the origin.
    ///
    /// # Returns
    ///
    /// The discriminant value as a floating-point number (`f64`).
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
    pub transform: M4x4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            id: Uuid::new_v4(),
            transform: IDENTITY_MATRIX_4X4,
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, transform: M4x4) {
        self.transform = transform;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    /// Calculates the normal vector at a given point on the surface of the sphere, transforming
    /// from world space to object space and back to world space correctly.
    ///
    /// # Arguments
    /// * `world_point` - A `Point` in world space for which the normal vector is to be calculated.
    ///                   This point is assumed to lie on the surface of the sphere.
    ///
    /// # Returns
    /// * `Vector` - The normal vector at the given point in world space.
    ///
    /// # Methodology
    /// 1. **World to Object Space Transformation**:
    ///    - The method begins by transforming the given `world_point` into the sphere's local
    ///      coordinate system (object space). This is achieved by applying the inverse of the
    ///      sphere's transformation matrix. In object space, the sphere is assumed to be centered
    ///      at the origin with a radius of 1. This simplifies the normal calculation.
    ///      \[
    ///      \text{object\_point} = T^{-1} \times \text{world\_point}
    ///      \]
    ///
    /// 2. **Normal Calculation in Object Space**:
    ///    - In object space, the normal at any point on the sphere's surface is simply the vector
    ///      from the origin (the sphere's center) to the point itself. This vector is calculated
    ///      by subtracting the origin from the `object_point`.
    ///
    /// 3. **Transforming the Normal to World Space**:
    ///    - The normal vector is then transformed back to world space. However, because normals
    ///      interact with transformations differently from points (especially under non-uniform
    ///      scaling), the transpose of the inverse of the transformation matrix is used:
    ///      \[
    ///      \text{world\_normal} = (T^{-1})^{T} \times \text{object\_normal}
    ///      \]
    ///
    /// 4. **Normalization and Correction**:
    ///    - The resulting world-space normal vector is normalized to ensure it has unit length.
    ///      Additionally, the `w` component of the normal vector is explicitly set to `0.0` to
    ///      indicate that it represents a direction rather than a point in space.
    ///
    /// # Considerations
    /// - The function assumes that the `world_point` provided is exactly on the sphere's surface.
    /// - The matrix inversion and transposition steps are computationally intensive and must be
    ///   carefully implemented to avoid numerical instability.
    /// - This method is crucial for accurate lighting and shading calculations, as the normal
    ///   vector plays a key role in determining how light interacts with the surface.
    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = invert_4x4(&self.transform).unwrap() * world_point;
        let object_normal = object_point - Point::new_point(0.0, 0.0, 0.0);
        // transposing the inverse matrix is necessary because it ensures that the normal vector
        // is correctly transformed to remain perpendicular to the surface after
        // non-uniform scaling, rotation, and other transformations
        let world_normal = transpose(invert_4x4(&self.transform).unwrap()) * object_normal;
        let mut normal = Vector::new(world_normal.x, world_normal.y, world_normal.z).normalize();
        // translation can mess up the w coordinate
        // avoid more complex code with hack / set w to 0
        normal.w = 0.0;
        normal
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
    pub fn size(&self) -> usize {
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
    // first transform ray by inverse of sphere's transformation
    let inverted_tx = invert_4x4(&s.transform).unwrap();
    let r = transform(r, inverted_tx);

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
    xs.items
        .iter() // Iterate over the intersections
        .filter(|i| i.t >= 0.0) // Only consider intersections with t >= 0.0
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) // Find the intersection with the smallest t
        .copied() // Convert the reference to an owned value
}

pub fn transform(ray: &Ray, translation_matrix: M4x4) -> Ray {
    let new_origin = translation_matrix * ray.origin;
    let new_direction = translation_matrix * ray.direction;
    Ray::new(new_origin, new_direction)
}

pub fn reflect(incoming: Vector, normal: Vector) -> Vector {
    incoming - normal * 2.0_f64 * incoming.dot(&normal)
}

#[cfg(test)]
mod tests {
    use crate::light::Material;
    use crate::matrix::IDENTITY_MATRIX_4X4;
    use crate::matrix_transformations::{rotation_z, scaling, translation};
    use crate::rays::{
        hit, intersect, reflect, transform, Intersection, Intersections, Ray, Sphere,
    };
    use crate::tuple::{Point, Vector};
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

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
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Point::new_point(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
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

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new_point(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = transform(&r, m);
        assert_eq!(r2.origin, Point::new_point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new_point(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = transform(&r, m);
        assert_eq!(r2.origin, Point::new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, IDENTITY_MATRIX_4X4);
    }

    #[test]
    fn changing_sphere_transform() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transform, t)
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new_point(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new_point(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = intersect(&r, s);
        assert_eq!(xs.size(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_x_axis() {
        let s = Sphere::new();
        let norm = s.normal_at(Point::new_point(1.0, 0.0, 0.0));
        assert_eq!(norm, Vector::new(1.0, 0.0, 0.0));
        assert_eq!(norm, norm.normalize());
    }

    #[test]
    fn normal_on_sphere_at_point_y_axis() {
        let s = Sphere::new();
        let norm = s.normal_at(Point::new_point(0.0, 1.0, 0.0));
        assert_eq!(norm, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(norm, norm.normalize());
    }
    #[test]
    fn normal_on_sphere_at_point_z_axis() {
        let s = Sphere::new();
        let norm = s.normal_at(Point::new_point(0.0, 0.0, 1.0));
        assert_eq!(norm, Vector::new(0.0, 0.0, 1.0));
        assert_eq!(norm, norm.normalize());
    }

    #[test]
    fn normal_on_sphere_at_non_axial_point() {
        let s = Sphere::new();
        let val = (3.0_f64).sqrt() / 3.0;
        let norm = s.normal_at(Point::new_point(val, val, val));
        assert_eq!(norm, Vector::new(val, val, val));
        assert_eq!(norm, norm.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new_point(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(Point::new_point(
            0.0,
            (2.0_f64.sqrt()) / 2.0,
            -(2.0_f64.sqrt()) / 2.0,
        ));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn reflecting_vector_at_45_degrees() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = reflect(v, n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let r = reflect(v, n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_has_default_material() {
        let s = Sphere::new();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    #[test]
    fn test_sphere_can_be_assigned_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m);
        assert_eq!(s.material, m);
    }
}
