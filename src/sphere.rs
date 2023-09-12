use crate::{intersection::Intersection, ray::Ray};
use glam::Vec3A;
use std::sync::atomic::{AtomicU64, Ordering};

// TODO: Maybe a better solution for sphere id? They need to be unique and UUID seems excessive
static SPHERE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub id: String,
    sphere_center_point: Vec3A,
}

impl Sphere {
    pub fn new(sphere_center_point: Vec3A) -> Self {
        let sphere_id = format!(
            "sphere-{}",
            SPHERE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
        );

        Self {
            sphere_center_point,
            id: sphere_id,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin_point - self.sphere_center_point;
        let a = ray.direction_vector.dot(ray.direction_vector);
        let b = 2.0 * ray.direction_vector.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i1 = Intersection {
                object_id: &self.id,
                t: t1,
            };
            let i2 = Intersection {
                object_id: &self.id,
                t: t2,
            };
            let mut sphere_intersections = vec![i1, i2];

            // Return in increasing order, the smallest value first
            sphere_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            sphere_intersections
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn multiple_spheres_have_unique_ids() {
        let s1 = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let s2 = Sphere::new(Vec3A::new(1.0, 1.0, 1.0));
        let s3 = Sphere::new(Vec3A::new(2.0, 2.0, 2.0));
        assert_ne!(s1.id, s2.id);
        assert_ne!(s1.id, s3.id);
        assert_ne!(s2.id, s3.id);
        assert!(s1.id.starts_with("sphere-"));
        assert!(s2.id.starts_with("sphere-"));
        assert!(s3.id.starts_with("sphere-"));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.len(), 2);
        assert_abs_diff_eq!(intersection.first().unwrap().t, 4.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(intersection.get(1).unwrap().t, 6.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = Ray::new(Vec3A::new(0.0, 1.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.len(), 2);
        assert_abs_diff_eq!(intersection.first().unwrap().t, 5.0, epsilon = f32::EPSILON);
        assert_abs_diff_eq!(intersection.get(1).unwrap().t, 5.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray::new(Vec3A::new(0.0, 2.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.len(), 2);
        assert_abs_diff_eq!(
            intersection.first().unwrap().t,
            -1.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(intersection.get(1).unwrap().t, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, 5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.len(), 2);
        assert_abs_diff_eq!(
            intersection.first().unwrap().t,
            -6.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(intersection.get(1).unwrap().t, -4.0, epsilon = f32::EPSILON);
    }
}
