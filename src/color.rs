use std::ops::{Add, Mul, Sub};

use glam::Vec3A;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    values: Vec3A,
}

impl Color {
    pub fn new_color(red: f32, green: f32, blue: f32) -> Self {
        Self {
            values: Vec3A::new(red, green, blue),
        }
    }

    pub fn new_black() -> Self {
        Self {
            values: Vec3A::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new_white() -> Self {
        Self {
            values: Vec3A::new(1.0, 1.0, 1.0),
        }
    }

    pub fn new_red() -> Self {
        Self {
            values: Vec3A::new(1.0, 0.0, 0.0),
        }
    }

    pub fn new_green() -> Self {
        Self {
            values: Vec3A::new(0.0, 1.0, 0.0),
        }
    }

    pub fn new_blue() -> Self {
        Self {
            values: Vec3A::new(0.0, 0.0, 1.0),
        }
    }

    pub fn get_red_val(&self) -> f32 {
        self.values.x
    }

    pub fn get_green_val(&self) -> f32 {
        self.values.y
    }

    pub fn get_blue_val(&self) -> f32 {
        self.values.z
    }

    pub fn get_red_val_as_u8(&self) -> u8 {
        (self.values.x * 255.0).round() as u8
    }

    pub fn get_green_val_as_u8(&self) -> u8 {
        (self.values.y * 255.0).round() as u8
    }

    pub fn get_blue_val_as_u8(&self) -> u8 {
        (self.values.z * 255.0).round() as u8
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            values: self.values + other.values,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            values: self.values - other.values,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            values: self.values * other.values,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            values: self.values * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::*;

    #[fixture]
    pub fn color() -> Color {
        Color::new_color(0.9, 0.6, 0.75)
    }

    #[fixture]
    pub fn color2() -> Color {
        Color::new_color(0.7, 0.1, 0.25)
    }

    #[rstest]
    #[case::red(Color::new_color(10.0, 0.0, 0.0), Color::new_color(10.0, 0.0, 0.0))]
    #[case::green(Color::new_color(0.0, 10.0, 0.0), Color::new_color(0.0, 10.0, 0.0))]
    #[case::blue(Color::new_color(0.0, 0.0, 10.0), Color::new_color(0.0, 0.0, 10.0))]
    #[case::red_green(Color::new_color(10.0, 10.0, 0.0), Color::new_color(10.0, 10.0, 0.0))]
    #[case::red_blue(Color::new_color(10.0, 0.0, 10.0), Color::new_color(10.0, 0.0, 10.0))]
    #[case::green_blue(Color::new_color(0.0, 10.0, 10.0), Color::new_color(0.0, 10.0, 10.0))]
    #[case::red_green_blue(Color::new_color(10.0, 10.0, 10.0), Color::new_color(10.0, 10.0, 10.0))]
    fn can_create_colors(#[case] input_color: Color, #[case] expected_color: Color) {
        assert_abs_diff_eq!(
            input_color.values.x,
            expected_color.values.x,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            input_color.values.y,
            expected_color.values.y,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            input_color.values.z,
            expected_color.values.z,
            epsilon = f32::EPSILON
        );
    }

    #[rstest]
    fn can_add_colors(color: Color, color2: Color) {
        let result = color + color2;
        assert_abs_diff_eq!(result.values.x, 1.6, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.y, 0.7, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.z, 1.0, epsilon = f32::EPSILON);
    }

    #[rstest]
    fn can_subtract_colors(color: Color, color2: Color) {
        let result = color - color2;
        assert_abs_diff_eq!(result.values.x, 0.2, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.y, 0.5, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.z, 0.5, epsilon = f32::EPSILON);
    }

    #[rstest]
    fn can_multiply_colors_aka_hadamard_product(color: Color, color2: Color) {
        let result = color * color2;
        assert_abs_diff_eq!(result.values.x, 0.63, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.y, 0.06, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.z, 0.1875, epsilon = f32::EPSILON);
    }

    #[rstest]
    fn can_multiply_color_by_a_scalar(color: Color) {
        let result = color * 2.0;
        assert_abs_diff_eq!(result.values.x, 1.8, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.y, 1.2, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(result.values.z, 1.5, epsilon = f32::EPSILON);
    }

    #[rstest]
    #[case::black(Color::new_black(), Color::new_color(0.0, 0.0, 0.0))]
    #[case::white(Color::new_white(), Color::new_color(1.0, 1.0, 1.0))]
    #[case::red(Color::new_red(), Color::new_color(1.0, 0.0, 0.0))]
    #[case::green(Color::new_green(), Color::new_color(0.0, 1.0, 0.0))]
    #[case::blue(Color::new_blue(), Color::new_color(0.0, 0.0, 1.0))]
    fn can_create_colors_with_utility_functions(
        #[case] input_color: Color,
        #[case] expected_color: Color,
    ) {
        assert_abs_diff_eq!(
            input_color.values.x,
            expected_color.values.x,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            input_color.values.y,
            expected_color.values.y,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            input_color.values.z,
            expected_color.values.z,
            epsilon = f32::EPSILON
        );
    }

    #[rstest]
    fn can_get_colors(color: Color) {
        assert_abs_diff_eq!(color.values.x, 0.9, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.values.y, 0.6, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.values.z, 0.75, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.get_red_val(), 0.9, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.get_green_val(), 0.6, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.get_blue_val(), 0.75, epsilon = f32::EPSILON);
    }

    #[rstest]
    #[case::black(Color::new_black(), 0, 0, 0)]
    #[case::white(Color::new_white(), 255, 255, 255)]
    #[case::red(Color::new_red(), 255, 0, 0)]
    #[case::green(Color::new_green(), 0, 255, 0)]
    #[case::blue(Color::new_blue(), 0, 0, 255)]
    #[case(Color::new_color(0.25, 0.25, 0.25), 64, 64, 64)]
    #[case(Color::new_color(0.5, 0.5, 0.5), 128, 128, 128)]
    #[case(Color::new_color(0.75, 0.75, 0.75), 191, 191, 191)]
    #[case(Color::new_color(0.4, 0.6, 0.8), 102, 153, 204)]
    fn can_get_colors_converted_to_u8(
        #[case] input_color: Color,
        #[case] expected_red_value: u8,
        #[case] expected_green_value: u8,
        #[case] expected_blue_value: u8,
    ) {
        assert_eq!(input_color.get_red_val_as_u8(), expected_red_value);
        assert_eq!(input_color.get_green_val_as_u8(), expected_green_value);
        assert_eq!(input_color.get_blue_val_as_u8(), expected_blue_value);
    }
}
