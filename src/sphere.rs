use std::sync::atomic::{AtomicU64, Ordering};

use glam::{Affine3A, Mat4, Vec3A, Vec4};

use crate::intersection::Intersections;
use crate::material::Material;
use crate::{intersection::SingleIntersection, ray::Ray};

// TODO: Maybe a better solution for sphere id? They need to be unique and UUID seems excessive
static SPHERE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub id: String,
    sphere_center_point: Vec3A,
    transform: Affine3A,
    material: Material,
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
            transform: Affine3A::default(),
            material: Material::default(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray_inverse = ray.transform(self.transform.inverse());
        let sphere_to_ray = ray_inverse.origin_point - self.sphere_center_point;
        let a = ray_inverse
            .direction_vector
            .dot(ray_inverse.direction_vector);
        let b = 2.0 * ray_inverse.direction_vector.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i1 = SingleIntersection {
                object_id: &self.id,
                t: t1,
            };
            let i2 = SingleIntersection {
                object_id: &self.id,
                t: t2,
            };

            Intersections::new(vec![i1, i2])
        }
    }

    // TODO: improve? Now the implementation follows the book implementation
    pub fn normal_at(&self, world_point: Vec3A) -> Vec3A {
        let sphere_transform = Mat4::from(self.transform);
        let object_point = sphere_transform.inverse() * Vec4::from((world_point, 1.0));
        let object_normal = self.normal_at_in_object_space(Vec3A::from(object_point));
        let world_normal =
            sphere_transform.inverse().transpose() * Vec4::from((object_normal, 0.0));
        Vec3A::from(world_normal).normalize()
    }

    pub fn set_transform(&mut self, transform: Affine3A) {
        self.transform = transform
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }

    fn normal_at_in_object_space(&self, object_space_point: Vec3A) -> Vec3A {
        (object_space_point - self.sphere_center_point).normalize()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    use approx::assert_abs_diff_eq;
    use glam::Vec3;

    use crate::color::Color;

    use super::*;

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
        assert_eq!(intersection.i.len(), 2);
        assert_abs_diff_eq!(
            intersection.i.first().unwrap().t,
            4.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            intersection.i.get(1).unwrap().t,
            6.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = Ray::new(Vec3A::new(0.0, 1.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.i.len(), 2);
        assert_abs_diff_eq!(
            intersection.i.first().unwrap().t,
            5.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            intersection.i.get(1).unwrap().t,
            5.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray::new(Vec3A::new(0.0, 2.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.i.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.i.len(), 2);
        assert_abs_diff_eq!(
            intersection.i.first().unwrap().t,
            -1.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            intersection.i.get(1).unwrap().t,
            1.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, 5.0), Vec3A::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.i.len(), 2);
        assert_abs_diff_eq!(
            intersection.i.first().unwrap().t,
            -6.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            intersection.i.get(1).unwrap().t,
            -4.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn new_sphere_has_default_transform() {
        let sphere = Sphere::new(Vec3A::new(1.0, 2.0, 3.0));
        assert_eq!(sphere.transform, Affine3A::IDENTITY);
    }

    #[test]
    fn can_change_spheres_transformation() {
        let mut sphere = Sphere::new(Vec3A::new(1.0, 2.0, 3.0));
        let translation = Affine3A::from_translation(Vec3::new(2.0, 3.0, 4.0));
        sphere.set_transform(translation);
        assert_eq!(sphere.transform, translation);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let mut sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let scaling = Affine3A::from_scale(Vec3::new(2.0, 2.0, 2.0));
        sphere.set_transform(scaling);
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.i.len(), 2);
        assert_abs_diff_eq!(
            intersection.i.first().unwrap().t,
            3.0,
            epsilon = f32::EPSILON
        );
        assert_abs_diff_eq!(
            intersection.i.get(1).unwrap().t,
            7.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let ray = Ray::new(Vec3A::new(0.0, 0.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let mut sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let translation = Affine3A::from_translation(Vec3::new(5.0, 0.0, 0.0));
        sphere.set_transform(translation);
        let intersection = sphere.intersect(ray);
        assert_eq!(intersection.i.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Sphere::new(Vec3A::splat(0.0));
        assert_eq!(
            sphere.normal_at(Vec3A::new(1.0, 0.0, 0.0)),
            Vec3A::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Sphere::new(Vec3A::splat(0.0));
        assert_eq!(
            sphere.normal_at(Vec3A::new(0.0, 1.0, 0.0)),
            Vec3A::new(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Sphere::new(Vec3A::splat(0.0));
        assert_eq!(
            sphere.normal_at(Vec3A::new(0.0, 0.0, 1.0)),
            Vec3A::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Sphere::new(Vec3A::splat(0.0));
        let f = 3_f32.sqrt() / 3_f32;
        assert!(sphere
            .normal_at(Vec3A::new(f, f, f))
            .abs_diff_eq(Vec3A::new(f, f, f), f32::EPSILON));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sphere = Sphere::new(Vec3A::splat(0.0));
        let f = 3_f32.sqrt() / 3_f32;
        let normal = sphere.normal_at(Vec3A::new(f, f, f));
        assert!(normal.abs_diff_eq(normal.normalize(), f32::EPSILON));
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut sphere = Sphere::new(Vec3A::splat(0.0));
        sphere.set_transform(Affine3A::from_translation(Vec3::new(0.0, 1.0, 0.0)));
        let normal = sphere.normal_at(Vec3A::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert!(normal.abs_diff_eq(Vec3A::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2), f32::EPSILON));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut sphere = Sphere::new(Vec3A::splat(0.0));
        let scale = Affine3A::from_scale(Vec3::new(1.0, 0.5, 1.0));
        let rotation = Affine3A::from_rotation_z(PI / 5_f32);
        let transform = scale * rotation;
        sphere.set_transform(transform);
        let normal = sphere.normal_at(Vec3A::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        println!("{:?}", normal);
        // TODO: Cannot use f32::EPSILON as the difference is too big
        assert!(normal.abs_diff_eq(Vec3A::new(0.0, 0.97014, -0.24254), 0.00001));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let sphere = Sphere::new(Vec3A::splat(0.0));
        assert_eq!(sphere.material, Material::default());
    }

    #[test]
    fn can_assign_material_to_sphere() {
        let custom_material = Material::new(Color::new_red(), 0.2, 0.4, 0.6, 300.0);
        let mut sphere = Sphere::new(Vec3A::splat(0.0));
        sphere.set_material(custom_material);
        assert_eq!(sphere.material, custom_material);
    }
}
