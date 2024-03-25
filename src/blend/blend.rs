use crate::{
    hct::{cam16::Cam16, hct::Hct},
    utils::{color_utils::ColorUtils, math_utils::MathUtils},
};

/// Functions for blending in HCT and CAM16.
pub struct Blend {}
impl Blend {
    /// Blend the design color's HCT hue towards the key color's HCT
    /// hue, in a way that leaves the original color recognizable and
    /// recognizably shifted towards the key color.
    ///
    /// [designColor] ARGB representation of an arbitrary color.
    /// [sourceColor] ARGB representation of the main theme color.
    /// Returns The design color with a hue shifted towards the
    /// system's color, a slightly warmer/cooler variant of the design
    /// color's hue.
    pub fn harmonize(design_color: i64, source_color: i64) -> i64 {
        let from_hct = Hct::from_int(design_color);
        let to_hct = Hct::from_int(source_color);
        let difference_degrees =
            MathUtils::difference_degrees(from_hct.get_hue(), to_hct.get_hue());
        let rotation_degrees = (difference_degrees * 0.5).min(15.0);
        let output_hue = MathUtils::sanitize_degrees_double(
            from_hct.get_hue()
                + rotation_degrees
                    * MathUtils::rotation_direction(from_hct.get_hue(), to_hct.get_hue()),
        );
        Hct::from(output_hue, from_hct.get_chroma(), from_hct.get_tone()).to_int()
    }

    /// Blends hue from one color into another. The chroma and tone of
    /// the original color are maintained.
    ///
    /// [from] ARGB representation of color
    /// [to] ARGB representation of color
    /// [amount] how much blending to perform; 0.0 >= and <= 1.0
    /// Returns from, with a hue blended towards to. Chroma and tone
    /// are constant.
    pub fn hct_hue(from: i64, to: i64, amount: f64) -> i64 {
        let ucs = Self::cam16_ucs(from, to, amount);
        let ucs_cam = Cam16::from_int(ucs);
        let from_cam = Cam16::from_int(from);
        let blended = Hct::from(
            ucs_cam.hue,
            from_cam.chroma,
            ColorUtils::lstar_from_argb(from),
        );
        blended.to_int()
    }

    /// Blend in CAM16-UCS space.
    ///
    /// [from] ARGB representation of color
    /// [to] ARGB representation of color
    /// [amount] how much blending to perform; 0.0 >= and <= 1.0
    /// Returns from, blended towards to. Hue, chroma, and tone will
    /// change.
    pub fn cam16_ucs(from: i64, to: i64, amount: f64) -> i64 {
        let from_cam = Cam16::from_int(from);
        let to_cam = Cam16::from_int(to);
        let from_j = from_cam.jstar;
        let from_a = from_cam.astar;
        let from_b = from_cam.bstar;
        let to_j = to_cam.jstar;
        let to_a = to_cam.astar;
        let to_b = to_cam.bstar;
        let jstar = from_j + (to_j - from_j) * amount;
        let astar = from_a + (to_a - from_a) * amount;
        let bstar = from_b + (to_b - from_b) * amount;
        Cam16::from_ucs(jstar, astar, bstar).to_int()
    }
}
