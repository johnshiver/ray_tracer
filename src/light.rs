use crate::color::Color;
use crate::rays::reflect;
use crate::tuple::{Point, Vector};

#[derive(Clone, Copy)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn lighting(
    material: Material,
    light: PointLight,
    point: Point,
    eyev: Vector,
    normalv: Vector,
) -> Color {
    // Combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;

    // Find the direction to the light source
    let lightv = (light.position - point).normalize();

    // Compute the ambient contribution
    let ambient = effective_color * material.ambient;

    // Light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = lightv.dot(&normalv);
    let (diffuse, specular);

    if light_dot_normal < 0.0 {
        diffuse = Color::new(0.0, 0.0, 0.0); // black
        specular = Color::new(0.0, 0.0, 0.0); // black
    } else {
        // Compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // Reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflectv = reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);

        if reflect_dot_eye <= 0.0 {
            specular = Color::new(0.0, 0.0, 0.0); // black
        } else {
            // Compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    // Add the three contributions together to get the final shading
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::light::{lighting, Material, PointLight};
    use crate::tuple::{Point, Vector};

    #[test]
    fn point_light_has_position_and_intensity() {
        let pos = Point::new_point(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let light = PointLight::new(pos, intensity);
        assert_eq!(light.position, pos);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn test_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_eye_offset_45_degrees() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, (2.0_f64).sqrt() / 2.0, -(2.0_f64).sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_with_light_offset_45_degrees() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -(2.0_f64).sqrt() / 2.0, -(2.0_f64).sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_with_light_behind_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
