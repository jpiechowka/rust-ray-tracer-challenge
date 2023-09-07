const MARGIN: f64 = 0.00001;

#[inline(always)]
pub fn compare_f64_with_margin(a: f64, b: f64) -> bool {
    (a - b).abs() < MARGIN
}
