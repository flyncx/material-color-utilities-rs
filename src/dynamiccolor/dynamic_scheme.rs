use crate::{hct::hct::Hct, palettes::tonal_palette::TonalPalette, utils::math_utils::MathUtils};

use super::{
    dynamic_color::DynamicColor, material_dynamic_colors::MaterialDynamicColors, variant::Variant,
};
use core::hash::Hash;
use std::hash::Hasher;

/// Constructed by a set of values representing the current UI state (such as
/// whether or not its dark theme, what the theme style is, etc.), and
/// provides a set of [TonalPalette]s that can create colors that fit in
/// with the theme style. Used by [DynamicColor] to resolve into a color.
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
    ) -> DynamicScheme {
        let contrast_level = contrast_level.unwrap_or(0.0);
        DynamicScheme {
            source_color_argb,
            source_color_hct: Hct::from_int(source_color_argb),
            variant,
            is_dark,
            contrast_level,
            primary_palette,
            secondary_palette,
            tertiary_palette,
            neutral_palette,
            neutral_variant_palette,
            error_palette: TonalPalette::of(25.0, 84.0),
        }
    }

    pub fn get_rotated_hue(source_color: Hct, hues: Vec<f64>, rotations: Vec<f64>) -> f64 {
        let source_hue = source_color.get_hue();
        assert!(hues.len() == rotations.len());
        if rotations.len() == 1 {
            return MathUtils::sanitize_degrees_double(source_color.get_hue() + rotations[0]);
        }
        let size = hues.len();
        for i in 0..(size - 2) {
            let this_hue = hues[i];
            let next_hue = hues[i + 1];
            if this_hue < source_hue && source_hue < next_hue {
                return MathUtils::sanitize_degrees_double(source_hue + rotations[i]);
            }
        }
        // If this statement executes, something is wrong, there should have been a rotation
        // found using the arrays.
        return source_hue;
    }

    // Getters.
    pub fn get_hct(&self, mut dynamic_color: DynamicColor) -> Hct {
        dynamic_color.get_hct(self.clone())
    }
    pub fn get_argb(&self, mut dynamic_color: DynamicColor) -> i64 {
        dynamic_color.get_argb(self.clone())
    }
    pub fn get_primary_palette_key_color(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::primary_palette_key_color())
    }
    pub fn get_secondary_palette_key_color(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::secondary_palette_key_color())
    }
    pub fn get_tertiary_palette_key_color(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::tertiary_palette_key_color())
    }
    pub fn get_neutral_palette_key_color(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::neutral_palette_key_color())
    }
    pub fn get_neutral_variant_palette_key_color(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::neutral_variant_palette_key_color())
    }
    pub fn get_background(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::background())
    }
    pub fn get_on_background(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_background())
    }
    pub fn get_surface(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface())
    }
    pub fn get_surface_dim(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_dim())
    }
    pub fn get_surface_bright(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_bright())
    }
    pub fn get_surface_container_lowest(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_container_lowest())
    }
    pub fn get_surface_container_low(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_container_low())
    }
    pub fn get_surface_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_container())
    }
    pub fn get_surface_container_high(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_container_high())
    }
    pub fn get_surface_container_highest(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_container_highest())
    }
    pub fn get_on_surface(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_surface(&MaterialDynamicColors {}))
    }
    pub fn get_surface_variant(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_variant())
    }
    pub fn get_on_surface_variant(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_surface_variant(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_inverse_surface(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::inverse_surface())
    }
    pub fn get_inverse_on_surface(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::inverse_on_surface())
    }
    pub fn get_outline(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::outline(&MaterialDynamicColors {}))
    }
    pub fn get_outline_variant(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::outline_variant(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_shadow(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::shadow())
    }
    pub fn get_scrim(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::scrim())
    }
    pub fn get_surface_tint(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::surface_tint())
    }
    pub fn get_primary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::primary(&MaterialDynamicColors {}))
    }
    pub fn get_on_primary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_primary(&MaterialDynamicColors {}))
    }
    pub fn get_primary_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::primary_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_primary_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_primary_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_inverse_primary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::inverse_primary())
    }
    pub fn get_secondary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::secondary(&MaterialDynamicColors {}))
    }
    pub fn get_on_secondary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_secondary(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_secondary_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::secondary_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_secondary_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_secondary_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_tertiary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::tertiary(&MaterialDynamicColors {}))
    }
    pub fn get_on_tertiary(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_tertiary(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_tertiary_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::tertiary_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_tertiary_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_tertiary_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_error(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::error(&MaterialDynamicColors {}))
    }
    pub fn get_on_error(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_error(&MaterialDynamicColors {}))
    }
    pub fn get_error_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::error_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_error_container(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_error_container(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_primary_fixed(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::primary_fixed(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_primary_fixed_dim(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::primary_fixed_dim(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_primary_fixed(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_primary_fixed(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_primary_fixed_variant(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_primary_fixed_variant(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_secondary_fixed(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::secondary_fixed(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_secondary_fixed_dim(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::secondary_fixed_dim(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_secondary_fixed(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_secondary_fixed(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_secondary_fixed_variant(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_secondary_fixed_variant(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_tertiary_fixed(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::tertiary_fixed(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_tertiary_fixed_dim(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::tertiary_fixed_dim(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_tertiary_fixed(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_tertiary_fixed(
            &MaterialDynamicColors {},
        ))
    }
    pub fn get_on_tertiary_fixed_variant(&self) -> i64 {
        self.get_argb(MaterialDynamicColors::on_tertiary_fixed_variant(
            &MaterialDynamicColors {},
        ))
    }
}

impl Hash for DynamicScheme {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contrast_level.to_ne_bytes().hash(state);
        self.is_dark.hash(state);
        self.variant.hash(state)
    }
}

impl Eq for DynamicScheme {}
impl PartialEq for DynamicScheme {
    fn eq(&self, other: &Self) -> bool {
        self.contrast_level == other.contrast_level
            && self.is_dark == other.is_dark
            && self.variant == other.variant
    }
}
