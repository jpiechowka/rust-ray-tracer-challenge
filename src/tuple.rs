use approx::abs_diff_eq;
use glam::Vec4;

pub trait Tuple {
    fn new_point_tuple(x: f32, y: f32, z: f32) -> Self;
    fn new_vector_tuple(x: f32, y: f32, z: f32) -> Self;
    fn is_point_tuple(&self) -> bool;
    fn is_vector_tuple(&self) -> bool;
}

impl Tuple for Vec4 {
    fn new_point_tuple(x: f32, y: f32, z: f32) -> Self {
        Vec4::new(x, y, z, 1.0)
    }

    fn new_vector_tuple(x: f32, y: f32, z: f32) -> Self {
        Vec4::new(x, y, z, 0.0)
    }

    fn is_point_tuple(&self) -> bool {
        abs_diff_eq!(self.w, 1.0, epsilon = f32::EPSILON)
    }

    fn is_vector_tuple(&self) -> bool {
        abs_diff_eq!(self.w, 0.0, epsilon = f32::EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use glam::Vec3A;
    use rstest::*;

    #[fixture]
    pub fn tuple() -> Vec4 {
        Vec4::new(1.0, -2.0, 3.0, -4.0)
    }

    #[fixture]
    pub fn point() -> Vec4 {
        Tuple::new_point_tuple(1.0, -2.0, 3.0)
    }

    #[fixture]
    pub fn point2() -> Vec4 {
        Tuple::new_point_tuple(-2.0, 4.0, -6.0)
    }

    #[fixture]
    pub fn vector() -> Vec4 {
        Tuple::new_vector_tuple(1.0, -2.0, 3.0)
    }

    #[fixture]
    pub fn vector2() -> Vec4 {
        Tuple::new_vector_tuple(-2.0, 4.0, -6.0)
    }

    #[rstest]
    fn a_tuple_with_w_equal_to_1_is_a_point(point: Vec4) {
        assert!(point.is_point_tuple());
        assert!(!point.is_vector_tuple());
        assert_abs_diff_eq!(point.x, 1.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(point.y, -2.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(point.z, 3.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(point.w, 1.0, epsilon = f32::EPSILON);
    }

    #[rstest]
    fn a_tuple_with_w_equal_to_0_is_a_vector(vector: Vec4) {
        assert!(vector.is_vector_tuple());
        assert!(!vector.is_point_tuple());
        assert_abs_diff_eq!(vector.x, 1.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(vector.y, -2.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(vector.z, 3.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(vector.w, 0.0, epsilon = f32::EPSILON);
    }

    #[rstest]
    fn can_add_two_tuples(point: Vec4, vector: Vec4) {
        let result = point + vector;
        assert_eq!(result, Vec4::new(2.0, -4.0, 6.0, 1.0));
        assert!(result.is_point_tuple());
        assert!(!result.is_vector_tuple());
    }

    #[rstest]
    fn can_add_assign_two_tuples(vector: Vec4) {
        let mut result = Vec4::new_point_tuple(2.2, -3.3, 4.4);
        result += vector;

        assert_eq!(result, Vec4::new(3.2, -5.3, 7.4, 1.0));
        assert!(result.is_point_tuple());
        assert!(!result.is_vector_tuple());
    }

    #[rstest]
    fn can_subtract_two_points(point: Vec4, point2: Vec4) {
        let result = point - point2;
        assert_eq!(result, Vec4::new(3.0, -6.0, 9.0, 0.0));
        assert!(!result.is_point_tuple());
        assert!(result.is_vector_tuple());
    }

    #[rstest]
    fn can_subtract_vector_from_a_point(point: Vec4, vector: Vec4) {
        let result = point - vector;
        assert_eq!(result, Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert!(result.is_point_tuple());
        assert!(!result.is_vector_tuple());
    }

    #[rstest]
    fn can_subtract_two_vectors(vector: Vec4, vector2: Vec4) {
        let result = vector - vector2;
        assert_eq!(result, Vec4::new(3.0, -6.0, 9.0, 0.0));
        assert!(!result.is_point_tuple());
        assert!(result.is_vector_tuple());
    }

    #[rstest]
    fn can_subtract_assign_two_vectors(vector: Vec4) {
        let mut result = Vec4::new_vector_tuple(3.0, 6.6, 1.0);
        result -= vector;

        assert_eq!(result, Vec4::new(2.0, 8.6, -2.0, 0.0));
        assert!(!result.is_point_tuple());
        assert!(result.is_vector_tuple());
    }

    #[rstest]
    fn can_negate_a_tuple(tuple: Vec4) {
        assert_eq!(-tuple, Vec4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[rstest]
    fn can_multiply_a_tuple_by_a_scalar(tuple: Vec4) {
        assert_eq!(tuple * 3.5, Vec4::new(3.5, -7.0, 10.5, -14.0));
        assert_eq!(tuple * -3.5, Vec4::new(-3.5, 7.0, -10.5, 14.0));
    }

    #[rstest]
    fn can_multiply_a_tuple_by_a_fraction(tuple: Vec4) {
        assert_eq!(tuple * 0.5, Vec4::new(0.5, -1.0, 1.5, -2.0));
        assert_eq!(tuple * -0.5, Vec4::new(-0.5, 1.0, -1.5, 2.0));
    }

    #[rstest]
    fn can_divide_a_tuple_by_a_scalar(tuple: Vec4) {
        assert_eq!(tuple / 2.0, Vec4::new(0.5, -1.0, 1.5, -2.0));
    }

    #[rstest]
    #[case::vector_x_is_1(Tuple::new_vector_tuple(1.0, 0.0, 0.0), 1.0)]
    #[case::vector_y_is_1(Tuple::new_vector_tuple(0.0, 1.0, 0.0), 1.0)]
    #[case::vector_z_is_1(Tuple::new_vector_tuple(0.0, 0.0, 1.0), 1.0)]
    #[case::vector_with_positive_values(Tuple::new_vector_tuple(1.0, 2.0, 3.0), 14_f32.sqrt())]
    #[case::vector_with_negative_values(Tuple::new_vector_tuple(-1.0, -2.0, -3.0), 14_f32.sqrt())]
    #[case::vector_with_mixed_fractional_values(Tuple::new_vector_tuple(0.5, -1.5, 2.5), 8.75_f32.sqrt())]
    fn can_calculate_magnitude_of_a_vector(
        #[case] input_vector: Vec4,
        #[case] expected_magnitude: f32,
    ) {
        assert_eq!(input_vector.length(), expected_magnitude);
    }

    #[rstest]
    #[case::vector_x_is_4(
        Tuple::new_vector_tuple(4.0, 0.0, 0.0),
        Tuple::new_vector_tuple(1.0, 0.0, 0.0)
    )]
    #[case::vector_1_2_3(
        Tuple::new_vector_tuple(1.0, 2.0, 3.0),
        Tuple::new_vector_tuple(1.0/14_f32.sqrt(), 2.0/14_f32.sqrt(), 3.0/14_f32.sqrt())
    )]
    fn can_normalize_a_vector(#[case] input_vector: Vec4, #[case] expected_vector: Vec4) {
        let normalized_vector = input_vector.normalize();
        assert!(normalized_vector.abs_diff_eq(expected_vector, f32::EPSILON))
    }

    #[rstest]
    fn can_calculate_magnitude_of_a_normalized_vector(vector: Vec4) {
        let normalized_vector = vector.normalize();
        assert_abs_diff_eq!(normalized_vector.length(), 1.0);
    }

    #[rstest]
    fn can_calculate_dot_product_of_two_vectors(vector: Vec4, vector2: Vec4) {
        assert_eq!(vector.dot(vector2), -28.0);
    }

    #[rstest]
    #[case::regular(
        Tuple::new_vector_tuple(1.0, 2.0, 3.0),
        Tuple::new_vector_tuple(2.0, 3.0, 4.0),
        Tuple::new_vector_tuple(-1.0, 2.0, -1.0)
    )]
    #[case::reversed_order(
        Tuple::new_vector_tuple(2.0, 3.0, 4.0),
        Tuple::new_vector_tuple(1.0, 2.0, 3.0),
        Tuple::new_vector_tuple(1.0, -2.0, 1.0)
    )]
    #[case::with_negative_numbers(
        Tuple::new_vector_tuple(3.0, 0.0, 2.0),
        Tuple::new_vector_tuple(-1.0, 4.0, 2.0),
        Tuple::new_vector_tuple(-8.0, -8.0, 12.0)
    )]
    fn can_calculate_cross_product_of_two_vectors(
        #[case] first_vector: Vec4,
        #[case] second_vector: Vec4,
        #[case] expected_vector: Vec4,
    ) {
        let first_vec3 = Vec3A::from(first_vector);
        let second_vec3 = Vec3A::from(second_vector);
        let expected_vec3 = Vec3A::from(expected_vector);
        assert_eq!(first_vec3.cross(second_vec3), expected_vec3);
    }
}
