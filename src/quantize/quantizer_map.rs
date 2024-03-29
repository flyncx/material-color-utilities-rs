use std::collections::HashMap;

use crate::utils::color_utils::ColorUtils;

use super::quantizer::{Quantizer, QuantizerResult};

pub struct QuantizerMap {}
impl Quantizer for QuantizerMap {
    fn quantize(
        &mut self,
        pixels: &Vec<i64>,
        _max_colors: i64,
        _: Option<bool>,
    ) -> QuantizerResult {
        let mut count_by_color: HashMap<i64, i64> = HashMap::new();
        for pixel in pixels {
            let alpha = ColorUtils::alpha_from_argb(*pixel);
            if alpha < 255 {
                continue;
            }
            count_by_color.insert(*pixel, count_by_color.get(&pixel).unwrap_or(&0) + 1);
        }
        QuantizerResult::new(count_by_color, None)
    }
}
