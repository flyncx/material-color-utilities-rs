use crate::{
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
};

/// A Dynamic Color theme that is grayscale.
pub struct SchemeMonochrome {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeMonochrome {
    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeMonochrome {
        SchemeMonochrome {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Monochrome,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
                TonalPalette::of(source_color_hct.get_hue(), 0.0),
            ),
        }
    }
}
