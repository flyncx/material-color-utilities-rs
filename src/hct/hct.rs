use crate::utils::color_utils::ColorUtils;
use core::hash::Hash;
use std::hash::Hasher;

use super::{cam16::Cam16, src::hct_solver::HctSolver, viewing_conditions::ViewingConditions};

/// HCT, hue, chroma, and tone. A color system that provides a perceptually
/// accurate color measurement system that can also accurately render what
/// colors will appear as in different lighting environments.
#[derive(Clone)]
pub struct Hct {
    _hue: f64,
    _chroma: f64,
    _tone: f64,
    _argb: i64,
}

impl Eq for Hct {}
impl PartialEq for Hct {
    fn eq(&self, o: &Self) -> bool {
        return o._argb == self._argb;
    }
}
impl Hash for Hct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._argb.hash(state);
    }
}

impl Hct {
    /// 0 <= [hue] < 360; invalid values are corrected.
    /// 0 <= [chroma] <= ?; Informally, colorfulness. The color returned may be
    ///    lower than the requested chroma. Chroma has a different maximum for any
    ///    given hue and tone.
    /// 0 <= [tone] <= 100; informally, lightness. Invalid values are corrected.
    pub fn from(hue: f64, chroma: f64, tone: f64) -> Hct {
        let argb = HctSolver::solve_to_int(hue, chroma, tone);
        return Hct::new(argb);
    }

    /// HCT representation of [argb].
    pub fn from_int(argb: i64) -> Hct {
        return Hct::new(argb);
    }

    pub fn to_int(&self) -> i64 {
        return self._argb;
    }

    /// A number, in degrees, representing ex. red, orange, yellow, etc.
    /// Ranges from 0 <= [hue] < 360
    pub fn get_hue(&self) -> f64 {
        return self._hue;
    }

    /// 0 <= [newHue] < 360; invalid values are corrected.
    /// After setting hue, the color is mapped from HCT to the more
    /// limited sRGB gamut for display. This will change its ARGB/integer
    /// representation. If the HCT color is outside of the sRGB gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_hue(&mut self, new_hue: f64) {
        self._argb = HctSolver::solve_to_int(new_hue, self.get_chroma(), self.get_tone());
        let cam16 = Cam16::from_int(self._argb);
        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = ColorUtils::lstar_from_argb(self._argb);
    }

    pub fn get_chroma(&self) -> f64 {
        return self._chroma;
    }

    /// 0 <= [newChroma] <= ?
    /// After setting chroma, the color is mapped from HCT to the more
    /// limited sRGB gamut for display. This will change its ARGB/integer
    /// representation. If the HCT color is outside of the sRGB gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_chroma(&mut self, new_chroma: f64) {
        self._argb = HctSolver::solve_to_int(self.get_hue(), new_chroma, self.get_tone());
        let cam16 = Cam16::from_int(self._argb);
        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = ColorUtils::lstar_from_argb(self._argb);
    }

    /// Lightness. Ranges from 0 to 100.
    pub fn get_tone(&self) -> f64 {
        return self._tone;
    }

    /// 0 <= [newTone] <= 100; invalid values are corrected.
    /// After setting tone, the color is mapped from HCT to the more
    /// limited sRGB gamut for display. This will change its ARGB/integer
    /// representation. If the HCT color is outside of the sRGB gamut, chroma
    /// will decrease until it is inside the gamut.
    pub fn set_tone(&mut self, new_tone: f64) {
        self._argb = HctSolver::solve_to_int(self.get_hue(), self.get_chroma(), new_tone);
        let cam16 = Cam16::from_int(self._argb);
        self._hue = cam16.hue;
        self._chroma = cam16.chroma;
        self._tone = ColorUtils::lstar_from_argb(self._argb);
    }

    pub fn new(argb: i64) -> Hct {
        let _argb = argb;
        let cam16 = Cam16::from_int(argb);
        let _hue = cam16.hue;
        let _chroma = cam16.chroma;
        let _tone = ColorUtils::lstar_from_argb(_argb);

        return Hct {
            _hue,
            _chroma,
            _tone,
            _argb,
        };
    }
    /// Translate a color into different [ViewingConditions].
    ///
    /// Colors change appearance. They look different with lights on versus off,
    /// the same color, as in hex code, on white looks different when on black.
    /// This is called color relativity, most famously explicated by Josef Albers
    /// in Interaction of Color.
    ///
    /// In color science, color appearance models can account for this and
    /// calculate the appearance of a color in different settings. HCT is based on
    /// CAM16, a color appearance model, and uses it to make these calculations.
    ///
    /// See [ViewingConditions.make] for parameters affecting color appearance.
    pub fn in_viewing_conditions(&self, vc: ViewingConditions) -> Hct {
        // 1. Use CAM16 to find XYZ coordinates of color in specified VC.
        let cam16 = Cam16::from_int(self.to_int());
        let viewed_in_vc = cam16.xyz_in_viewing_conditions(vc, None);

        // 2. Create CAM16 of those XYZ coordinates in default VC.
        let recast_in_vc = Cam16::from_xyz_in_viewing_conditions(
            viewed_in_vc[0],
            viewed_in_vc[1],
            viewed_in_vc[2],
            ViewingConditions::make(None, None, None, None, None),
        );

        // 3. Create HCT from:
        // - CAM16 using default VC with XYZ coordinates in specified VC.
        // - L* converted from Y in XYZ coordinates in specified VC.
        let recast_hct = Hct::from(
            recast_in_vc.hue,
            recast_in_vc.chroma,
            ColorUtils::lstar_from_y(viewed_in_vc[1]),
        );
        return recast_hct;
    }
}

impl ToString for Hct {
    fn to_string(&self) -> String {
        format!(
            "H{} C{} T{}",
            self.get_hue().round().to_string(),
            self.get_chroma().round(),
            self.get_tone().round().to_string()
        )
    }
}
