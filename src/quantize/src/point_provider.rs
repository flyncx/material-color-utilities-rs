pub trait PointProvider {
    fn from_int(argb: i64) -> Vec<f64>;
    fn to_int(point: Vec<f64>) -> i64;
    fn distance(a: Vec<f64>, b: Vec<f64>) -> f64;
}
