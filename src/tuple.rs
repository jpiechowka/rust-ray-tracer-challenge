#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: i32,
}

impl Tuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_equal_to_1_is_a_point() {
        let tuple = Tuple::new_point(1.3, -4.4, 7.0);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
        assert_eq!(tuple.x, 1.3);
        assert_eq!(tuple.y, -4.4);
        assert_eq!(tuple.z, 7.0);
        assert_eq!(tuple.w, 1);
    }

    #[test]
    fn a_tuple_with_w_equal_to_0_is_a_vector() {
        let tuple = Tuple::new_vector(1.3, -4.4, 7.0);
        assert!(tuple.is_vector());
        assert!(!tuple.is_point());
        assert_eq!(tuple.x, 1.3);
        assert_eq!(tuple.y, -4.4);
        assert_eq!(tuple.z, 7.0);
        assert_eq!(tuple.w, 0);
    }
}
