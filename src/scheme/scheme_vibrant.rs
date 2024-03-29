/* use crate::{
    dynamiccolor::{dynamic_scheme::DynamicScheme, variant::Variant},
    hct::hct::Hct,
    palettes::tonal_palette::TonalPalette,
};

/// A Dynamic Color theme that maxes out colorfulness at each position in the
/// Primary [TonalPalette].
pub struct SchemeVibrant {
    pub dynamic_scheme: DynamicScheme,
}
impl SchemeVibrant {
    /// Hues used at breakpoints such that designers can specify a hue rotation
    /// that occurs at a given break point.
    const HUES: [f64; 9] = [0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];

    /// Hue rotations of the Secondary [TonalPalette], corresponding to the
    /// breakpoints in [hues].
    const SECONDARY_ROTATIONS: [f64; 9] = [18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0];

    /// Hue rotations of the Tertiary [TonalPalette], corresponding to the
    /// breakpoints in [hues].
    const TERTIARY_ROTATIONS: [f64; 9] = [35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0];

    pub fn new(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> SchemeVibrant {
        SchemeVibrant {
            dynamic_scheme: DynamicScheme::new(
                source_color_hct.to_int(),
                Variant::Vibrant,
                Some(contrast_level),
                is_dark,
                TonalPalette::of(source_color_hct.get_hue(), 200.0),
                TonalPalette::of(
                    DynamicScheme::get_rotated_hue(
                        source_color_hct.clone(),
                        Self::HUES.to_vec(),
                        Self::SECONDARY_ROTATIONS.to_vec(),
                    ),
                    24.0,
                ),
                TonalPalette::of(
                    DynamicScheme::get_rotated_hue(
                        source_color_hct.clone(),
                        Self::HUES.to_vec(),
                        Self::TERTIARY_ROTATIONS.to_vec(),
                    ),
                    32.0,
                ),
                TonalPalette::of(source_color_hct.get_hue(), 10.0),
                TonalPalette::of(source_color_hct.get_hue(), 12.0),
            ),
        }
    }
}
 */
