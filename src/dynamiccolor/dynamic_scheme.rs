use std::hash::Hash;

use crate::{hct::hct::Hct, palettes::tonal_palette::TonalPalette, utils::math_utils::MathUtils};

use super::{
    dynamic_color::DynamicColor, material_dynamic_colors::MaterialDynamicColors, variant::Variant,
};

/// Constructed by a set of values representing the current UI state (such as
/// whether or not its dark theme, what the theme style is, etc.), and
/// provides a set of [TonalPalette]s that can create colors that fit in
/// with the theme style. Used by [DynamicColor] to resolve into a color.\
#[derive(Clone)]
pub struct DynamicScheme {
    /// The source color of the theme as an ARGB integer.
    pub source_color_argb: i64,

    /// The source color of the theme in HCT.
    pub source_color_hct: Hct,

    /// The variant, or style, of the theme.
    pub variant: Variant,

    /// Whether or not the scheme is in 'dark mode' or 'light mode'.
    pub is_dark: bool,

    /// Value from -1 to 1. -1 represents minimum contrast, 0 represents
    /// standard (i.e. the design as spec'd), and 1 represents maximum contrast.
    pub contrast_level: f64,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually colorful.
    pub primary_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually less colorful.
    pub secondary_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually a different hue from
    /// primary and colorful.
    pub tertiary_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually not colorful at all,
    /// intended for background & surface colors.
    pub neutral_palette: TonalPalette,

    /// Given a tone, produces a color. Hue and chroma of the color are specified
    /// in the design specification of the variant. Usually not colorful, but
    /// slightly more colorful than Neutral. Intended for backgrounds & surfaces.
    pub neutral_variant_palette: TonalPalette,

    /// Given a tone, produces a reddish, colorful, color.
    pub error_palette: TonalPalette,
}
impl PartialEq for DynamicScheme {
    fn eq(&self, other: &Self) -> bool {
        self.source_color_argb == other.source_color_argb
            && self.source_color_hct == other.source_color_hct
            && self.variant == other.variant
            && self.is_dark == other.is_dark
            && self.contrast_level == other.contrast_level
            && self.primary_palette == other.primary_palette
            && self.secondary_palette == other.secondary_palette
            && self.tertiary_palette == other.tertiary_palette
            && self.neutral_palette == other.neutral_palette
            && self.neutral_variant_palette == other.neutral_variant_palette
            && self.error_palette == other.error_palette
    }
}
impl Eq for DynamicScheme {}
impl Hash for DynamicScheme {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source_color_argb.hash(state);
        self.source_color_hct.hash(state);
        self.variant.hash(state);
        self.is_dark.hash(state);
        self.contrast_level.to_ne_bytes().hash(state);
        self.primary_palette.hash(state);
        self.secondary_palette.hash(state);
        self.tertiary_palette.hash(state);
        self.neutral_palette.hash(state);
        self.neutral_variant_palette.hash(state);
        self.error_palette.hash(state);
    }
}

impl DynamicScheme {
    pub fn new(
        source_color_argb: i64,
        variant: Variant,
        contrast_level: Option<f64>,
        is_dark: bool,
        primary_palette: TonalPalette,
        secondary_palette: TonalPalette,
        tertiary_palette: TonalPalette,
        neutral_palette: TonalPalette,
        neutral_variant_palette: TonalPalette,
    ) -> Self {
        Self {
            source_color_argb,
            source_color_hct: Hct::from_int(source_color_argb),
            variant,
            is_dark,
            contrast_level: contrast_level.unwrap_or(0.0),
            primary_palette,
            secondary_palette,
            tertiary_palette,
            neutral_palette,
            neutral_variant_palette,
            error_palette: TonalPalette::of(25.0, 84.0),
        }
    }

    pub fn get_rotated_hue(source_color: &Hct, hues: &Vec<f64>, rotations: &Vec<f64>) -> f64 {
        let source_hue = source_color.get_hue();
        assert!(hues.len() == rotations.len());
        if rotations.len() == 1 {
            return MathUtils::sanitize_degrees_double(source_color.get_hue() + rotations[0]);
        }
        let size: i64 = hues.len() as i64;
        let mut i: i64 = 0;
        while i <= (size - 2) {
            let this_hue = hues[i as usize];
            let next_hue = hues[i as usize + 1];
            if this_hue < source_hue && source_hue < next_hue {
                return MathUtils::sanitize_degrees_double(source_hue + rotations[i as usize]);
            }
            i += 1;
        }
        // If this statement executes, something is wrong, there should have been a rotation
        // found using the arrays.
        return source_hue;
    }

    pub fn get_hct(&self, dynamic_color: &mut DynamicColor) -> Hct {
        dynamic_color.get_hct(self)
    }
    pub fn get_argb(&self, dynamic_color: &mut DynamicColor) -> i64 {
        dynamic_color.get_argb(self)
    }

    // Getters.

    pub fn get_surface_dim(&self) -> i64 {
        self.get_argb(&mut MaterialDynamicColors::surface_dim())
    }
    pub fn get_surface_bright(&self) -> i64 {
        self.get_argb(&mut MaterialDynamicColors::surface_bright())
    }
    pub fn get_primary(&self) -> i64 {
        self.get_argb(&mut MaterialDynamicColors::primary())
    }
    pub fn get_primary_container(&self) -> i64 {
        self.get_argb(&mut MaterialDynamicColors::primary_container())
    }
}
