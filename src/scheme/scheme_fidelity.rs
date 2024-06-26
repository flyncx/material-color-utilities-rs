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
/// Tertiary Container is the complement to the source color, using
/// [TemperatureCache]. It also maintains constant appearance.
pub struct SchemeFidelity {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeFidelity {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeFidelity {
        let hue_of_source_color = source_color_hct.get_hue();
        let chroma_of_source_color = source_color_hct.get_chroma();
        SchemeFidelity {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Fidelity,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(hue_of_source_color, chroma_of_source_color),
                TonalPalette::of(
                    hue_of_source_color,
                    (chroma_of_source_color - 32.0).max(chroma_of_source_color * 0.5),
                ),
                TonalPalette::from_hct(&DislikeAnalyzer::fix_if_disliked(
                    &TemperatureCache::new(source_color_hct).get_complement(),
                )),
                TonalPalette::of(hue_of_source_color, chroma_of_source_color / 8.0),
                TonalPalette::of(hue_of_source_color, (chroma_of_source_color / 8.0) + 4.0),
            ),
        }
    }
}
