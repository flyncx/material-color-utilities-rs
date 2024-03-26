/*
class QuantizerMap implements Quantizer {
  @override
  Future<QuantizerResult> quantize(Iterable<int> pixels, int maxColors) async {
    final countByColor = <int, int>{};
    for (final pixel in pixels) {
      final alpha = ColorUtils.alphaFromArgb(pixel);
      if (alpha < 255) {
        continue;
      }
      countByColor[pixel] = (countByColor[pixel] ?? 0) + 1;
    }
    return QuantizerResult(countByColor);
  }
}
 */

use std::collections::HashMap;

use crate::utils::color_utils::ColorUtils;

use super::quantizer::{Quantizer, QuantizerResult};

pub struct QuantizerMap {}
impl Quantizer for QuantizerMap {
    fn quantize(pixels: Vec<i64>, _max_colors: i64) -> QuantizerResult {
        let mut count_by_color: HashMap<i64, i64> = HashMap::new();
        for pixel in pixels {
            let alpha = ColorUtils::alpha_from_argb(pixel);
            if alpha < 255 {
                continue;
            }
            count_by_color.insert(pixel, count_by_color.get(&pixel).unwrap_or(&0) + 1);
        }
        QuantizerResult::new(count_by_color, None)
    }
}
