use crate::utils::{color_utils::ColorUtils, math_utils::MathUtils};

/// In traditional color spaces, a color can be identified solely by the
/// observer's measurement of the color. Color appearance models such as CAM16
/// also use information about the environment where the color was
/// observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white
/// point is accurately measured as a slightly chromatic blue by CAM16.
/// (roughly, hue 203, chroma 3, lightness 100)
///
/// This class caches intermediate values of the CAM16 conversion process that
/// depend only on viewing conditions, enabling speed ups.
pub struct ViewingConditions {
    pub white_point: Vec<f64>,
    pub adapting_luminance: f64,
    pub background_lstar: f64,
    pub surround: f64,
    pub discounting_illuminant: bool,
    pub background_y_to_white_point_y: f64,
    pub aw: f64,
    pub nbb: f64,
    pub ncb: f64,
    pub c: f64,
    pub n_c: f64,
    pub drgb_inverse: Vec<f64>,
    pub rgb_d: Vec<f64>,
    pub fl: f64,
    pub f_lroot: f64,
    pub z: f64,
}
impl ViewingConditions {
    pub fn s_rgb() -> ViewingConditions {
        ViewingConditions::make(None, None, None, None, None)
    }
    pub fn standard() -> ViewingConditions {
        Self::s_rgb()
    }

    /// Convenience constructor for [ViewingConditions].
    ///
    /// Parameters affecting color appearance include:
    /// [whitePoint]: coordinates of white in XYZ color space.
    /// [adaptingLuminance]: light strength, in lux.
    /// [backgroundLstar]: average luminance of 10 degrees around color.
    /// [surround]: brightness of the entire environment.
    /// [discountingIlluminant]: whether eyes have adjusted to lighting.
    pub fn make(
        white_point: Option<Vec<f64>>,
        adapting_luminance: Option<f64>,
        background_lstar: Option<f64>,
        surround: Option<f64>,
        discounting_illuminant: Option<bool>,
    ) -> ViewingConditions {
        let adapting_luminance = {
            let adapting_luminance = adapting_luminance.unwrap_or(-1.0);
            if adapting_luminance > 0.0 {
                adapting_luminance
            } else {
                200.0 / std::f64::consts::PI * ColorUtils::y_from_lstar(50.0) / 100.0
            }
        };
        let mut background_lstar = background_lstar.unwrap_or(50.0);
        let surround = surround.unwrap_or(2.0);
        let discounting_illuminant = discounting_illuminant.unwrap_or(false);
        let white_point = white_point.unwrap_or(ColorUtils::white_point_d65());

        // A background of pure black is non-physical and leads to infinities that
        // represent the idea that any color viewed in pure black can't be seen.
        background_lstar = 0.1f64.max(background_lstar);

        // Transform test illuminant white in XYZ to 'cone'/'rgb' responses
        let xyz = white_point.clone();
        let r_w = xyz[0] * 0.401288 + xyz[1] * 0.650173 + xyz[2] * -0.051461;
        let g_w = xyz[0] * -0.250268 + xyz[1] * 1.204414 + xyz[2] * 0.045854;
        let b_w = xyz[0] * -0.002079 + xyz[1] * 0.048952 + xyz[2] * 0.953127;

        // Scale input surround, domain (0, 2), to CAM16 surround, domain (0.8, 1.0)
        assert!(surround >= 0.0 && surround <= 2.0);
        let f = 0.8 + (surround / 10.0);
        // "Exponential non-linearity"
        let c = {
            if f >= 0.9 {
                MathUtils::lerp(0.59, 0.69, (f - 0.9) * 10.0)
            } else {
                MathUtils::lerp(0.525, 0.59, (f - 0.8) * 10.0)
            }
        };

        // Calculate degree of adaptation to illuminant
        let mut d = {
            if discounting_illuminant {
                1.0
            } else {
                f * (1.0 - ((1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp()))
            }
        };

        // Per Li et al, if D is greater than 1 or less than 0, set it to 1 or 0.
        d = {
            if d > 1.0 {
                1.0
            } else if d < 0.0 {
                0.0
            } else {
                d
            }
        };

        // chromatic induction factor
        let n_c = f;

        // Cone responses to the whitePoint, r/g/b/W, adjusted for discounting.
        //
        // Why use 100.0 instead of the white point's relative luminance?
        //
        // Some papers and implementations, for both CAM02 and CAM16, use the Y
        // value of the reference white instead of 100. Fairchild's Color Appearance
        // Models (3rd edition) notes that this is in error: it was included in the
        // CIE 2004a report on CIECAM02, but, later parts of the conversion process
        // account for scaling of appearance relative to the white point relative
        // luminance. This part should simply use 100 as luminance.
        let rgb_d = [
            d * (100.0 / r_w) + 1.0 - d,
            d * (100.0 / g_w) + 1.0 - d,
            d * (100.0 / b_w) + 1.0 - d,
        ];

        // Factor used in calculating meaningful factors
        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4_f = 1.0 - k4;

        // Luminance-level adaptation factor
        let fl = (k4 * adapting_luminance)
            + (0.1 * k4_f * k4_f * (5.0 * adapting_luminance).powf(1.0 / 3.0));

        // Intermediate factor, ratio of background relative luminance to white relative luminance
        let n = ColorUtils::y_from_lstar(background_lstar) / white_point[1];

        // Base exponential nonlinearity
        // note Schlomer 2018 has a typo and uses 1.58, the correct factor is 1.48
        let z = 1.48 + (n.sqrt());

        // Luminance-level induction factors
        let nbb = 0.725 / (n).powf(0.2);
        let ncb = nbb;

        // Discounted cone responses to the white point, adjusted for post-saturationtic
        // adaptation perceptual nonlinearities.
        let rgb_afactors = [
            (fl * rgb_d[0] * r_w / 100.0).powf(0.42),
            (fl * rgb_d[1] * g_w / 100.0).powf(0.42),
            (fl * rgb_d[2] * b_w / 100.0).powf(0.42),
        ];

        let rgb_a = [
            (400.0 * rgb_afactors[0]) / (rgb_afactors[0] + 27.13),
            (400.0 * rgb_afactors[1]) / (rgb_afactors[1] + 27.13),
            (400.0 * rgb_afactors[2]) / (rgb_afactors[2] + 27.13),
        ];

        let aw = (40.0 * rgb_a[0] + 20.0 * rgb_a[1] + rgb_a[2]) / 20.0 * nbb;

        ViewingConditions {
            white_point,
            adapting_luminance,
            background_lstar,
            surround,
            discounting_illuminant,
            background_y_to_white_point_y: n,
            aw,
            nbb,
            ncb,
            c,
            n_c,
            drgb_inverse: [0.0, 0.0, 0.0].to_vec(),
            rgb_d: rgb_d.to_vec(),
            fl,
            f_lroot: fl.powf(0.25),
            z,
        }
    }
}