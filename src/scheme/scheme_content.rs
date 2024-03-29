use crate::{
    dislike::dislike_analyzer::DislikeAnalyzer,
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
    temperature::temperature_cache::TemperatureCache,
};

/// A scheme that places the source color in [Scheme.primaryContainer].
///
/// Primary Container is the source color, adjusted for color relativity.
/// It maintains constant appearance in light mode and dark mode.
/// This adds ~5 tone in light mode, and subtracts ~5 tone in dark mode.
///
/// Tertiary Container is an analogous color, specifically, the analog of a
/// color wheel divided into 6, and the precise analog is the one found by
/// increasing hue. It also maintains constant appearance.
pub struct SchemeContent {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeContent {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeContent {
        let binding = TemperatureCache::new(source_color_hct.clone()).analogous(Some(3), Some(6));
        let temp = binding.last().unwrap();
        SchemeContent {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Content,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(source_color_hct.get_hue(), source_color_hct.get_chroma()),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    (source_color_hct.get_chroma() - 32.0).max(source_color_hct.get_chroma() * 0.5),
                ),
                TonalPalette::from_hct(&DislikeAnalyzer::fix_if_disliked(temp)),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    source_color_hct.get_chroma() / 8.0,
                ),
                TonalPalette::of(
                    source_color_hct.get_hue(),
                    (source_color_hct.get_chroma() / 8.0) + 4.0,
                ),
            ),
        }
    }
}
