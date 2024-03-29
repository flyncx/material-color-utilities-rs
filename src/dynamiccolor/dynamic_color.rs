use std::collections::HashMap;

use crate::{
    contrast::contrast::Contrast, dynamiccolor::src::tone_delta_pair::TonePolarity, hct::hct::Hct,
    palettes::tonal_palette::TonalPalette, utils::math_utils::MathUtils,
};

use super::{
    dynamic_scheme::DynamicScheme,
    src::{contrast_curve::ContrastCurve, tone_delta_pair::ToneDeltaPair},
};
pub struct DynamicColor {
    pub name: String,
    pub palette: Box<dyn Fn(&DynamicScheme) -> &TonalPalette>,
    pub tone: Box<dyn Fn(&DynamicScheme) -> f64>,
    pub is_background: bool,
    pub background: Option<Box<dyn Fn(&DynamicScheme) -> DynamicColor>>,
    pub second_background: Option<Box<dyn Fn(&DynamicScheme) -> DynamicColor>>,
    pub contrast_curve: Option<ContrastCurve>,
    pub tone_delta_pair: Option<Box<dyn Fn(&DynamicScheme) -> ToneDeltaPair>>,
    _hct_cache: HashMap<DynamicScheme, Hct>,
}
impl DynamicColor {
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
        name: Option<&str>,
        palette: Box<dyn Fn(&DynamicScheme) -> &TonalPalette>,
        tone: Box<dyn Fn(&DynamicScheme) -> f64>,
        is_background: Option<bool>,
        background: Option<Box<dyn Fn(&DynamicScheme) -> DynamicColor>>,
        second_background: Option<Box<dyn Fn(&DynamicScheme) -> DynamicColor>>,
        contrast_curve: Option<ContrastCurve>,
        tone_delta_pair: Option<Box<dyn Fn(&DynamicScheme) -> ToneDeltaPair>>,
    ) -> Self {
        Self {
            name: name.unwrap_or("").to_string(),
            palette,
            tone,
            is_background: is_background.unwrap_or(false),
            background,
            second_background,
            contrast_curve,
            tone_delta_pair,
            _hct_cache: HashMap::new(),
        }
    }

    /// Return a ARGB integer (i.e. a hex code).

    /// [scheme] Defines the conditions of the user interface, for example,
    /// whether or not it is dark mode or light mode, and what the desired
    /// contrast level is.
    pub fn get_argb(&mut self, scheme: &DynamicScheme) -> i64 {
        return self.get_hct(scheme).to_int();
    }

    /// Return a color, expressed in the HCT color space, that this
    /// [DynamicColor] is under the conditions in [scheme].
    ///
    /// [scheme] Defines the conditions of the user interface, for example,
    /// whether or not it is dark mode or light mode, and what the desired
    /// contrast level is.
    pub fn get_hct(&mut self, scheme: &DynamicScheme) -> Hct {
        let cached_answer = self._hct_cache.get(scheme);
        match cached_answer {
            Some(cached_answer) => cached_answer.clone(),
            None => {
                let tone = self.get_tone(scheme);
                let answer = (self.palette)(scheme).get_hct(tone);
                if self._hct_cache.len() > 4 {
                    self._hct_cache.clear();
                }
                self._hct_cache.insert(scheme.clone(), answer.clone());
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
    pub fn get_tone(&self, scheme: &DynamicScheme) -> f64 {
        let decreasing_contrast = scheme.contrast_level < 0.0;

        match &self.tone_delta_pair {
            // Case 1: dual foreground, pair of colors with delta constraint.
            Some(tone_delta_pair) => {
                let pair = (tone_delta_pair)(scheme);
                let role_a = pair.role_a;
                let role_b = pair.role_b;
                let delta = pair.delta;
                let polarity = pair.polarity;
                let stay_together = pair.stay_together;

                let background = self.background.as_ref().unwrap();
                let bg = (background)(scheme);
                let bg_tone = bg.get_tone(scheme);

                let a_is_nearer = polarity == TonePolarity::Nearer
                    || (polarity == TonePolarity::Lighter && !scheme.is_dark)
                    || (polarity == TonePolarity::Darker && scheme.is_dark);
                let nearer = if a_is_nearer { &role_a } else { &role_b };
                let farther = if a_is_nearer { &role_b } else { &role_a };
                let am_nearer = self.name == nearer.name;
                let expansion_dir = if scheme.is_dark { 1.0 } else { -1.0 };

                // 1st round: solve to min, each
                let n_contrast = nearer
                    .contrast_curve
                    .as_ref()
                    .unwrap()
                    .get(scheme.contrast_level);
                let f_contrast = farther
                    .contrast_curve
                    .as_ref()
                    .unwrap()
                    .get(scheme.contrast_level);

                // If a color is good enough, it is not adjusted.
                // Initial and adjusted tones for `nearer`
                let n_initial_tone = (nearer.tone)(scheme);

                let mut n_tone = {
                    if Contrast::ratio_of_tones(bg_tone, n_initial_tone) >= n_contrast {
                        n_initial_tone
                    } else {
                        DynamicColor::foreground_tone(bg_tone, n_contrast)
                    }
                };

                // Initial and adjusted tones for `farther`
                let f_initial_tone = (farther.tone)(scheme);
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
                if (f_tone - n_tone) * expansion_dir >= delta {
                    // Good! Tones satisfy the constraint; no change needed.
                } else {
                    // 2nd round: expand farther to match delta.
                    f_tone = MathUtils::clamp_double(0.0, 100.0, n_tone + delta * expansion_dir);
                    if (f_tone - n_tone) * expansion_dir >= delta {
                        // Good! Tones now satisfy the constraint; no change needed.
                    } else {
                        // 3rd round: contract nearer to match delta.
                        n_tone =
                            MathUtils::clamp_double(0.0, 100.0, f_tone - delta * expansion_dir);
                    }
                }

                // Avoids the 50-59 awkward zone.
                if 50.0 <= n_tone && n_tone < 60.0 {
                    // If `nearer` is in the awkward zone, move it away, together with
                    // `farther`.
                    if expansion_dir > 0.0 {
                        n_tone = 60.0;
                        f_tone = f_tone.max(n_tone + delta * expansion_dir);
                    } else {
                        n_tone = 49.0;
                        f_tone = f_tone.min(n_tone + delta * expansion_dir);
                    }
                } else if 50.0 <= f_tone && f_tone < 60.0 {
                    if stay_together {
                        // Fixes both, to avoid two colors on opposite sides of the "awkward
                        // zone".
                        if expansion_dir > 0.0 {
                            n_tone = 60.0;
                            f_tone = f_tone.max(n_tone + delta * expansion_dir);
                        } else {
                            n_tone = 49.0;
                            f_tone = f_tone.min(n_tone + delta * expansion_dir);
                        }
                    } else {
                        // Not required to stay together; fixes just one.
                        if expansion_dir > 0.0 {
                            f_tone = 60.0;
                        } else {
                            f_tone = 49.0;
                        }
                    }
                }

                // Returns `nTone` if this color is `nearer`, otherwise `fTone`.
                return if am_nearer { n_tone } else { f_tone };
            }
            // Case 2: No contrast pair; just solve for itself.
            None => {
                let answer = (self.tone)(scheme);
                let this_background_null = self.background.as_ref().is_none();
                if this_background_null {
                    return answer;
                }
                /*
                final bgTone = this.background!(scheme).getTone(scheme);

                final desiredRatio = this.contrastCurve!.get(scheme.contrastLevel);

                if (Contrast.ratioOfTones(bgTone, answer) >= desiredRatio) {
                  // Don't "improve" what's good enough.
                } else {
                  // Rough improvement.
                  answer = DynamicColor.foregroundTone(bgTone, desiredRatio);
                }

                if (decreasingContrast) {
                  answer = DynamicColor.foregroundTone(bgTone, desiredRatio);
                }

                if (this.isBackground && 50 <= answer && answer < 60) {
                  // Must adjust
                  if (Contrast.ratioOfTones(49, bgTone) >= desiredRatio) {
                    answer = 49;
                  } else {
                    answer = 60;
                  }
                }

                if (this.secondBackground != null) {
                  // Case 3: Adjust for dual backgrounds.

                  final bgTone1 = this.background!(scheme).getTone(scheme);
                  final bgTone2 = this.secondBackground!(scheme).getTone(scheme);

                  final upper = math.max(bgTone1, bgTone2);
                  final lower = math.min(bgTone1, bgTone2);

                  if (Contrast.ratioOfTones(upper, answer) >= desiredRatio &&
                      Contrast.ratioOfTones(lower, answer) >= desiredRatio) {
                    return answer;
                  }

                  // The darkest light tone that satisfies the desired ratio,
                  // or -1 if such ratio cannot be reached.
                  final lightOption = Contrast.lighter(tone: upper, ratio: desiredRatio);

                  // The lightest dark tone that satisfies the desired ratio,
                  // or -1 if such ratio cannot be reached.
                  final darkOption = Contrast.darker(tone: lower, ratio: desiredRatio);

                  // Tones suitable for the foreground.
                  final availables = [];
                  if (lightOption != -1) availables.add(lightOption);
                  if (darkOption != -1) availables.add(darkOption);

                  final prefersLight = DynamicColor.tonePrefersLightForeground(bgTone1) ||
                      DynamicColor.tonePrefersLightForeground(bgTone2);
                  if (prefersLight) {
                    return (lightOption < 0) ? 100 : lightOption;
                  }
                  if (availables.length == 1) {
                    return availables[0];
                  }
                  return (darkOption < 0) ? 0 : darkOption;
                }

                return answer;
                        */
                todo!("case 2")
            }
        };
    }

    /// Given a background tone, find a foreground tone, while ensuring they reach
    /// a contrast ratio that is as close to [ratio] as possible.
    ///
    /// [bgTone] Tone in HCT. Range is 0 to 100, undefined behavior when it falls
    /// outside that range.
    /// [ratio] The contrast ratio desired between [bgTone] and the return value.
    fn foreground_tone(_bg_tone: f64, _ratio: f64) -> f64 {
        todo!("foreground tone")
    }
    /*   static double foregroundTone(double bgTone, double ratio) {
      final lighterTone = Contrast.lighterUnsafe(tone: bgTone, ratio: ratio);
      final darkerTone = Contrast.darkerUnsafe(tone: bgTone, ratio: ratio);
      final lighterRatio = Contrast.ratioOfTones(lighterTone, bgTone);
      final darkerRatio = Contrast.ratioOfTones(darkerTone, bgTone);
      final preferLighter = tonePrefersLightForeground(bgTone);

      if (preferLighter) {
        // This handles an edge case where the initial contrast ratio is high
        // (ex. 13.0), and the ratio passed to the function is that high ratio,
        // and both the lighter and darker ratio fails to pass that ratio.
        //
        // This was observed with Tonal Spot's On Primary Container turning black
        // momentarily between high and max contrast in light mode.
        // PC's standard tone was T90, OPC's was T10, it was light mode, and the
        // contrast value was 0.6568521221032331.
        final negligibleDifference = ((lighterRatio - darkerRatio).abs() < 0.1 &&
            lighterRatio < ratio &&
            darkerRatio < ratio);
        return lighterRatio >= ratio ||
                lighterRatio >= darkerRatio ||
                negligibleDifference
            ? lighterTone
            : darkerTone;
      } else {
        return darkerRatio >= ratio || darkerRatio >= lighterRatio
            ? darkerTone
            : lighterTone;
      }
    } */
}
