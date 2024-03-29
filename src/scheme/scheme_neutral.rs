/* use crate::{
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
};

/// A Dynamic Color theme that is near grayscale.
pub struct SchemeNeutral {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeNeutral {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeNeutral {
        SchemeNeutral {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Neutral,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(source_color_hct.get_hue(), 12.0),
                TonalPalette::of(source_color_hct.get_hue(), 8.0),
                TonalPalette::of(source_color_hct.get_hue(), 16.0),
                TonalPalette::of(source_color_hct.get_hue(), 2.0),
                TonalPalette::of(source_color_hct.get_hue(), 2.0),
            ),
        }
    }
}
 */
