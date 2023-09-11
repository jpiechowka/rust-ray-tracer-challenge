#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    pub t1: Option<f32>,
    pub t2: Option<f32>,
    pub object_id: &'a String,
}
