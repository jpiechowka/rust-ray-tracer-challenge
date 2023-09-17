use glam::Vec3A;

use crate::color::Color;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Light {
    position: Vec3A,
    intensity: Color,
}

impl Light {
    pub fn new_point_light(position: Vec3A, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3A;

    use crate::color::Color;
    use crate::light::Light;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let light_position = Vec3A::splat(0.0);
        let light_intensity = Color::new_white();
        let light = Light::new_point_light(light_position, light_intensity);
        assert_eq!(light.position, light_position);
        assert_eq!(light.intensity, light_intensity);
    }
}
