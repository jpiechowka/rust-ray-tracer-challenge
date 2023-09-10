use glam::Vec3A;

pub trait Color {
    // TODO: Decide if we should use clamping at all? Conversion methods could perform some more calculations
    fn new_color(red: f32, green: f32, blue: f32) -> Self;
    fn new_black() -> Self;
    fn new_white() -> Self;
    fn new_red() -> Self;
    fn new_green() -> Self;
    fn new_blue() -> Self;
    fn get_red_val(&self) -> f32;
    fn get_green_val(&self) -> f32;
    fn get_blue_val(&self) -> f32;
    fn get_red_val_as_u8(&self) -> u8;
    fn get_green_val_as_u8(&self) -> u8;
    fn get_blue_val_as_u8(&self) -> u8;
}

impl Color for Vec3A {
    fn new_color(red: f32, green: f32, blue: f32) -> Self {
        Vec3A::new(red, green, blue)
    }

    fn new_black() -> Self {
        Vec3A::new(0.0, 0.0, 0.0)
    }

    fn new_white() -> Self {
        Vec3A::new(1.0, 1.0, 1.0)
    }

    fn new_red() -> Self {
        Vec3A::new(1.0, 0.0, 0.0)
    }

    fn new_green() -> Self {
        Vec3A::new(0.0, 1.0, 0.0)
    }

    fn new_blue() -> Self {
        Vec3A::new(0.0, 0.0, 1.0)
    }

    fn get_red_val(&self) -> f32 {
        self.x
    }

    fn get_green_val(&self) -> f32 {
        self.y
    }

    fn get_blue_val(&self) -> f32 {
        self.z
    }

    fn get_red_val_as_u8(&self) -> u8 {
        (self.x * 255.0).round() as u8
    }

    fn get_green_val_as_u8(&self) -> u8 {
        (self.y * 255.0).round() as u8
    }

    fn get_blue_val_as_u8(&self) -> u8 {
        (self.z * 255.0).round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::*;

    #[fixture]
    pub fn color() -> Vec3A {
        Vec3A::new_color(0.9, 0.6, 0.75)
    }

    #[fixture]
    pub fn color2() -> Vec3A {
        Vec3A::new_color(0.7, 0.1, 0.25)
    }

    #[rstest]
    #[case::red(Vec3A::new_color(10.0, 0.0, 0.0), Vec3A::new_color(10.0, 0.0, 0.0))]
    #[case::green(Vec3A::new_color(0.0, 10.0, 0.0), Vec3A::new_color(0.0, 10.0, 0.0))]
    #[case::blue(Vec3A::new_color(0.0, 0.0, 10.0), Vec3A::new_color(0.0, 0.0, 10.0))]
    #[case::red_green(Vec3A::new_color(10.0, 10.0, 0.0), Vec3A::new_color(10.0, 10.0, 0.0))]
    #[case::red_blue(Vec3A::new_color(10.0, 0.0, 10.0), Vec3A::new_color(10.0, 0.0, 10.0))]
    #[case::green_blue(Vec3A::new_color(0.0, 10.0, 10.0), Vec3A::new_color(0.0, 10.0, 10.0))]
    #[case::red_green_blue(Vec3A::new_color(10.0, 10.0, 10.0), Vec3A::new_color(10.0, 10.0, 10.0))]
    fn can_create_color_without_clamped_values(
        #[case] input_color: Vec3A,
        #[case] expected_color: Vec3A,
    ) {
        assert!(input_color.abs_diff_eq(expected_color, f32::EPSILON));
    }

    #[rstest]
    fn can_add_colors(color: Vec3A, color2: Vec3A) {
        let result = color + color2;
        assert!(result.abs_diff_eq(Vec3A::new_color(1.6, 0.7, 1.0), f32::EPSILON));
    }

    #[rstest]
    fn can_subtract_colors(color: Vec3A, color2: Vec3A) {
        let result = color - color2;
        assert!(result.abs_diff_eq(Vec3A::new_color(0.2, 0.5, 0.5), f32::EPSILON));
    }

    #[rstest]
    fn can_multiply_colors_aka_hadamard_product(color: Vec3A, color2: Vec3A) {
        let result = color * color2;
        assert!(result.abs_diff_eq(Vec3A::new_color(0.63, 0.06, 0.1875), f32::EPSILON));
    }

    #[rstest]
    fn can_multiply_color_by_a_scalar(color: Vec3A) {
        let result = color * 2.0;
        assert!(result.abs_diff_eq(Vec3A::new_color(1.8, 1.2, 1.5), f32::EPSILON));
    }

    #[rstest]
    #[case::black(Vec3A::new_black(), Vec3A::new_color(0.0, 0.0, 0.0))]
    #[case::white(Vec3A::new_white(), Vec3A::new_color(1.0, 1.0, 1.0))]
    #[case::red(Vec3A::new_red(), Vec3A::new_color(1.0, 0.0, 0.0))]
    #[case::green(Vec3A::new_green(), Vec3A::new_color(0.0, 1.0, 0.0))]
    #[case::blue(Vec3A::new_blue(), Vec3A::new_color(0.0, 0.0, 1.0))]
    fn can_create_colors_with_utility_functions(
        #[case] input_color: Vec3A,
        #[case] expected_color: Vec3A,
    ) {
        assert!(input_color.abs_diff_eq(expected_color, f32::EPSILON));
    }

    #[rstest]
    fn can_get_colors(color: Vec3A) {
        assert_abs_diff_eq!(color.x, 0.9, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.y, 0.6, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.z, 0.75, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.get_red_val(), 0.9, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.get_green_val(), 0.6, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(color.get_blue_val(), 0.75, epsilon = f32::EPSILON);
    }

    #[rstest]
    #[case::black(Vec3A::new_black(), 0, 0, 0)]
    #[case::white(Vec3A::new_white(), 255, 255, 255)]
    #[case::red(Vec3A::new_red(), 255, 0, 0)]
    #[case::green(Vec3A::new_green(), 0, 255, 0)]
    #[case::blue(Vec3A::new_blue(), 0, 0, 255)]
    #[case(Vec3A::new_color(0.25, 0.25, 0.25), 64, 64, 64)]
    #[case(Vec3A::new_color(0.5, 0.5, 0.5), 128, 128, 128)]
    #[case(Vec3A::new_color(0.75, 0.75, 0.75), 191, 191, 191)]
    #[case(Vec3A::new_color(0.4, 0.6, 0.8), 102, 153, 204)]
    fn can_get_colors_converted_to_u8(
        #[case] input_color: Vec3A,
        #[case] expected_red_value: u8,
        #[case] expected_green_value: u8,
        #[case] expected_blue_value: u8,
    ) {
        assert_eq!(input_color.get_red_val_as_u8(), expected_red_value);
        assert_eq!(input_color.get_green_val_as_u8(), expected_green_value);
        assert_eq!(input_color.get_blue_val_as_u8(), expected_blue_value);
    }
}
