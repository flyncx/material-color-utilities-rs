/* use crate::{
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
    utils::math_utils::MathUtils,
};

/// A Dynamic Color theme with low to medium colorfulness and a Tertiary
/// [TonalPalette] with a hue related to the source color. The default
/// Material You theme on Android 12 and 13.
pub struct SchemeTonalSpot {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeTonalSpot {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeTonalSpot {
        SchemeTonalSpot {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::TonalSpot,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(source_color_hct.get_hue(), 36.0),
                TonalPalette::of(source_color_hct.get_hue(), 16.0),
                TonalPalette::of(
                    MathUtils::sanitize_degrees_double(source_color_hct.get_hue() + 60.0),
                    24.0,
                ),
                TonalPalette::of(source_color_hct.get_hue(), 6.0),
                TonalPalette::of(source_color_hct.get_hue(), 8.0),
            ),
        }
    }
}
 */
