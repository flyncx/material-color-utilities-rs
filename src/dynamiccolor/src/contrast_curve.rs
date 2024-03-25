use crate::utils::math_utils::MathUtils;

/// A class containing a value that changes with the contrast level.
///
/// Usually represents the contrast requirements for a dynamic color on its
/// background. The four values correspond to values for contrast levels
/// -1.0, 0.0, 0.5, and 1.0, respectively.
#[derive(Clone)]
pub struct ContrastCurve {
    pub low: f64,
    pub normal: f64,
    pub medium: f64,
    pub high: f64,
}
impl ContrastCurve {
    /// Creates a `ContrastCurve` object.
    ///
    /// [low] Value for contrast level -1.0
    /// [normal] Value for contrast level 0.0
    /// [medium] Value for contrast level 0.5
    /// [high] Value for contrast level 1.0
    pub fn new(low: f64, normal: f64, medium: f64, high: f64) -> ContrastCurve {
        ContrastCurve {
            low,
            normal,
            medium,
            high,
        }
    }

    /// Returns the value at a given contrast level.
    ///
    /// [contrastLevel] The contrast level. 0.0 is the default (normal);
    /// -1.0 is the lowest; 1.0 is the highest.
    /// Returns the value. For contrast ratios, a number between 1.0 and 21.0.
    pub fn get(&self, contrast_level: f64) -> f64 {
        if contrast_level <= -1.0 {
            return self.low;
        } else if contrast_level < 0.0 {
            return MathUtils::lerp(self.low, self.normal, (contrast_level - (-1.0)) / 1.0);
        } else if contrast_level < 0.5 {
            return MathUtils::lerp(self.normal, self.medium, (contrast_level - 0.0) / 0.5);
        } else if contrast_level < 1.0 {
            return MathUtils::lerp(self.medium, self.high, (contrast_level - 0.5) / 0.5);
        } else {
            return self.high;
        }
    }
}
