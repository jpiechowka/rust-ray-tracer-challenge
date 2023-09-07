use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::float_comparison::is_equal_f64_with_margin;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        is_equal_f64_with_margin(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        is_equal_f64_with_margin(self.w, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot_product(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross_product(&self, other: &Tuple) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: self.w, // Should always be 0. We care about three-dimensional cross product only
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Tuple {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Tuple {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        };
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn tuple() -> Tuple {
        Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        }
    }

    #[fixture]
    pub fn point() -> Tuple {
        Tuple::new_point(1.0, -2.0, 3.0)
    }

    #[fixture]
    pub fn point2() -> Tuple {
        Tuple::new_point(-2.0, 4.0, -6.0)
    }

    #[fixture]
    pub fn vector() -> Tuple {
        Tuple::new_vector(1.0, -2.0, 3.0)
    }

    #[fixture]
    pub fn vector2() -> Tuple {
        Tuple::new_vector(-2.0, 4.0, -6.0)
    }

    #[rstest]
    fn a_tuple_with_w_equal_to_1_is_a_point(point: Tuple) {
        assert!(point.is_point());
        assert!(!point.is_vector());
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, -2.0);
        assert_eq!(point.z, 3.0);
        assert_eq!(point.w, 1.0);
    }

    #[rstest]
    fn a_tuple_with_w_equal_to_0_is_a_vector(vector: Tuple) {
        assert!(vector.is_vector());
        assert!(!vector.is_point());
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, -2.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[rstest]
    fn can_add_two_tuples(point: Tuple, vector: Tuple) {
        let result = point + vector;
        assert_eq!(
            result,
            Tuple {
                x: 2.0,
                y: -4.0,
                z: 6.0,
                w: 1.0
            }
        );
        assert!(result.is_point());
        assert!(!result.is_vector());
    }

    #[rstest]
    fn can_add_assign_two_tuples(vector: Tuple) {
        let mut result = Tuple::new_point(2.2, -3.3, 4.4);
        result += vector;

        assert_eq!(
            result,
            Tuple {
                x: 3.2,
                y: -5.3,
                z: 7.4,
                w: 1.0
            }
        );
        assert!(result.is_point());
        assert!(!result.is_vector());
    }

    #[rstest]
    fn can_subtract_two_points(point: Tuple, point2: Tuple) {
        let result = point - point2;
        assert_eq!(
            result,
            Tuple {
                x: 3.0,
                y: -6.0,
                z: 9.0,
                w: 0.0
            }
        );
        assert!(!result.is_point());
        assert!(result.is_vector());
    }

    #[rstest]
    fn can_subtract_vector_from_a_point(point: Tuple, vector: Tuple) {
        let result = point - vector;
        assert_eq!(
            result,
            Tuple {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0
            }
        );
        assert!(result.is_point());
        assert!(!result.is_vector());
    }

    #[rstest]
    fn can_subtract_two_vectors(vector: Tuple, vector2: Tuple) {
        let result = vector - vector2;
        assert_eq!(
            result,
            Tuple {
                x: 3.0,
                y: -6.0,
                z: 9.0,
                w: 0.0
            }
        );
        assert!(!result.is_point());
        assert!(result.is_vector());
    }

    #[rstest]
    fn can_subtract_assign_two_vectors(vector: Tuple) {
        let mut result = Tuple::new_vector(3.0, 6.6, 1.0);
        result -= vector;

        assert_eq!(
            result,
            Tuple {
                x: 2.0,
                y: 8.6,
                z: -2.0,
                w: 0.0
            }
        );
        assert!(!result.is_point());
        assert!(result.is_vector());
    }

    #[rstest]
    fn can_negate_a_tuple(tuple: Tuple) {
        assert_eq!(
            -tuple,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    #[rstest]
    fn can_multiply_a_tuple_by_a_scalar(tuple: Tuple) {
        assert_eq!(
            tuple * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        );
        assert_eq!(
            tuple * -3.5,
            Tuple {
                x: -3.5,
                y: 7.0,
                z: -10.5,
                w: 14.0
            }
        );
    }

    #[rstest]
    fn can_multiply_a_tuple_by_a_fraction(tuple: Tuple) {
        assert_eq!(
            tuple * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
        assert_eq!(
            tuple * -0.5,
            Tuple {
                x: -0.5,
                y: 1.0,
                z: -1.5,
                w: 2.0
            }
        );
    }

    #[rstest]
    fn can_divide_a_tuple_by_a_scalar(tuple: Tuple) {
        assert_eq!(
            tuple / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[rstest]
    #[case::vector_x_is_1(Tuple::new_vector(1.0, 0.0, 0.0), 1.0)]
    #[case::vector_y_is_1(Tuple::new_vector(0.0, 1.0, 0.0), 1.0)]
    #[case::vector_z_is_1(Tuple::new_vector(0.0, 0.0, 1.0), 1.0)]
    #[case::vector_with_positive_values(Tuple::new_vector(1.0, 2.0, 3.0), 14_f64.sqrt())]
    #[case::vector_with_negative_values(Tuple::new_vector(-1.0, -2.0, -3.0), 14_f64.sqrt())]
    #[case::vector_with_mixed_fractional_values(Tuple::new_vector(0.5, -1.5, 2.5), 8.75_f64.sqrt())]
    fn can_calculate_magnitude_of_a_vector(
        #[case] input_vector: Tuple,
        #[case] expected_magnitude: f64,
    ) {
        assert!(is_equal_f64_with_margin(
            input_vector.magnitude(),
            expected_magnitude
        ));
    }

    #[rstest]
    #[case::vector_x_is_4(Tuple::new_vector(4.0, 0.0, 0.0), Tuple::new_vector(1.0, 0.0, 0.0))]
    #[case::vector_1_2_3(Tuple::new_vector(1.0, 2.0, 3.0), Tuple::new_vector(1.0/14_f64.sqrt(), 2.0/14_f64.sqrt(), 3.0/14_f64.sqrt()))]
    fn can_normalize_a_vector(#[case] input_vector: Tuple, #[case] expected_vector: Tuple) {
        let normalized_vector = input_vector.normalize();
        assert!(is_equal_f64_with_margin(
            normalized_vector.x,
            expected_vector.x
        ));
        assert!(is_equal_f64_with_margin(
            normalized_vector.y,
            expected_vector.y
        ));
        assert!(is_equal_f64_with_margin(
            normalized_vector.z,
            expected_vector.z
        ));
        assert!(is_equal_f64_with_margin(
            normalized_vector.w,
            expected_vector.w
        ));
    }

    #[rstest]
    fn can_calculate_magnitude_of_a_normalized_vector(vector: Tuple) {
        let normalized_vector = vector.normalize();
        assert_eq!(normalized_vector.magnitude(), 1.0);
    }

    #[rstest]
    fn can_calculate_dot_product_of_two_vectors(vector: Tuple, vector2: Tuple) {
        assert_eq!(vector.dot_product(&vector2), -28.0);
    }

    #[rstest]
    #[case::regular(
        Tuple::new_vector(1.0, 2.0, 3.0),
        Tuple::new_vector(2.0, 3.0, 4.0),
        Tuple::new_vector(-1.0, 2.0, -1.0)
    )]
    #[case::reversed_order(
        Tuple::new_vector(2.0, 3.0, 4.0),
        Tuple::new_vector(1.0, 2.0, 3.0),
        Tuple::new_vector(1.0, -2.0, 1.0)
    )]
    #[case::with_negative_numbers(
        Tuple::new_vector(3.0, 0.0, 2.0),
        Tuple::new_vector(-1.0, 4.0, 2.0),
        Tuple::new_vector(-8.0, -8.0, 12.0)
    )]
    fn can_calculate_cross_product_of_two_vectors(
        #[case] first_vector: Tuple,
        #[case] second_vector: Tuple,
        #[case] expected_vector: Tuple,
    ) {
        assert_eq!(first_vector.cross_product(&second_vector), expected_vector);
    }
}
