use crate::{dislike::dislike_analyzer::DislikeAnalyzer, hct::hct::Hct};

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

    pub fn primary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_palette_key_color"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| s.primary_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn secondary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_palette_key_color"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| s.secondary_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn tertiary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_palette_key_color"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| s.tertiary_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn neutral_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("neutral_palette_key_color"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| s.neutral_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn neutral_variant_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("neutral_variant_palette_key_color"),
            Box::new(|s| &s.neutral_variant_palette),
            Box::new(|s| s.neutral_variant_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn background() -> DynamicColor {
        DynamicColor::from_palette(
            Some("background"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| if s.is_dark { 6.0 } else { 98.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_background() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_background"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| if s.is_dark { 90.0 } else { 10.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::background())),
            None,
            Some(ContrastCurve::new(3.0, 3.0, 4.5, 7.0)),
            None,
        )
    }

    pub fn surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| if s.is_dark { 6.0 } else { 98.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
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

    pub fn surface_container_lowest() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_container_lowest"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    ContrastCurve::new(4.0, 4.0, 2.0, 0.0).get(s.contrast_level)
                } else {
                    100.0
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_low() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_container_low"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    ContrastCurve::new(10.0, 10.0, 11.0, 12.0).get(s.contrast_level)
                } else {
                    ContrastCurve::new(96.0, 96.0, 96.0, 95.0).get(s.contrast_level)
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_container"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    ContrastCurve::new(12.0, 12.0, 16.0, 20.0).get(s.contrast_level)
                } else {
                    ContrastCurve::new(94.0, 94.0, 92.0, 90.0).get(s.contrast_level)
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_high() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_container_high"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    ContrastCurve::new(17.0, 17.0, 21.0, 25.0).get(s.contrast_level)
                } else {
                    ContrastCurve::new(92.0, 92.0, 88.0, 85.0).get(s.contrast_level)
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_container_highest() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_container_highest"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| {
                if s.is_dark {
                    ContrastCurve::new(22.0, 22.0, 26.0, 30.0).get(s.contrast_level)
                } else {
                    ContrastCurve::new(90.0, 90.0, 84.0, 80.0).get(s.contrast_level)
                }
            }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_surface"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| if s.is_dark { 90.0 } else { 10.0 }),
            None,
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn surface_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_variant"),
            Box::new(|s| &s.neutral_variant_palette),
            Box::new(|s| if s.is_dark { 30.0 } else { 90.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_surface_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_surface_variant"),
            Box::new(|s| &s.neutral_variant_palette),
            Box::new(|s| if s.is_dark { 80.0 } else { 30.0 }),
            None,
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }

    pub fn inverse_surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("inverse_surface"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| if s.is_dark { 90.0 } else { 20.0 }),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn inverse_on_surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("inverse_on_surface"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|s| if s.is_dark { 20.0 } else { 95.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::inverse_surface())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn outline() -> DynamicColor {
        DynamicColor::from_palette(
            Some("outline"),
            Box::new(|s| &s.neutral_variant_palette),
            Box::new(|s| if s.is_dark { 60.0 } else { 50.0 }),
            None,
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.5, 3.0, 4.5, 7.0)),
            None,
        )
    }

    pub fn outline_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("outline_variant"),
            Box::new(|s| &s.neutral_variant_palette),
            Box::new(|s| if s.is_dark { 30.0 } else { 80.0 }),
            None,
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            None,
        )
    }

    pub fn shadow() -> DynamicColor {
        DynamicColor::from_palette(
            Some("shadow"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|_| 0.0),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn scrim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("scrim"),
            Box::new(|s| &s.neutral_palette),
            Box::new(|_s| 0.0),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_tint() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_tint"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| if s.is_dark { 80.0 } else { 40.0 }),
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

    pub fn on_primary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 10.0 } else { 90.0 };
                } else {
                    return if s.is_dark { 20.0 } else { 100.0 };
                }
            }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::primary())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
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

    pub fn on_primary_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary_container"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| {
                if _is_fidelity(s) {
                    return DynamicColor::foreground_tone(
                        MaterialDynamicColors::primary_container().get_tone(s),
                        4.5,
                    );
                } else if _is_monochrome(s) {
                    return if s.is_dark { 0.0 } else { 100.0 };
                } else {
                    return if s.is_dark { 90.0 } else { 10.0 };
                }
            }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::primary_container())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn inverse_primary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("inverse_primary"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| if s.is_dark { 40.0 } else { 80.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::inverse_surface())),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            None,
        )
    }

    pub fn secondary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| if s.is_dark { 80.0 } else { 40.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_container(),
                    MaterialDynamicColors::secondary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_secondary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 10.0 } else { 100.0 };
                } else {
                    return if s.is_dark { 20.0 } else { 100.0 };
                }
            }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::secondary())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn secondary_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_container"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| {
                let initial_tone = if s.is_dark { 30.0 } else { 90.0 };
                if _is_monochrome(s) {
                    return if s.is_dark { 30.0 } else { 85.0 };
                }
                if !_is_fidelity(s) {
                    return initial_tone;
                }
                return Self::_find_desired_chroma_by_tone(
                    s.secondary_palette.hue,
                    s.secondary_palette.chroma,
                    initial_tone,
                    {
                        if s.is_dark {
                            false
                        } else {
                            true
                        }
                    },
                );
            }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_container(),
                    MaterialDynamicColors::secondary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_secondary_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary_container"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| {
                if !_is_fidelity(s) {
                    return if s.is_dark { 90.0 } else { 10.0 };
                }
                return DynamicColor::foreground_tone(
                    MaterialDynamicColors::secondary_container().get_tone(s),
                    4.5,
                );
            }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::secondary_container())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn tertiary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 90.0 } else { 25.0 };
                }
                return if s.is_dark { 80.0 } else { 40.0 };
            }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_container(),
                    MaterialDynamicColors::tertiary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_tertiary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 10.0 } else { 90.0 };
                }
                return if s.is_dark { 20.0 } else { 100.0 };
            }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::tertiary())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn tertiary_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_container"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 60.0 } else { 49.0 };
                }
                if !_is_fidelity(s) {
                    return if s.is_dark { 30.0 } else { 90.0 };
                }
                let proposed_hct = s.tertiary_palette.get_hct(s.source_color_hct.get_tone());
                return DislikeAnalyzer::fix_if_disliked(&proposed_hct).get_tone();
            }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_container(),
                    MaterialDynamicColors::tertiary(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_tertiary_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary_container"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| {
                if _is_monochrome(s) {
                    return if s.is_dark { 0.0 } else { 100.0 };
                }
                if !_is_fidelity(s) {
                    return if s.is_dark { 90.0 } else { 10.0 };
                }
                return DynamicColor::foreground_tone(
                    MaterialDynamicColors::tertiary_container().get_tone(s),
                    4.5,
                );
            }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::tertiary_container())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn error() -> DynamicColor {
        DynamicColor::from_palette(
            Some("error"),
            Box::new(|s| &s.error_palette),
            Box::new(|s| if s.is_dark { 80.0 } else { 40.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::error_container(),
                    MaterialDynamicColors::error(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_error() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_error"),
            Box::new(|s| &s.error_palette),
            Box::new(|s| if s.is_dark { 20.0 } else { 100.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::error())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn error_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("error_container"),
            Box::new(|s| &s.error_palette),
            Box::new(|s| if s.is_dark { 30.0 } else { 90.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::error_container(),
                    MaterialDynamicColors::error(),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }
    pub fn on_error_container() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_error_container"),
            Box::new(|s| &s.error_palette),
            Box::new(|s| if s.is_dark { 90.0 } else { 10.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::error_container())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn primary_fixed() -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_fixed"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| if _is_monochrome(s) { 40.0 } else { 90.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_fixed(),
                    MaterialDynamicColors::primary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn primary_fixed_dim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_fixed_dim"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| if _is_monochrome(s) { 30.0 } else { 80.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_fixed(),
                    MaterialDynamicColors::primary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn on_primary_fixed() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary_fixed"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| if _is_monochrome(s) { 100.0 } else { 10.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::primary_fixed_dim())),
            Some(Box::new(|_| MaterialDynamicColors::primary_fixed())),
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn on_primary_fixed_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary_fixed_variant"),
            Box::new(|s| &s.primary_palette),
            Box::new(|s| if _is_monochrome(s) { 90.0 } else { 30.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::primary_fixed_dim())),
            Some(Box::new(|_| MaterialDynamicColors::primary_fixed())),
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }

    pub fn secondary_fixed() -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_fixed"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| if _is_monochrome(s) { 80.0 } else { 90.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_fixed(),
                    MaterialDynamicColors::secondary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn secondary_fixed_dim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_fixed_dim"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| if _is_monochrome(s) { 70.0 } else { 80.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_fixed(),
                    MaterialDynamicColors::secondary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn on_secondary_fixed() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary_fixed"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|_s| 10.0),
            None,
            Some(Box::new(|_| MaterialDynamicColors::secondary_fixed_dim())),
            Some(Box::new(|_| MaterialDynamicColors::secondary_fixed())),
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn on_secondary_fixed_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary_fixed_variant"),
            Box::new(|s| &s.secondary_palette),
            Box::new(|s| if _is_monochrome(s) { 25.0 } else { 30.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::secondary_fixed_dim())),
            Some(Box::new(|_| MaterialDynamicColors::secondary_fixed())),
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }

    pub fn tertiary_fixed() -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_fixed"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| if _is_monochrome(s) { 40.0 } else { 90.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_fixed(),
                    MaterialDynamicColors::tertiary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn tertiary_fixed_dim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_fixed_dim"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| if _is_monochrome(s) { 30.0 } else { 80.0 }),
            Some(true),
            Some(Box::new(|s| MaterialDynamicColors::highest_surface(s))),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Box::new(|_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_fixed(),
                    MaterialDynamicColors::tertiary_fixed_dim(),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn on_tertiary_fixed() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary_fixed"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| if _is_monochrome(s) { 100.0 } else { 10.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::tertiary_fixed_dim())),
            Some(Box::new(|_| MaterialDynamicColors::tertiary_fixed())),
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }
    pub fn on_tertiary_fixed_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary_fixed_variant"),
            Box::new(|s| &s.tertiary_palette),
            Box::new(|s| if _is_monochrome(s) { 90.0 } else { 30.0 }),
            None,
            Some(Box::new(|_| MaterialDynamicColors::tertiary_fixed_dim())),
            Some(Box::new(|_s| MaterialDynamicColors::tertiary_fixed())),
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }
    fn _find_desired_chroma_by_tone(
        hue: f64,
        chroma: f64,
        tone: f64,
        by_decreasing_tone: bool,
    ) -> f64 {
        let mut answer = tone;
        let mut closest_to_chroma = Hct::from(hue, chroma, tone);
        if closest_to_chroma.get_chroma() < chroma {
            let mut chroma_peak = closest_to_chroma.get_chroma();
            while closest_to_chroma.get_chroma() < chroma {
                answer += if by_decreasing_tone { -1.0 } else { 1.0 };
                let potential_solution = Hct::from(hue, chroma, answer);
                if chroma_peak > potential_solution.get_chroma() {
                    break;
                }
                if (potential_solution.get_chroma() - chroma).abs() < 0.4 {
                    break;
                }

                let potential_delta = (potential_solution.get_chroma() - chroma).abs();
                let current_delta = (closest_to_chroma.get_chroma() - chroma).abs();

                chroma_peak = chroma_peak.max(potential_solution.get_chroma());
                if potential_delta < current_delta {
                    closest_to_chroma = potential_solution;
                }
            }
        }
        return answer;
    }
}
