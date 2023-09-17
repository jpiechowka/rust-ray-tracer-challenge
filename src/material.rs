use crate::color::Color;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Color::new_white(), 0.1, 0.9, 0.9, 200.0)
    }
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::material::Material;

    #[test]
    fn can_create_default_material() {
        let material = Material::default();
        assert_eq!(material.color, Color::new_white());
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }
}
