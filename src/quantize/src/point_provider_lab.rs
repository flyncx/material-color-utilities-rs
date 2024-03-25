use crate::utils::color_utils::ColorUtils;

use super::point_provider::PointProvider;

pub struct PointProviderLab {}
impl PointProvider for PointProviderLab {
    fn from_int(argb: i64) -> Vec<f64> {
        return ColorUtils::lab_from_argb(argb);
    }

    fn to_int(lab: Vec<f64>) -> i64 {
        ColorUtils::argb_from_lab(lab[0], lab[1], lab[2])
    }

    fn distance(one: Vec<f64>, two: Vec<f64>) -> f64 {
        let d_l = one[0] - two[0];
        let d_a = one[1] - two[1];
        let d_b = one[2] - two[2];
        // Standard CIE 1976 delta E formula also takes the square root, unneeded
        // here. This method is used by quantization algorithms to compare distance,
        // and the relative ordering is the same, with or without a square root.

        // This relatively minor optimization is helpful because this method is
        // called at least once for each pixel in an image.
        return d_l * d_l + d_a * d_a + d_b * d_b;
    }
}
