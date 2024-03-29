pub trait PointProvider {
    fn from_int(&self, argb: i64) -> Vec<f64>;
    fn to_int(&self, point: &Vec<f64>) -> i64;
    fn distance(&self, a: &Vec<f64>, b: &Vec<f64>) -> f64;
}
