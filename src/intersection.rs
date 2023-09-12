#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object_id: &'a String,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object_id: &'a String) -> Self {
        Self { t, object_id }
    }
}

pub fn aggregate_and_sort_intersections_in_place(intersections: &mut [Intersection]) {
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, sphere::Sphere};

    use super::*;
    use approx::assert_abs_diff_eq;
    use glam::Vec3A;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = Intersection::new(3.5, &sphere.id);
        assert_abs_diff_eq!(intersection.t, 3.5);
        assert_eq!(intersection.object_id, &sphere.id);
    }

    #[test]
    fn can_aggregate_intersections() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let i1 = Intersection::new(2.0, &sphere.id);
        let i2 = Intersection::new(1.0, &sphere.id);
        let mut aggregated_intersections = vec![i1, i2];
        aggregate_and_sort_intersections_in_place(&mut aggregated_intersections);
        assert_eq!(aggregated_intersections.len(), 2);
        assert_abs_diff_eq!(aggregated_intersections.first().unwrap().t, 1.0);
        assert_abs_diff_eq!(aggregated_intersections.get(1).unwrap().t, 2.0);
        assert_eq!(i1.object_id, &sphere.id);
        assert_eq!(i2.object_id, &sphere.id);
    }

    #[test]
    fn intersect_sets_the_object_id_on_the_intersection() {
        let r1 = Ray::new(Vec3A::new(0.0, 1.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let r2 = Ray::new(Vec3A::new(0.0, 0.0, -5.0), Vec3A::new(0.0, 0.0, 1.0));
        let s1 = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let s2 = Sphere::new(Vec3A::new(0.0, 1.0, 0.0));
        let i1 = s1.intersect(r1);
        let i2 = s2.intersect(r2);
        assert_eq!(i1.len(), 2);
        assert_eq!(i1.first().unwrap().object_id, &s1.id);
        assert_eq!(i1.get(1).unwrap().object_id, &s1.id);
        assert_eq!(i2.len(), 2);
        assert_eq!(i2.first().unwrap().object_id, &s2.id);
        assert_eq!(i2.get(1).unwrap().object_id, &s2.id);
    }
}
