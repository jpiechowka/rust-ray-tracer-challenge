use glam::{Affine3A, Vec3A};

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

    pub fn transform(&self, transformation: Affine3A) -> Self {
        // TODO: check if right
        Self {
            origin_point: transformation.transform_point3a(self.origin_point),
            direction_vector: transformation.transform_vector3a(self.direction_vector),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use glam::Vec3;
    use rstest::*;

    use super::*;

    #[fixture]
    pub fn ray() -> Ray {
        Ray::new(Vec3A::new(1.0, 2.0, 3.0), Vec3A::new(4.0, 5.0, 6.0))
    }

    #[fixture]
    pub fn ray2() -> Ray {
        Ray::new(Vec3A::new(1.0, 2.0, 3.0), Vec3A::new(0.0, 1.0, 0.0))
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
            .abs_diff_eq(expected_point, f32::EPSILON));
    }

    #[rstest]
    fn can_translate_a_ray(ray2: Ray) {
        let translation = Affine3A::from_translation(Vec3::new(3.0, 4.0, 5.0));
        let transformed = ray2.transform(translation);
        assert!(transformed
            .origin_point
            .abs_diff_eq(Vec3A::new(4.0, 6.0, 8.0), f32::EPSILON));
        assert!(transformed
            .direction_vector
            .abs_diff_eq(Vec3A::new(0.0, 1.0, 0.0), f32::EPSILON));
    }

    #[rstest]
    fn can_scale_a_ray(ray2: Ray) {
        let scale = Affine3A::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let transformed = ray2.transform(scale);
        assert!(transformed
            .origin_point
            .abs_diff_eq(Vec3A::new(2.0, 6.0, 12.0), f32::EPSILON));
        assert!(transformed
            .direction_vector
            .abs_diff_eq(Vec3A::new(0.0, 3.0, 0.0), f32::EPSILON));
    }
}
