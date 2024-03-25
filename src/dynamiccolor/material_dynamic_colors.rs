use std::sync::Arc;

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

fn _is_fidelity(scheme: DynamicScheme) -> bool {
    if let Variant::Fidelity = scheme.variant {
        return true;
    }
    if let Variant::Content = scheme.variant {
        return true;
    }
    false
}

fn _is_monochrome(scheme: DynamicScheme) -> bool {
    if let Variant::Monochrome = scheme.variant {
        return true;
    }
    false
}

/// Tokens, or named colors, in the Material Design system.
pub struct MaterialDynamicColors {}
impl MaterialDynamicColors {
    const _CONTENT_ACCENT_TONE_DELTA: f64 = 15.0;
    pub fn highest_surface(&self, s: DynamicScheme) -> DynamicColor {
        return {
            if s.is_dark {
                Self::surface_bright()
            } else {
                Self::surface_dim()
            }
        };
    }

    pub fn primary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_palette_key_color".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| s.primary_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn secondary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_palette_key_color".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| s.secondary_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn tertiary_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_palette_key_color".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| s.tertiary_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn neutral_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("neutral_palette_key_color".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| s.neutral_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn neutral_variant_palette_key_color() -> DynamicColor {
        DynamicColor::from_palette(
            Some("neutral_variant_palette_key_color".to_string()),
            Arc::new(|s| s.neutral_variant_palette),
            Arc::new(|s| s.neutral_variant_palette.key_color.get_tone()),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn background() -> DynamicColor {
        DynamicColor::from_palette(
            Some("background".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| if s.is_dark { 6.0 } else { 98.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_background() -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_background".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| if s.is_dark { 90.0 } else { 10.0 }),
            None,
            Some(Arc::new(|_| MaterialDynamicColors::background())),
            None,
            Some(ContrastCurve::new(3.0, 3.0, 4.5, 7.0)),
            None,
        )
    }

    pub fn surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| if s.is_dark { 6.0 } else { 98.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_dim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_dim".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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
            Some("surface_bright".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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
            Some("surface_container_lowest".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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
            Some("surface_container_low".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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
            Some("surface_container".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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
            Some("surface_container_high".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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
            Some("surface_container_highest".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| {
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

    pub fn on_surface(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_surface".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| if s.is_dark { 90.0 } else { 10.0 }),
            None,
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn surface_variant() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_variant".to_string()),
            Arc::new(|s| s.neutral_variant_palette),
            Arc::new(|s| if s.is_dark { 30.0 } else { 90.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn on_surface_variant(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_surface_variant".to_string()),
            Arc::new(|s| s.neutral_variant_palette),
            Arc::new(|s| if s.is_dark { 80.0 } else { 30.0 }),
            None,
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }

    pub fn inverse_surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("inverse_surface".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| if s.is_dark { 90.0 } else { 20.0 }),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn inverse_on_surface() -> DynamicColor {
        DynamicColor::from_palette(
            Some("inverse_on_surface".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|s| if s.is_dark { 20.0 } else { 95.0 }),
            None,
            Some(Arc::new(|_| MaterialDynamicColors::inverse_surface())),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn outline(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("outline".to_string()),
            Arc::new(|s| s.neutral_variant_palette),
            Arc::new(|s| if s.is_dark { 60.0 } else { 50.0 }),
            None,
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.5, 3.0, 4.5, 7.0)),
            None,
        )
    }

    pub fn outline_variant(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("outline_variant".to_string()),
            Arc::new(|s| s.neutral_variant_palette),
            Arc::new(|s| if s.is_dark { 30.0 } else { 80.0 }),
            None,
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            None,
        )
    }

    pub fn shadow() -> DynamicColor {
        DynamicColor::from_palette(
            Some("shadow".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|_| 0.0),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn scrim() -> DynamicColor {
        DynamicColor::from_palette(
            Some("scrim".to_string()),
            Arc::new(|s| s.neutral_palette),
            Arc::new(|_| 0.0),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn surface_tint() -> DynamicColor {
        DynamicColor::from_palette(
            Some("surface_tint".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| if s.is_dark { 80.0 } else { 40.0 }),
            Some(true),
            None,
            None,
            None,
            None,
        )
    }

    pub fn primary(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| {
                if _is_monochrome(s.clone()) {
                    if s.is_dark {
                        100.0
                    } else {
                        0.0
                    }
                } else {
                    if s.is_dark {
                        80.0
                    } else {
                        40.0
                    }
                }
            }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_container(self),
                    MaterialDynamicColors::primary(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_primary(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| {
                if _is_monochrome(s.clone()) {
                    return {
                        if s.is_dark {
                            10.0
                        } else {
                            90.0
                        }
                    };
                }
                return {
                    if s.is_dark {
                        20.0
                    } else {
                        100.0
                    }
                };
            }),
            None,
            Some(Arc::new(move |_| MaterialDynamicColors::primary(self))),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn primary_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_container".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| {
                if _is_fidelity(s.clone()) {
                    return s.source_color_hct.get_tone();
                }
                if _is_monochrome(s.clone()) {
                    return {
                        if s.is_dark {
                            85.0
                        } else {
                            25.0
                        }
                    };
                }
                return if s.is_dark { 30.0 } else { 90.0 };
            }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_container(self),
                    MaterialDynamicColors::primary(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_primary_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary_container".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(move |s| {
                if _is_fidelity(s.clone()) {
                    return DynamicColor::foreground_tone(
                        MaterialDynamicColors::primary_container(self).get_tone(s),
                        4.5,
                    );
                }
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 0.0 } else { 100.0 };
                }
                return if s.is_dark { 90.0 } else { 10.0 };
            }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::primary_container(self)
            })),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn inverse_primary() -> DynamicColor {
        DynamicColor::from_palette(
            Some("inverse_primary".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| if s.is_dark { 40.0 } else { 80.0 }),
            None,
            Some(Arc::new(|_| MaterialDynamicColors::inverse_surface())),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            None,
        )
    }

    pub fn secondary(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| if s.is_dark { 80.0 } else { 40.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_container(self),
                    MaterialDynamicColors::secondary(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_secondary(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| {
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 10.0 } else { 100.0 };
                } else {
                    return if s.is_dark { 20.0 } else { 100.0 };
                }
            }),
            None,
            Some(Arc::new(move |_| MaterialDynamicColors::secondary(self))),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn secondary_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_container".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| {
                let initial_tone = if s.is_dark { 30.0 } else { 90.0 };
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 30.0 } else { 85.0 };
                }
                if !_is_fidelity(s.clone()) {
                    return initial_tone;
                }
                return Self::_find_desired_chroma_by_tone(
                    s.secondary_palette.hue,
                    s.secondary_palette.chroma,
                    initial_tone,
                    if s.is_dark { false } else { true },
                );
            }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_container(self),
                    MaterialDynamicColors::secondary(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }
    pub fn on_secondary_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary_container".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(move |s| {
                if !_is_fidelity(s.clone()) {
                    return if s.is_dark { 90.0 } else { 10.0 };
                }
                return DynamicColor::foreground_tone(
                    MaterialDynamicColors::secondary_container(self).get_tone(s),
                    4.5,
                );
            }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::secondary_container(self)
            })),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn tertiary(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| {
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 90.0 } else { 25.0 };
                }
                return if s.is_dark { 80.0 } else { 40.0 };
            }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_container(self),
                    MaterialDynamicColors::tertiary(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_tertiary(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| {
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 10.0 } else { 90.0 };
                }
                return if s.is_dark { 20.0 } else { 100.0 };
            }),
            None,
            Some(Arc::new(move |_| MaterialDynamicColors::tertiary(self))),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn tertiary_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_container".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| {
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 60.0 } else { 49.0 };
                }
                if !_is_fidelity(s.clone()) {
                    return if s.is_dark { 30.0 } else { 90.0 };
                }
                let proposed_hct = s.tertiary_palette.get_hct(s.source_color_hct.get_tone());
                return DislikeAnalyzer::fix_if_disliked(proposed_hct).get_tone();
            }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_container(self),
                    MaterialDynamicColors::tertiary(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_tertiary_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary_container".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(move |s| {
                if _is_monochrome(s.clone()) {
                    return if s.is_dark { 0.0 } else { 100.0 };
                }
                if !_is_fidelity(s.clone()) {
                    return if s.is_dark { 90.0 } else { 10.0 };
                }
                return DynamicColor::foreground_tone(
                    MaterialDynamicColors::tertiary_container(self).get_tone(s),
                    4.5,
                );
            }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::tertiary_container(self)
            })),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn error(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("error".to_string()),
            Arc::new(|s| s.error_palette),
            Arc::new(|s| if s.is_dark { 80.0 } else { 40.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::error_container(self),
                    MaterialDynamicColors::error(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_error(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_error".to_string()),
            Arc::new(|s| s.error_palette),
            Arc::new(|s| if s.is_dark { 20.0 } else { 100.0 }),
            None,
            Some(Arc::new(move |_| MaterialDynamicColors::error(self))),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn error_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("error_container".to_string()),
            Arc::new(|s| s.error_palette),
            Arc::new(|s| if s.is_dark { 30.0 } else { 90.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::error_container(self),
                    MaterialDynamicColors::error(self),
                    10.0,
                    TonePolarity::Nearer,
                    false,
                )
            })),
        )
    }

    pub fn on_error_container(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_error_container".to_string()),
            Arc::new(|s| s.error_palette),
            Arc::new(|s| if s.is_dark { 90.0 } else { 10.0 }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::error_container(self)
            })),
            None,
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }

    pub fn primary_fixed(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_fixed".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| if _is_monochrome(s) { 40.0 } else { 90.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_fixed(self),
                    MaterialDynamicColors::primary_fixed_dim(self),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn primary_fixed_dim(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("primary_fixed_dim".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| if _is_monochrome(s) { 30.0 } else { 80.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::primary_fixed(self),
                    MaterialDynamicColors::primary_fixed_dim(self),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }

    pub fn on_primary_fixed(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary_fixed".to_string()),
            Arc::new(|s| s.primary_palette),
            Arc::new(|s| if _is_monochrome(s) { 100.0 } else { 10.0 }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::primary_fixed_dim(self)
            })),
            Some(Arc::new(move |_| {
                MaterialDynamicColors::primary_fixed(self)
            })),
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }
    pub fn on_primary_fixed_variant(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_primary_fixed_variant".to_string()),
            Arc::new(move |s| s.primary_palette),
            Arc::new(move |s| if _is_monochrome(s) { 90.0 } else { 30.0 }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::primary_fixed_dim(self)
            })),
            Some(Arc::new(move |_| {
                MaterialDynamicColors::primary_fixed(self)
            })),
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }
    pub fn secondary_fixed(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_fixed".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| if _is_monochrome(s) { 80.0 } else { 90.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_fixed(self),
                    MaterialDynamicColors::secondary_fixed_dim(self),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }
    pub fn secondary_fixed_dim(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("secondary_fixed_dim".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| if _is_monochrome(s) { 70.0 } else { 80.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::secondary_fixed(self),
                    MaterialDynamicColors::secondary_fixed_dim(self),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }
    pub fn on_secondary_fixed(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary_fixed".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|_| 10.0),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::secondary_fixed_dim(self)
            })),
            Some(Arc::new(move |_| {
                MaterialDynamicColors::secondary_fixed(self)
            })),
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }
    pub fn on_secondary_fixed_variant(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_secondary_fixed_variant".to_string()),
            Arc::new(|s| s.secondary_palette),
            Arc::new(|s| if _is_monochrome(s) { 25.0 } else { 30.0 }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::secondary_fixed_dim(self)
            })),
            Some(Arc::new(move |_| {
                MaterialDynamicColors::secondary_fixed(self)
            })),
            Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            None,
        )
    }
    pub fn tertiary_fixed(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_fixed".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| if _is_monochrome(s) { 40.0 } else { 90.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_fixed(self),
                    MaterialDynamicColors::tertiary_fixed_dim(self),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }
    pub fn tertiary_fixed_dim(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("tertiary_fixed_dim".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| if _is_monochrome(s) { 30.0 } else { 80.0 }),
            Some(true),
            Some(Arc::new(|s| {
                MaterialDynamicColors::highest_surface(self, s)
            })),
            None,
            Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Some(Arc::new(move |_| {
                ToneDeltaPair::new(
                    MaterialDynamicColors::tertiary_fixed(self),
                    MaterialDynamicColors::tertiary_fixed_dim(self),
                    10.0,
                    TonePolarity::Lighter,
                    true,
                )
            })),
        )
    }
    pub fn on_tertiary_fixed(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary_fixed".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| if _is_monochrome(s) { 100.0 } else { 10.0 }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::tertiary_fixed_dim(self)
            })),
            Some(Arc::new(move |_| {
                MaterialDynamicColors::tertiary_fixed(self)
            })),
            Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            None,
        )
    }
    pub fn on_tertiary_fixed_variant(&'static self) -> DynamicColor {
        DynamicColor::from_palette(
            Some("on_tertiary_fixed_variant".to_string()),
            Arc::new(|s| s.tertiary_palette),
            Arc::new(|s| if _is_monochrome(s) { 90.0 } else { 30.0 }),
            None,
            Some(Arc::new(move |_| {
                MaterialDynamicColors::tertiary_fixed_dim(self)
            })),
            Some(Arc::new(move |_| {
                MaterialDynamicColors::tertiary_fixed(self)
            })),
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
        let mut closest_to_chroma: Hct = Hct::from(hue, chroma, tone);
        if closest_to_chroma.get_chroma() < chroma {
            let mut chroma_peak: f64 = closest_to_chroma.get_chroma();
            while closest_to_chroma.get_chroma() < chroma {
                answer += {
                    if by_decreasing_tone {
                        -1.0
                    } else {
                        1.0
                    }
                };
                let potential_solution = Hct::from(hue, chroma, answer);
                if chroma_peak > potential_solution.get_chroma() {
                    break;
                }
                if (potential_solution.get_chroma() - chroma).abs() < 0.4 {
                    break;
                }
                let potential_delta = (potential_solution.get_chroma() - chroma).abs();
                let current_delta = (closest_to_chroma.get_chroma() - chroma).abs();
                if potential_delta < current_delta {
                    closest_to_chroma = potential_solution.clone();
                }
                chroma_peak = chroma_peak.max(potential_solution.get_chroma());
            }
        }
        return answer;
    }
}
