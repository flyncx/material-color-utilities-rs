use crate::quantize::{
    quantizer_wsmeans::QuantizerWsmeans, quantizer_wu::QuantizerWu,
    src::point_provider_lab::PointProviderLab,
};

use super::quantizer::{Quantizer, QuantizerResult};

pub struct QuantizerCelebi {}
impl Quantizer for QuantizerCelebi {
    fn quantize(
        &mut self,
        pixels: Vec<i64>,
        max_colors: i64,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        let return_input_pixel_to_cluster_pixel =
            return_input_pixel_to_cluster_pixel.unwrap_or(false);
        let mut wu = QuantizerWu::new();
        let wu_result = wu.quantize(pixels.clone(), max_colors, None);
        let wsmeans_result = QuantizerWsmeans::quantize(
            pixels,
            max_colors,
            Some(wu_result.color_to_count.keys().cloned().collect()),
            Some(PointProviderLab::new()),
            None,
            Some(return_input_pixel_to_cluster_pixel),
        );
        return wsmeans_result;
    }
}
