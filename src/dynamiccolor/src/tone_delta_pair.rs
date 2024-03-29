use crate::dynamiccolor::dynamic_color::DynamicColor;

#[derive(PartialEq)]
pub enum TonePolarity {
    Darker,
    Lighter,
    Nearer,
    Farther,
}

/// Documents a constraint between two DynamicColors, in which their tones must
/// have a certain distance from each other.
///
/// Prefer a DynamicColor with a background, this is for special cases when
/// designers want tonal distance, literally contrast, between two colors that
/// don't have a background / foreground relationship or a contrast guarantee.
pub struct ToneDeltaPair {
    pub role_a: DynamicColor,
    pub role_b: DynamicColor,
    pub delta: f64,
    pub polarity: TonePolarity,
    pub stay_together: bool,
}
impl ToneDeltaPair {
    /// Documents a constraint in tone distance between two DynamicColors.
    ///
    /// The polarity is an adjective that describes "A", compared to "B".
    ///
    /// For instance, ToneDeltaPair(A, B, 15, 'darker', stayTogether) states that
    /// A's tone should be at least 15 darker than B's.
    ///
    /// 'nearer' and 'farther' describes closeness to the surface roles. For
    /// instance, ToneDeltaPair(A, B, 10, 'nearer', stayTogether) states that A
    /// should be 10 lighter than B in light mode, and 10 darker than B in dark
    /// mode.
    ///
    /// [roleA] The first role in a pair.
    /// [roleB] The second role in a pair.
    /// [delta] Required difference between tones. Absolute value, negative
    /// values have undefined behavior.
    /// [polarity] The relative relation between tones of roleA and roleB,
    /// as described above.
    /// [stayTogether] Whether these two roles should stay on the same side of
    /// the "awkward zone" (T50-59). This is necessary for certain cases where
    /// one role has two backgrounds.
    pub fn new(
        role_a: DynamicColor,
        role_b: DynamicColor,
        delta: f64,
        polarity: TonePolarity,
        stay_together: bool,
    ) -> ToneDeltaPair {
        ToneDeltaPair {
            role_a,
            role_b,
            delta,
            polarity,
            stay_together,
        }
    }
}
