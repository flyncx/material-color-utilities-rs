use std::collections::HashMap;

pub trait Quantizer {
    fn quantize(
        &mut self,
        pixels: Vec<i64>,
        max_colors: i64,
        returnInputPixelToClusterPixel: Option<bool>,
    ) -> QuantizerResult;
}

pub struct QuantizerResult {
    pub color_to_count: HashMap<i64, i64>,
    pub input_pixel_to_cluster_pixel: HashMap<i64, i64>,
}
impl QuantizerResult {
    pub fn new(
        color_to_count: HashMap<i64, i64>,
        input_pixel_to_cluster_pixel: Option<HashMap<i64, i64>>,
    ) -> QuantizerResult {
        let input_pixel_to_cluster_pixel = input_pixel_to_cluster_pixel.unwrap_or(HashMap::new());
        QuantizerResult {
            color_to_count,
            input_pixel_to_cluster_pixel,
        }
    }
}
