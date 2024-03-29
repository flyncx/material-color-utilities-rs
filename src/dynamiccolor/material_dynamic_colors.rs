use super::{
    dynamic_color::DynamicColor,
    dynamic_scheme::DynamicScheme,
    src::{
        contrast_curve::ContrastCurve,
        tone_delta_pair::{ToneDeltaPair, TonePolarity},
    },
    variant::Variant,
};

fn _is_fidelity(scheme: &DynamicScheme) -> bool {
    scheme.variant == Variant::Fidelity || scheme.variant == Variant::Content
}
fn _is_monochrome(scheme: &DynamicScheme) -> bool {
    scheme.variant == Variant::Monochrome
}

pub struct MaterialDynamicColors {}
impl MaterialDynamicColors {
    //static const double contentAccentToneDelta = 15.0;
    pub fn highest_surface(s: &DynamicScheme) -> DynamicColor {
        return if s.is_dark {
            Self::surface_bright()
        } else {
            Self::surface_dim()
        };
    }
    pub fn surface_dim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_dim"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    6.0
                } else {
                    ContrastCurve::new(87.0, 87.0, 80.0, 75.0).get(s.contrast_level)
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_bright() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_bright"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    ContrastCurve::new(24.0, 24.0, 29.0, 34.0).get(s.contrast_level)
                } else {
                    98.0
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn primary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 100.0 } else { 0.0 };
                }
                return if s.is_dark { 80.0 } else { 40.0 };
            }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_container(),
                    MaterialDynamicColors::primary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn primary_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_container"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| {
                if _is_fidelity(s) {
                    return s.source_color_hct.get_tone();
                }
                if _is_monochrome(s) {
                    return if s.is_dark { 85.0 } else { 25.0 };
                }
                return if s.is_dark { 30.0 } else { 90.0 };
            }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_container(),
                    MaterialDynamicColors::primary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }
}
