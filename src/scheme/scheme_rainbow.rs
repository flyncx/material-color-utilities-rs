/* use crate::{
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
    utils::math_utils::MathUtils,
};

/// A playful theme - the source color's hue does not appear in the theme.
pub struct SchemeRainbow {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeRainbow {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeRainbow {
        SchemeRainbow {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Rainbow,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(source_color_hct.get_hue(), 48.0),
                TonalPalette::of(source_color_hct.get_hue(), 16.0),
                TonalPalette::of(
                    MathUtils::sanitize_degrees_double(source_color_hct.get_hue() + 60.0),
                    24.0,
                ),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
            ),
        }
    }
}
 */
