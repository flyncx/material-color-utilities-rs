use crate::utils::{color_utils::ColorUtils, math_utils::MathUtils};

/// Utility methods for calculating contrast given two colors, or calculating a
/// color given one color and a contrast ratio.
///
/// Contrast ratio is calculated using XYZ's Y. When linearized to match human
/// perception, Y becomes HCT's tone and L\*a\*b\*'s' L\*. Informally, this is the
/// lightness of a color.
///
/// Methods refer to tone, T in the the HCT color space.
/// Tone is equivalent to L\* in the L\*a\*b\* color space, or L in the LCH color
/// space.
pub struct Contrast {}
impl Contrast {
    /// Returns a contrast ratio, which ranges from 1 to 21.
    /// [toneA] Tone between 0 and 100. Values outside will be clamped.
    /// [toneB] Tone between 0 and 100. Values outside will be clamped.
    pub fn ratio_of_tones(tone_a: f64, tone_b: f64) -> f64 {
        let tone_a = MathUtils::clamp_double(0.0, 100.0, tone_a);
        let tone_b = MathUtils::clamp_double(0.0, 100.0, tone_b);
        Self::_ratio_of_ys(
            ColorUtils::y_from_lstar(tone_a),
            ColorUtils::y_from_lstar(tone_b),
        )
    }

    fn _ratio_of_ys(y1: f64, y2: f64) -> f64 {
        let lighter = {
            if y1 > y2 {
                y1
            } else {
                y2
            }
        };
        let darker = {
            if lighter == y2 {
                y1
            } else {
                y2
            }
        };
        (lighter + 5.0) / (darker + 5.0)
    }

    /// Returns a tone >= [tone] that ensures [ratio].
    /// Return value is between 0 and 100.
    /// Returns -1 if [ratio] cannot be achieved with [tone].
    ///
    /// [tone] Tone return value must contrast with.
    /// Range is 0 to 100. Invalid values will result in -1 being returned.
    /// [ratio] Contrast ratio of return value and [tone].
    /// Range is 1 to 21, invalid values have undefined behavior.
    pub fn lighter(tone: f64, ratio: f64) -> f64 {
        if tone < 0.0 || tone > 100.0 {
            return -1.0;
        }

        let dark_y = ColorUtils::y_from_lstar(tone);
        let light_y = ratio * (dark_y + 5.0) - 5.0;
        let real_contrast = Self::_ratio_of_ys(light_y, dark_y);
        let delta = (real_contrast - ratio).abs();
        if real_contrast < ratio && delta > 0.04 {
            return -1.0;
        }

        // Ensure gamut mapping, which requires a 'range' on tone, will still result
        // the correct ratio by darkening slightly.
        let return_value = ColorUtils::lstar_from_y(light_y) + 0.4;
        if return_value < 0.0 || return_value > 100.0 {
            return -1.0;
        }
        return_value
    }

    /// Returns a tone <= [tone] that ensures [ratio].
    /// Return value is between 0 and 100.
    /// Returns -1 if [ratio] cannot be achieved with [tone].
    ///
    /// [tone] Tone return value must contrast with.
    /// Range is 0 to 100. Invalid values will result in -1 being returned.
    /// [ratio] Contrast ratio of return value and [tone].
    /// Range is 1 to 21, invalid values have undefined behavior.
    pub fn darker(tone: f64, ratio: f64) -> f64 {
        if tone < 0.0 || tone > 100.0 {
            return -1.0;
        }

        let light_y = ColorUtils::y_from_lstar(tone);
        let dark_y = ((light_y + 5.0) / ratio) - 5.0;
        let real_contrast = Self::_ratio_of_ys(light_y, dark_y);

        let delta = (real_contrast - ratio).abs();
        if real_contrast < ratio && delta > 0.04 {
            return -1.0;
        }

        // Ensure gamut mapping, which requires a 'range' on tone, will still result
        // the correct ratio by darkening slightly.
        let return_value = ColorUtils::lstar_from_y(dark_y) - 0.4;
        if return_value < 0.0 || return_value > 100.0 {
            return -1.0;
        }
        return_value
    }

    /// Returns a tone >= [tone] that ensures [ratio].
    /// Return value is between 0 and 100.
    /// Returns 100 if [ratio] cannot be achieved with [tone].
    ///
    /// This method is unsafe because the returned value is guaranteed to be in
    /// bounds for tone, i.e. between 0 and 100. However, that value may not reach
    /// the [ratio] with [tone]. For example, there is no color lighter than T100.
    ///
    /// [tone] Tone return value must contrast with.
    /// Range is 0 to 100. Invalid values will result in 100 being returned.
    /// [ratio] Desired contrast ratio of return value and tone parameter.
    /// Range is 1 to 21, invalid values have undefined behavior.
    pub fn lighter_unsafe(tone: f64, ratio: f64) -> f64 {
        let lighter_safe = Self::lighter(tone, ratio);
        if lighter_safe < 0.0 {
            100.0
        } else {
            lighter_safe
        }
    }

    /// Returns a tone <= [tone] that ensures [ratio].
    /// Return value is between 0 and 100.
    /// Returns 0 if [ratio] cannot be achieved with [tone].
    ///
    /// This method is unsafe because the returned value is guaranteed to be in
    /// bounds for tone, i.e. between 0 and 100. However, that value may not reach
    /// the [ratio] with [tone]. For example, there is no color darker than T0.
    ///
    /// [tone] Tone return value must contrast with.
    /// Range is 0 to 100. Invalid values will result in 0 being returned.
    /// [ratio] Desired contrast ratio of return value and tone parameter.
    /// Range is 1 to 21, invalid values have undefined behavior.
    pub fn darker_unsafe(tone: f64, ratio: f64) -> f64 {
        let darker_safe = Self::darker(tone, ratio);
        return if darker_safe < 0.0 { 0.0 } else { darker_safe };
    }
}
