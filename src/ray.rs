use glam::Vec3A;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    origin_point: Vec3A,
    direction_vector: Vec3A,
}

impl Ray {
    pub fn new(origin_point: Vec3A, direction_vector: Vec3A) -> Self {
        Self {
            origin_point,
            direction_vector,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::*;

    #[fixture]
    pub fn ray() -> Ray {
        Ray::new(Vec3A::new(1.0, 2.0, 3.0), Vec3A::new(4.0, 5.0, 6.0))
    }

    #[rstest]
    fn can_create_ray(ray: Ray) {
        assert_abs_diff_eq!(ray.origin_point.x, 1.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(ray.origin_point.y, 2.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(ray.origin_point.z, 3.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(ray.direction_vector.x, 4.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(ray.direction_vector.y, 5.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(ray.direction_vector.z, 6.0, epsilon = f32::EPSILON);
    }
}
