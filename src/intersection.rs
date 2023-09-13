use rayon::{
    prelude::{IntoParallelRefIterator, ParallelExtend, ParallelIterator},
    slice::ParallelSliceMut,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SingleIntersection<'a> {
    pub t: f32,
    pub object_id: &'a String,
}

impl<'a> SingleIntersection<'a> {
    pub fn new(t: f32, object_id: &'a String) -> Self {
        Self { t, object_id }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersections<'a> {
    pub i: Vec<SingleIntersection<'a>>,
}

// TODO: Do we need to sort this many times?
impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<SingleIntersection<'a>>) -> Self {
        let mut intersections = Self { i: intersections };
        intersections.sort_intersections();
        intersections
    }

    pub fn aggregate_and_sort(&mut self, other_intersections: Vec<SingleIntersection<'a>>) {
        self.i.par_extend(other_intersections);
        self.sort_intersections();
    }

    /// Hit returns lowest non-negative intersection. Assumes that intersections are sorted.
    pub fn hit(&self) -> Option<&SingleIntersection> {
        self.i.par_iter().find_first(|i| i.t >= 0.0)
    }

    fn sort_intersections(&mut self) {
        self.i.par_sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use glam::Vec3A;

    use crate::{ray::Ray, sphere::Sphere};

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = SingleIntersection::new(3.5, &sphere.id);
        assert_abs_diff_eq!(intersection.t, 3.5);
        assert_eq!(intersection.object_id, &sphere.id);
    }

    #[test]
    fn can_aggregate_intersections() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let i1 = SingleIntersection::new(2.0, &sphere.id);
        let i2 = SingleIntersection::new(1.0, &sphere.id);
        let mut intersections = Intersections::new(vec![i1, i2, i1]);
        intersections.aggregate_and_sort(vec![i2, i1, i2]);
        assert_eq!(intersections.i.len(), 6);
        assert_abs_diff_eq!(intersections.i.first().unwrap().t, 1.0);
        assert_abs_diff_eq!(intersections.i.get(1).unwrap().t, 1.0);
        assert_abs_diff_eq!(intersections.i.last().unwrap().t, 2.0);
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

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let i1 = SingleIntersection::new(1.0, &sphere.id);
        let i2 = SingleIntersection::new(2.0, &sphere.id);
        let intersections = Intersections::new(vec![i1, i2]);
        let hit = intersections.hit();
        assert_eq!(*hit.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_positive_t() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let i1 = SingleIntersection::new(-1.0, &sphere.id);
        let i2 = SingleIntersection::new(1.0, &sphere.id);
        let intersections = Intersections::new(vec![i1, i2]);
        let hit = intersections.hit();
        assert_eq!(*hit.unwrap(), i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let i1 = SingleIntersection::new(-2.0, &sphere.id);
        let i2 = SingleIntersection::new(-1.0, &sphere.id);
        let intersections = Intersections::new(vec![i1, i2]);
        let hit = intersections.hit();
        assert!(hit.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let i1 = SingleIntersection::new(5.0, &sphere.id);
        let i2 = SingleIntersection::new(7.0, &sphere.id);
        let i3 = SingleIntersection::new(-3.0, &sphere.id);
        let i4 = SingleIntersection::new(2.0, &sphere.id);
        let mut intersections = Intersections::new(vec![i1, i2]);
        intersections.aggregate_and_sort(vec![i3, i4]);
        let hit = intersections.hit();
        assert_eq!(*hit.unwrap(), i4);
    }
}
