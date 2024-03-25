use crate::palettes::core_palette::CorePalette;

/// Prefer [ColorScheme]. This class is the same concept as Flutter's
/// ColorScheme class, inlined to ensure parity across languages.
#[deprecated = r#"The `Scheme` class is deprecated in favor of `DynamicScheme`.
Please see https://github.com/material-foundation/material-color-utilities/blob/main/make_schemes.md for migration guidance.
"#]
pub struct Scheme {
    pub primary: i64,
    pub on_primary: i64,
    pub primary_container: i64,
    pub on_primary_container: i64,
    pub secondary: i64,
    pub on_secondary: i64,
    pub secondary_container: i64,
    pub on_secondary_container: i64,
    pub tertiary: i64,
    pub on_tertiary: i64,
    pub tertiary_container: i64,
    pub on_tertiary_container: i64,
    pub error: i64,
    pub on_error: i64,
    pub error_container: i64,
    pub on_error_container: i64,
    pub background: i64,
    pub on_background: i64,
    pub surface: i64,
    pub on_surface: i64,
    pub surface_variant: i64,
    pub on_surface_variant: i64,
    pub outline: i64,
    pub outline_variant: i64,
    pub shadow: i64,
    pub scrim: i64,
    pub inverse_surface: i64,
    pub inverse_on_surface: i64,
    pub inverse_primary: i64,
}
#[deprecated = r#"The `Scheme` class is deprecated in favor of `DynamicScheme`.
Please see https://github.com/material-foundation/material-color-utilities/blob/main/make_schemes.md for migration guidance.
"#]
#[allow(deprecated)]
impl Scheme {
    pub fn light(color: i64) -> Scheme {
        Self::light_from_core_palette(CorePalette::of(color))
    }
    pub fn dark(color: i64) -> Scheme {
        Self::dark_from_core_palette(CorePalette::of(color))
    }
    pub fn light_content(color: i64) -> Scheme {
        Self::light_from_core_palette(CorePalette::content_of(color))
    }
    pub fn dark_content(color: i64) -> Scheme {
        Self::dark_from_core_palette(CorePalette::content_of(color))
    }
    pub fn light_from_core_palette(mut palette: CorePalette) -> Scheme {
        Scheme {
            primary: palette.primary.get(40),
            on_primary: palette.primary.get(100),
            primary_container: palette.primary.get(90),
            on_primary_container: palette.primary.get(10),
            secondary: palette.secondary.get(40),
            on_secondary: palette.secondary.get(100),
            secondary_container: palette.secondary.get(90),
            on_secondary_container: palette.secondary.get(10),
            tertiary: palette.tertiary.get(40),
            on_tertiary: palette.tertiary.get(100),
            tertiary_container: palette.tertiary.get(90),
            on_tertiary_container: palette.tertiary.get(10),
            error: CorePalette::get_error().get(40),
            on_error: CorePalette::get_error().get(100),
            error_container: CorePalette::get_error().get(90),
            on_error_container: CorePalette::get_error().get(10),
            background: palette.neutral.get(99),
            on_background: palette.neutral.get(10),
            surface: palette.neutral.get(99),
            on_surface: palette.neutral.get(10),
            surface_variant: palette.neutral_variant.get(90),
            on_surface_variant: palette.neutral_variant.get(30),
            outline: palette.neutral_variant.get(50),
            outline_variant: palette.neutral_variant.get(80),
            shadow: palette.neutral.get(0),
            scrim: palette.neutral.get(0),
            inverse_surface: palette.neutral.get(20),
            inverse_on_surface: palette.neutral.get(95),
            inverse_primary: palette.primary.get(80),
        }
    }
    pub fn dark_from_core_palette(mut palette: CorePalette) -> Scheme {
        Scheme {
            primary: palette.primary.get(80),
            on_primary: palette.primary.get(20),
            primary_container: palette.primary.get(30),
            on_primary_container: palette.primary.get(90),
            secondary: palette.secondary.get(80),
            on_secondary: palette.secondary.get(20),
            secondary_container: palette.secondary.get(30),
            on_secondary_container: palette.secondary.get(90),
            tertiary: palette.tertiary.get(80),
            on_tertiary: palette.tertiary.get(20),
            tertiary_container: palette.tertiary.get(30),
            on_tertiary_container: palette.tertiary.get(90),
            error: CorePalette::get_error().get(80),
            on_error: CorePalette::get_error().get(20),
            error_container: CorePalette::get_error().get(30),
            on_error_container: CorePalette::get_error().get(80),
            background: palette.neutral.get(10),
            on_background: palette.neutral.get(90),
            surface: palette.neutral.get(10),
            on_surface: palette.neutral.get(90),
            surface_variant: palette.neutral_variant.get(30),
            on_surface_variant: palette.neutral_variant.get(80),
            outline: palette.neutral_variant.get(60),
            outline_variant: palette.neutral_variant.get(30),
            shadow: palette.neutral.get(0),
            scrim: palette.neutral.get(0),
            inverse_surface: palette.neutral.get(90),
            inverse_on_surface: palette.neutral.get(20),
            inverse_primary: palette.primary.get(40),
        }
    }
}
