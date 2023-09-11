use glam::Vec3A;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin_point: Vec3A,
    pub direction_vector: Vec3A,
}

impl Ray {
    pub fn new(origin_point: Vec3A, direction_vector: Vec3A) -> Self {
        Self {
            origin_point,
            direction_vector,
        }
    }

    pub fn position(&self, time: f32) -> Vec3A {
        self.origin_point + self.direction_vector * time
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

    #[rstest]
    #[case(
        Ray::new(Vec3A::new(2.0, 3.0, 4.0), Vec3A::new(1.0, 0.0, 0.0)),
        0.0,
        Vec3A::new(2.0, 3.0, 4.0)
    )]
    #[case(
        Ray::new(Vec3A::new(2.0, 3.0, 4.0), Vec3A::new(1.0, 0.0, 0.0)),
        1.0,
        Vec3A::new(3.0, 3.0, 4.0)
    )]
    #[case(
        Ray::new(Vec3A::new(2.0, 3.0, 4.0), Vec3A::new(1.0, 0.0, 0.0)),
        -1.0,
        Vec3A::new(1.0, 3.0, 4.0)
    )]
    #[case(
        Ray::new(Vec3A::new(2.0, 3.0, 4.0), Vec3A::new(1.0, 0.0, 0.0)),
        2.5,
        Vec3A::new(4.5, 3.0, 4.0)
    )]
    #[case(
        Ray::new(Vec3A::new(2.0, 3.0, 4.0), Vec3A::new(3.0, 2.0, 1.0)),
        1.5,
        Vec3A::new(6.5, 6.0, 5.5)
    )]
    fn can_compute_a_point_from_distance(
        #[case] input_ray: Ray,
        #[case] time: f32,
        #[case] expected_point: Vec3A,
    ) {
        assert!(input_ray
            .position(time)
            .abs_diff_eq(expected_point, f32::EPSILON))
    }
}
