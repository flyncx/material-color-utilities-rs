use std::{collections::HashMap, sync::Arc};

use crate::{
    contrast::contrast::Contrast, hct::hct::Hct, palettes::tonal_palette::TonalPalette,
    utils::math_utils::MathUtils,
};

use super::{
    dynamic_scheme::DynamicScheme,
    src::{
        contrast_curve::ContrastCurve,
        tone_delta_pair::{ToneDeltaPair, TonePolarity},
    },
};

/// A color that adjusts itself based on UI state provided by [DynamicScheme].
///
/// This color automatically adjusts to accommodate a desired contrast level, or
/// other adjustments such as differing in light mode versus dark mode, or what
/// the theme is, or what the color that produced the theme is, etc.
///
/// Colors without backgrounds do not change tone when contrast changes. Colors
/// with backgrounds become closer to their background as contrast lowers, and
/// further when contrast increases.
///
/// Prefer the static constructors. They provide a much more simple interface,
/// such as requiring just a hexcode, or just a hexcode and a background.
///
/// Ultimately, each component necessary for calculating a color, adjusting it
/// for a desired contrast level, and ensuring it has a certain lightness/tone
/// difference from another color, is provided by a function that takes a
/// [DynamicScheme] and returns a value. This ensures ultimate flexibility, any
/// desired behavior of a color for any design system, but it usually
/// unnecessary. See the default constructor for more information.
#[derive(Clone)]
pub struct DynamicColor {
    pub name: String,
    pub palette: Arc<dyn Fn(DynamicScheme) -> TonalPalette>,
    pub tone: Arc<dyn Fn(DynamicScheme) -> f64>,
    pub is_background: bool,
    pub background: Option<Arc<dyn Fn(DynamicScheme) -> DynamicColor>>,
    pub second_background: Option<Arc<dyn Fn(DynamicScheme) -> DynamicColor>>,
    pub contrast_curve: Option<ContrastCurve>,
    pub tone_delta_pair: Option<Arc<dyn Fn(DynamicScheme) -> ToneDeltaPair>>,
    pub _hct_cache: HashMap<DynamicScheme, Hct>,
}

impl DynamicColor {
    /// The base (explicit) constructor for [DynamicColor].
    ///
    /// [name] The name of the dynamic color.
    /// [palette] Function that provides a TonalPalette given
    /// DynamicScheme. A TonalPalette is defined by a hue and chroma, so this
    /// replaces the need to specify hue/chroma. By providing a tonal palette,
    /// when contrast adjustments are made, intended chroma can be preserved.
    /// [tone] Function that provides a tone, given a DynamicScheme.
    /// [isBackground] Whether this dynamic color is a background, with
    /// some other color as the foreground.
    /// [background] The background of the dynamic color (as a function of a
    /// `DynamicScheme`), if it exists.
    /// [secondBackground] A second background of the dynamic color (as a function
    /// of a `DynamicScheme`), if it
    /// exists.
    /// [contrastCurve] A [ContrastCurve] object specifying how its contrast
    /// against its background should behave in various contrast levels options.
    /// [toneDeltaPair] A [ToneDeltaPair] object specifying a tone delta
    /// constraint between two colors. One of them must be the color being
    /// constructed.
    pub fn new(
        name: String,
        palette: Arc<dyn Fn(DynamicScheme) -> TonalPalette>,
        tone: Arc<dyn Fn(DynamicScheme) -> f64>,
        is_background: bool,
        background: Option<Arc<dyn Fn(DynamicScheme) -> DynamicColor>>,
        second_background: Option<Arc<dyn Fn(DynamicScheme) -> DynamicColor>>,
        contrast_curve: Option<ContrastCurve>,
        tone_delta_pair: Option<Arc<dyn Fn(DynamicScheme) -> ToneDeltaPair>>,
    ) -> DynamicColor {
        DynamicColor {
            name,
            palette,
            tone,
            is_background,
            background,
            second_background,
            contrast_curve,
            tone_delta_pair,
            _hct_cache: HashMap::new(),
        }
    }

    /// The convenience constructor for [DynamicColor].
    ///
    /// Similar to the base constructor, but all parameters other than [palette]
    /// and [tone] have defaults.
    ///
    /// [name] The name of the dynamic color. Defaults to empty.
    /// [palette] Function that provides a TonalPalette given
    /// DynamicScheme. A TonalPalette is defined by a hue and chroma, so this
    /// replaces the need to specify hue/chroma. By providing a tonal palette,
    /// when contrast adjustments are made, intended chroma can be preserved.
    /// [tone] Function that provides a tone, given a DynamicScheme.
    /// [isBackground] Whether this dynamic color is a background, with
    /// some other color as the foreground. Defaults to false.
    /// [background] The background of the dynamic color (as a function of a
    /// `DynamicScheme`), if it exists.
    /// [secondBackground] A second background of the dynamic color (as a function
    /// of a `DynamicScheme`), if it exists.
    /// [contrastCurve] A [ContrastCurve] object specifying how its contrast
    /// against its background should behave in various contrast levels options.
    /// [toneDeltaPair] A [ToneDeltaPair] object specifying a tone delta
    /// constraint between two colors. One of them must be the color being
    /// constructed.
    pub fn from_palette(
        name: Option<String>,
        palette: Arc<dyn Fn(DynamicScheme) -> TonalPalette>,
        tone: Arc<dyn Fn(DynamicScheme) -> f64>,
        is_background: Option<bool>,
        background: Option<Arc<dyn Fn(DynamicScheme) -> DynamicColor>>,
        second_background: Option<Arc<dyn Fn(DynamicScheme) -> DynamicColor>>,
        contrast_curve: Option<ContrastCurve>,
        tone_delta_pair: Option<Arc<dyn Fn(DynamicScheme) -> ToneDeltaPair>>,
    ) -> DynamicColor {
        let name = name.unwrap_or("".to_owned());
        let is_background = is_background.unwrap_or(false);

        Self::new(
            name,
            palette,
            tone,
            is_background,
            background,
            second_background,
            contrast_curve,
            tone_delta_pair,
        )
    }
    /// Return a ARGB integer (i.e. a hex code).

    /// [scheme] Defines the conditions of the user interface, for example,
    /// whether or not it is dark mode or light mode, and what the desired
    /// contrast level is.
    pub fn get_argb(&mut self, scheme: DynamicScheme) -> i64 {
        return self.get_hct(scheme).to_int();
    }

    /// Return a color, expressed in the HCT color space, that this
    /// [DynamicColor] is under the conditions in [scheme].
    ///
    /// [scheme] Defines the conditions of the user interface, for example,
    /// whether or not it is dark mode or light mode, and what the desired
    /// contrast level is.
    pub fn get_hct(&mut self, scheme: DynamicScheme) -> Hct {
        let cached_answer = self._hct_cache.get(&scheme); // self._hctCache[&scheme];

        match cached_answer {
            Some(cached_answer) => {
                return cached_answer.clone();
            }
            None => {
                let tone = self.get_tone(scheme.clone());

                let answer = (self.palette)(scheme.clone()).get_hct(tone);

                if self._hct_cache.len() > 4 {
                    self._hct_cache.clear();
                }
                self._hct_cache.insert(scheme, answer.clone());
                return answer;
            }
        }
    }

    /// Return a tone, T in the HCT color space, that this [DynamicColor] is under
    /// the conditions in [scheme].
    ///
    /// [scheme] Defines the conditions of the user interface, for example,
    /// whether or not it is dark mode or light mode, and what the desired
    /// contrast level is.
    pub fn get_tone(&self, scheme: DynamicScheme) -> f64 {
        let decreasing_contrast = scheme.contrast_level < 0.0;

        match &self.tone_delta_pair {
            Some(tone_delta_pair) => {
                let pair = (tone_delta_pair.clone())(scheme.clone());
                let role_a = pair.role_a.clone();
                let role_b = pair.role_b.clone();
                let delta = pair.delta;
                let polarity = pair.polarity;
                let stay_together = pair.stay_together;

                let bg = (self.background.as_ref().unwrap())(scheme.clone());
                let bg_tone = bg.get_tone(scheme.clone());

                let a_is_nearer: bool = {
                    match polarity {
                        TonePolarity::Darker => scheme.is_dark,
                        TonePolarity::Lighter => !scheme.is_dark,
                        TonePolarity::Nearer => true,
                        TonePolarity::Farther => false,
                    }
                };

                let nearer = {
                    if a_is_nearer {
                        role_a.clone()
                    } else {
                        role_b.clone()
                    }
                };
                let farther = {
                    if a_is_nearer {
                        role_b
                    } else {
                        role_a
                    }
                };
                let am_nearer = self.name == nearer.name;
                let expansion_dir = {
                    if scheme.is_dark {
                        1
                    } else {
                        -1
                    }
                };

                // 1st round: solve to min, each
                let n_contrast = nearer
                    .clone()
                    .contrast_curve
                    .unwrap()
                    .get(scheme.contrast_level as f64);
                let f_contrast = farther
                    .clone()
                    .contrast_curve
                    .unwrap()
                    .get(scheme.contrast_level as f64);

                // If a color is good enough, it is not adjusted.
                // Initial and adjusted tones for `nearer`
                let n_initial_tone = nearer.get_tone(scheme.clone());
                let mut n_tone = {
                    if Contrast::ratio_of_tones(bg_tone, n_initial_tone) >= n_contrast {
                        n_initial_tone
                    } else {
                        DynamicColor::foreground_tone(bg_tone, n_contrast)
                    }
                };
                // Initial and adjusted tones for `farther`
                let f_initial_tone = farther.get_tone(scheme);
                let mut f_tone = {
                    if Contrast::ratio_of_tones(bg_tone, f_initial_tone) >= f_contrast {
                        f_initial_tone
                    } else {
                        DynamicColor::foreground_tone(bg_tone, f_contrast)
                    }
                };

                if decreasing_contrast {
                    // If decreasing contrast, adjust color to the "bare minimum"
                    // that satisfies contrast.
                    n_tone = DynamicColor::foreground_tone(bg_tone, n_contrast);
                    f_tone = DynamicColor::foreground_tone(bg_tone, f_contrast);
                }

                if (f_tone - n_tone) * expansion_dir as f64 >= delta {
                    // Good! Tones satisfy the constraint; no change needed.
                } else {
                    // 2nd round: expand farther to match delta.
                    f_tone =
                        MathUtils::clamp_double(0.0, 100.0, n_tone + delta * expansion_dir as f64);
                    if (f_tone - n_tone) * expansion_dir as f64 >= delta {
                        // Good! Tones now satisfy the constraint; no change needed.
                    } else {
                        // 3rd round: contract nearer to match delta.
                        n_tone = MathUtils::clamp_double(
                            0.0,
                            100.0,
                            f_tone - delta * expansion_dir as f64,
                        );
                    }
                }

                // Avoids the 50-59 awkward zone.
                if 50.0 <= n_tone && n_tone < 60.0 {
                    // If `nearer` is in the awkward zone, move it away, together with
                    // `farther`.
                    if expansion_dir > 0 {
                        n_tone = 60.0;
                        f_tone = f_tone.max(n_tone + delta * expansion_dir as f64);
                    } else {
                        n_tone = 49.0;
                        f_tone = f_tone.min(n_tone + delta * expansion_dir as f64);
                    }
                } else if 50.0 <= f_tone && f_tone < 60.0 {
                    if stay_together {
                        // Fixes both, to avoid two colors on opposite sides of the "awkward
                        // zone".
                        if expansion_dir > 0 {
                            n_tone = 60.0;
                            f_tone = f_tone.max(n_tone + delta * expansion_dir as f64);
                        } else {
                            n_tone = 49.0;
                            f_tone = f_tone.min(n_tone + delta * expansion_dir as f64);
                        }
                    } else {
                        // Not required to stay together; fixes just one.
                        if expansion_dir > 0 {
                            f_tone = 60.0;
                        } else {
                            f_tone = 49.0;
                        }
                    }
                }

                // Returns `nTone` if this color is `nearer`, otherwise `fTone`.
                return {
                    if am_nearer {
                        n_tone
                    } else {
                        f_tone
                    }
                };
            }
            None => {
                // Case 2: No contrast pair; just solve for itself.
                //let mut answer = self.get_tone(scheme.clone());
                let mut answer = (self.tone)(scheme.clone());
                if self.background.is_none() {
                    return answer; // No adjustment for colors with no background.
                }

                let bg_tone =
                    (self.background.clone().unwrap())(scheme.clone()).get_tone(scheme.clone());

                let desired_ratio = self
                    .contrast_curve
                    .clone()
                    .unwrap()
                    .get(scheme.contrast_level as f64);

                if Contrast::ratio_of_tones(bg_tone, answer) >= desired_ratio {
                    // Don't "improve" what's good enough.
                } else {
                    // Rough improvement.
                    answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
                }

                if decreasing_contrast {
                    answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
                }

                if self.is_background && 50.0 <= answer && answer < 60.0 {
                    // Must adjust
                    if Contrast::ratio_of_tones(49.0, bg_tone) >= desired_ratio {
                        answer = 49.0;
                    } else {
                        answer = 60.0;
                    }
                }

                if let Some(second_background) = &self.second_background {
                    // Case 3: Adjust for dual backgrounds.

                    let bg_tone1 =
                        (self.background.clone().unwrap())(scheme.clone()).get_tone(scheme.clone());
                    let bg_tone2 =
                        (second_background.clone())(scheme.clone()).get_tone(scheme.clone());

                    let upper = bg_tone1.max(bg_tone2);
                    let lower = bg_tone1.min(bg_tone2);

                    if Contrast::ratio_of_tones(upper, answer) >= desired_ratio
                        && Contrast::ratio_of_tones(lower, answer) >= desired_ratio
                    {
                        return answer;
                    }

                    // The darkest light tone that satisfies the desired ratio,
                    // or -1 if such ratio cannot be reached.
                    let light_option = Contrast::lighter(upper, desired_ratio);

                    // The lightest dark tone that satisfies the desired ratio,
                    // or -1 if such ratio cannot be reached.
                    let dark_option = Contrast::darker(lower, desired_ratio);

                    // Tones suitable for the foreground.
                    let mut availables: Vec<f64> = Vec::new();
                    if light_option != -1.0 {
                        availables.push(light_option);
                    }
                    if dark_option != -1.0 {
                        availables.push(dark_option);
                    }

                    let prefers_light = DynamicColor::tone_prefers_light_foreground(bg_tone1)
                        || DynamicColor::tone_prefers_light_foreground(bg_tone2);
                    if prefers_light {
                        return {
                            if light_option < 0.0 {
                                100.0
                            } else {
                                light_option
                            }
                        };
                    }
                    if availables.len() == 1 {
                        return availables[0];
                    }
                    return {
                        if dark_option < 0.0 {
                            0.0
                        } else {
                            dark_option
                        }
                    };
                }

                return answer;
            }
        }
    }

    /// Given a background tone, find a foreground tone, while ensuring they reach
    /// a contrast ratio that is as close to [ratio] as possible.
    ///
    /// [bgTone] Tone in HCT. Range is 0 to 100, undefined behavior when it falls
    /// outside that range.
    /// [ratio] The contrast ratio desired between [bgTone] and the return value.
    pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
        let lighter_tone = Contrast::lighter_unsafe(bg_tone, ratio);
        let darker_tone = Contrast::darker_unsafe(bg_tone, ratio);
        let lighter_ratio = Contrast::ratio_of_tones(lighter_tone, bg_tone);
        let darker_ratio = Contrast::ratio_of_tones(darker_tone, bg_tone);
        let prefer_lighter = Self::tone_prefers_light_foreground(bg_tone);

        if prefer_lighter {
            // This handles an edge case where the initial contrast ratio is high
            // (ex. 13.0), and the ratio passed to the function is that high ratio,
            // and both the lighter and darker ratio fails to pass that ratio.
            //
            // This was observed with Tonal Spot's On Primary Container turning black
            // momentarily between high and max contrast in light mode.
            // PC's standard tone was T90, OPC's was T10, it was light mode, and the
            // contrast value was 0.6568521221032331.
            let negligible_difference = (lighter_ratio - darker_ratio).abs() < 0.1
                && lighter_ratio < ratio
                && darker_ratio < ratio;

            return {
                if lighter_ratio >= ratio || lighter_ratio >= darker_ratio || negligible_difference
                {
                    lighter_tone
                } else {
                    darker_tone
                }
            };
        } else {
            return {
                if darker_ratio >= ratio || darker_ratio >= lighter_ratio {
                    darker_tone
                } else {
                    lighter_tone
                }
            };
        }
    }

    /// Adjust a tone such that white has 4.5 contrast, if the tone is
    /// reasonably close to supporting it.
    pub fn enable_light_foreground(tone: f64) -> f64 {
        if Self::tone_prefers_light_foreground(tone) && !Self::tone_allows_light_foreground(tone) {
            return 49.0;
        }
        return tone;
    }

    /// Returns whether [tone] prefers a light foreground.
    ///
    /// People prefer white foregrounds on ~T60-70. Observed over time, and also
    /// by Andrew Somers during research for APCA.
    ///
    /// T60 used as to create the smallest discontinuity possible when skipping
    /// down to T49 in order to ensure light foregrounds.
    ///
    /// Since `tertiaryContainer` in dark monochrome scheme requires a tone of
    /// 60, it should not be adjusted. Therefore, 60 is excluded here.
    pub fn tone_prefers_light_foreground(tone: f64) -> bool {
        return tone.round() < 60.0;
    }

    /// Returns whether [tone] can reach a contrast ratio of 4.5 with a lighter
    /// color.
    pub fn tone_allows_light_foreground(tone: f64) -> bool {
        return tone.round() <= 49.0;
    }
}
