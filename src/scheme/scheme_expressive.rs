use crate::{
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
    utils::math_utils::MathUtils,
};

/// A Dynamic Color theme that is intentionally detached from the input color.
pub struct SchemeExpressive {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeExpressive {
    /// Hues used at breakpoints such that designers can specify a hue rotation
    /// that occurs at a given break point.
    const HUES: [f64; 9] = [0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0];

    /// Hue rotations of the Secondary [TonalPalette], corresponding to the
    /// breakpoints in [hues].
    const SECONDARY_ROTATIONS: [f64; 9] = [45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0];

    /// Hue rotations of the Tertiary [TonalPalette], corresponding to the
    /// breakpoints in [hues].
    const TERTIARY_ROTATIONS: [f64; 9] = [120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0];

    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeExpressive {
        SchemeExpressive {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Expressive,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(
                    MathUtils::sanitize_degrees_double(source_color_hct.get_hue() + 240.0),
                    40.0,
                ),
                TonalPalette::of(
                    DynamicScheme::get_rotated_hue(
                        &source_color_hct,
                        &Self::HUES.to_vec(),
                        &Self::SECONDARY_ROTATIONS.to_vec(),
                    ),
                    24.0,
                ),
                TonalPalette::of(
                    DynamicScheme::get_rotated_hue(
                        &source_color_hct,
                        &Self::HUES.to_vec(),
                        &Self::TERTIARY_ROTATIONS.to_vec(),
                    ),
                    32.0,
                ),
                TonalPalette::of(source_color_hct.get_hue() + 15.0, 8.0),
                TonalPalette::of(source_color_hct.get_hue() + 15.0, 12.0),
            ),
        }
    }
}
