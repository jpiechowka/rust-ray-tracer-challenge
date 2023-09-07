use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new_color_unclamped(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn new_color_clamp(red: f64, green: f64, blue: f64) -> Self {
        Self {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    pub fn clamp(&self) -> Self {
        Self {
            red: self.red.clamp(0.0, 1.0),
            green: self.green.clamp(0.0, 1.0),
            blue: self.blue.clamp(0.0, 1.0),
        }
    }

    pub fn hadamard_product(&self, other: &Color) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::float_comparison::is_equal_f64_with_margin;
    use rstest::*;

    #[fixture]
    pub fn color() -> Color {
        Color::new_color_unclamped(0.9, 0.6, 0.75)
    }

    #[fixture]
    pub fn color2() -> Color {
        Color::new_color_unclamped(0.7, 0.1, 0.25)
    }

    #[rstest]
    #[case::red(
        Color::new_color_clamp(10.0, 0.0, 0.0),
        Color::new_color_unclamped(1.0, 0.0, 0.0)
    )]
    #[case::green(
        Color::new_color_clamp(0.0, 10.0, 0.0),
        Color::new_color_unclamped(0.0, 1.0, 0.0)
    )]
    #[case::blue(
        Color::new_color_clamp(0.0, 0.0, 10.0),
        Color::new_color_unclamped(0.0, 0.0, 1.0)
    )]
    #[case::red_green(
        Color::new_color_clamp(10.0, 10.0, 0.0),
        Color::new_color_unclamped(1.0, 1.0, 0.0)
    )]
    #[case::red_blue(
        Color::new_color_clamp(10.0, 0.0, 10.0),
        Color::new_color_unclamped(1.0, 0.0, 1.0)
    )]
    #[case::green_blue(
        Color::new_color_clamp(0.0, 10.0, 10.0),
        Color::new_color_unclamped(0.0, 1.0, 1.0)
    )]
    #[case::red_green_blue(
        Color::new_color_clamp(10.0, 10.0, 10.0),
        Color::new_color_unclamped(1.0, 1.0, 1.0)
    )]
    fn can_create_color_with_clamped_values(
        #[case] input_color: Color,
        #[case] expected_color: Color,
    ) {
        assert!(is_equal_f64_with_margin(
            input_color.red,
            expected_color.red
        ));
        assert!(is_equal_f64_with_margin(
            input_color.green,
            expected_color.green
        ));
        assert!(is_equal_f64_with_margin(
            input_color.blue,
            expected_color.blue
        ));
    }

    #[rstest]
    #[case::red(
        Color::new_color_unclamped(10.0, 0.0, 0.0),
        Color::new_color_unclamped(10.0, 0.0, 0.0)
    )]
    #[case::green(
        Color::new_color_unclamped(0.0, 10.0, 0.0),
        Color::new_color_unclamped(0.0, 10.0, 0.0)
    )]
    #[case::blue(
        Color::new_color_unclamped(0.0, 0.0, 10.0),
        Color::new_color_unclamped(0.0, 0.0, 10.0)
    )]
    #[case::red_green(
        Color::new_color_unclamped(10.0, 10.0, 0.0),
        Color::new_color_unclamped(10.0, 10.0, 0.0)
    )]
    #[case::red_blue(
        Color::new_color_unclamped(10.0, 0.0, 10.0),
        Color::new_color_unclamped(10.0, 0.0, 10.0)
    )]
    #[case::green_blue(
        Color::new_color_unclamped(0.0, 10.0, 10.0),
        Color::new_color_unclamped(0.0, 10.0, 10.0)
    )]
    #[case::red_green_blue(
        Color::new_color_unclamped(10.0, 10.0, 10.0),
        Color::new_color_unclamped(10.0, 10.0, 10.0)
    )]
    fn can_create_color_without_clamped_values(
        #[case] input_color: Color,
        #[case] expected_color: Color,
    ) {
        assert!(is_equal_f64_with_margin(
            input_color.red,
            expected_color.red
        ));
        assert!(is_equal_f64_with_margin(
            input_color.green,
            expected_color.green
        ));
        assert!(is_equal_f64_with_margin(
            input_color.blue,
            expected_color.blue
        ));
    }

    #[rstest]
    #[case::correct_values(Color::new_color_unclamped(1.0, 0.0, 0.0).clamp(), Color::new_color_unclamped(1.0, 0.0, 0.0))]
    #[case::correct_values2(Color::new_color_unclamped(0.0, 0.0, 0.0).clamp(), Color::new_color_unclamped(0.0, 0.0, 0.0))]
    #[case::bigger_values(Color::new_color_unclamped(0.0, 10.0, 0.0).clamp(), Color::new_color_unclamped(0.0, 1.0, 0.0))]
    #[case::negative_values(Color::new_color_unclamped(0.0, 0.0, -10.0).clamp(), Color::new_color_unclamped(0.0, 0.0, 0.0))]
    fn can_clamp_color_values(#[case] clamped_color: Color, #[case] expected_color: Color) {
        assert!(is_equal_f64_with_margin(
            clamped_color.red,
            expected_color.red
        ));
        assert!(is_equal_f64_with_margin(
            clamped_color.green,
            expected_color.green
        ));
        assert!(is_equal_f64_with_margin(
            clamped_color.blue,
            expected_color.blue
        ));
    }

    #[rstest]
    fn can_add_colors(color: Color, color2: Color) {
        let result = color + color2;
        assert!(is_equal_f64_with_margin(result.red, 1.6));
        assert!(is_equal_f64_with_margin(result.green, 0.7));
        assert!(is_equal_f64_with_margin(result.blue, 1.0));
    }

    #[rstest]
    fn can_subtract_colors(color: Color, color2: Color) {
        let result = color - color2;
        assert!(is_equal_f64_with_margin(result.red, 0.2));
        assert!(is_equal_f64_with_margin(result.green, 0.5));
        assert!(is_equal_f64_with_margin(result.blue, 0.5));
    }

    #[rstest]
    fn can_multiply_colors_aka_hadamard_product(color: Color, color2: Color) {
        let result = color.hadamard_product(&color2);
        assert!(is_equal_f64_with_margin(result.red, 0.63));
        assert!(is_equal_f64_with_margin(result.green, 0.06));
        assert!(is_equal_f64_with_margin(result.blue, 0.1875));
    }

    #[rstest]
    fn can_multiply_color_by_a_scalar(color: Color) {
        let result = color * 2.0;
        assert!(is_equal_f64_with_margin(result.red, 1.8));
        assert!(is_equal_f64_with_margin(result.green, 1.2));
        assert!(is_equal_f64_with_margin(result.blue, 1.5));
    }
}
