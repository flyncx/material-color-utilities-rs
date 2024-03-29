use crate::hct::hct::Hct;

/// Check and/or fix universally disliked colors.
///
/// Color science studies of color preference indicate universal distaste for
/// dark yellow-greens, and also show this is correlated to distate for
/// biological waste and rotting food.
///
/// See Palmer and Schloss, 2010 or Schloss and Palmer's Chapter 21 in Handbook
/// of Color Psychology (2015).
pub struct DislikeAnalyzer {}
impl DislikeAnalyzer {
    /// Returns true if [hct] is disliked.
    ///
    /// Disliked is defined as a dark yellow-green that is not neutral.
    pub fn is_disliked(hct: &Hct) -> bool {
        let hue_passes = hct.get_hue().round() >= 90.0 && hct.get_hue().round() <= 111.0;
        let chroma_passes = hct.get_chroma().round() > 16.0;
        let tone_passes = hct.get_tone().round() < 65.0;

        return hue_passes && chroma_passes && tone_passes;
    }

    /// If [hct] is disliked, lighten it to make it likable.
    pub fn fix_if_disliked(hct: &Hct) -> Hct {
        if Self::is_disliked(hct) {
            return Hct::from(hct.get_hue(), hct.get_chroma(), 70.0);
        } else {
            hct.clone()
        }
    }
}
