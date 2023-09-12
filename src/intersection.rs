#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    pub t: Vec<f32>,
    pub object_id: &'a String,
}

impl<'a> Intersection<'a> {
    pub fn new(t_vec: Vec<f32>, object_id: &'a String) -> Self {
        Self {
            t: t_vec,
            object_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::Sphere;

    use super::*;
    use glam::Vec3A;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new(Vec3A::new(0.0, 0.0, 0.0));
        let intersection = Intersection::new(vec![3.5], &sphere.id);
        assert_eq!(intersection.t.len(), 1);
        assert_eq!(*intersection.t.first().unwrap(), 3.5);
        assert_eq!(intersection.object_id, &sphere.id);
    }
}
