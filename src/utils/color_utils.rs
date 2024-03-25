use super::math_utils::MathUtils;

/// Color science utilities.
///
/// Utility methods for color science constants and color space
/// conversions that aren't HCT or CAM16.
pub struct ColorUtils {}
impl ColorUtils {
    fn _srgb_to_xyz() -> Vec<Vec<f64>> {
        vec![
            vec![0.41233895, 0.35762064, 0.18051042],
            vec![0.2126, 0.7152, 0.0722],
            vec![0.01932141, 0.11916382, 0.95034478],
        ]
    }

    fn _xyz_to_srgb() -> Vec<Vec<f64>> {
        vec![
            vec![
                3.2413774792388685,
                -1.5376652402851851,
                -0.49885366846268053,
            ],
            vec![-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
            vec![
                0.05562093689691305,
                -0.20395524564742123,
                1.0571799111220335,
            ],
        ]
    }

    fn _white_point_d65() -> Vec<f64> {
        vec![95.047, 100.0, 108.883]
    }

    /// Converts a color from RGB components to ARGB format.
    pub fn argb_from_rgb(red: i64, green: i64, blue: i64) -> i64 {
        return 255 << 24 | (red & 255) << 16 | (green & 255) << 8 | blue & 255;
    }
    /// Converts a color from linear RGB components to ARGB format.
    pub fn argb_from_linrgb(linrgb: Vec<f64>) -> i64 {
        let r = Self::delinearized(linrgb[0]);
        let g = Self::delinearized(linrgb[1]);
        let b = Self::delinearized(linrgb[2]);
        return Self::argb_from_rgb(r, g, b);
    }

    /// Returns the alpha component of a color in ARGB format.
    pub fn alpha_from_argb(argb: i64) -> i64 {
        return argb >> 24 & 255;
    }

    /// Returns the red component of a color in ARGB format.
    pub fn red_from_argb(argb: i64) -> i64 {
        return argb >> 16 & 255;
    }

    /// Returns the green component of a color in ARGB format.
    pub fn green_from_argb(argb: i64) -> i64 {
        return argb >> 8 & 255;
    }

    /// Returns the blue component of a color in ARGB format.
    pub fn blue_from_argb(argb: i64) -> i64 {
        return argb & 255;
    }

    /// Returns whether a color in ARGB format is opaque.
    pub fn is_opaque(argb: i64) -> bool {
        return Self::alpha_from_argb(argb) >= 255;
    }

    /// Converts a color from ARGB to XYZ.
    pub fn argb_from_xyz(x: f64, y: f64, z: f64) -> i64 {
        let matrix = Self::_xyz_to_srgb();
        let linear_r = matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z;
        let linear_g = matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z;
        let linear_b = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z;
        let r = Self::delinearized(linear_r);
        let g = Self::delinearized(linear_g);
        let b = Self::delinearized(linear_b);
        return Self::argb_from_rgb(r, g, b);
    }

    /// Converts a color from XYZ to ARGB.
    pub fn xyz_from_argb(argb: i64) -> Vec<f64> {
        let r = Self::linearized(Self::red_from_argb(argb));
        let g = Self::linearized(Self::green_from_argb(argb));
        let b = Self::linearized(Self::blue_from_argb(argb));
        return MathUtils::matrix_multiply(
            [r, g, b].to_vec(),
            Self::_srgb_to_xyz()
                .iter()
                .map(|row| row.to_vec())
                .collect(),
        );
    }

    /// Converts a color represented in Lab color space into an ARGB
    /// integer.
    pub fn argb_from_lab(l: f64, a: f64, b: f64) -> i64 {
        let white_point = Self::_white_point_d65();
        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;
        let x_normalized = Self::_lab_invf(fx);
        let y_normalized = Self::_lab_invf(fy);
        let z_normalized = Self::_lab_invf(fz);
        let x = x_normalized * white_point[0];
        let y = y_normalized * white_point[1];
        let z = z_normalized * white_point[2];
        return Self::argb_from_xyz(x, y, z);
    }

    /// Converts a color from ARGB representation to L*a*b*
    /// representation.
    ///
    /// [argb] the ARGB representation of a color
    /// Returns a Lab object representing the color
    pub fn lab_from_argb(argb: i64) -> Vec<f64> {
        let linear_r = Self::linearized(Self::red_from_argb(argb));
        let linear_g = Self::linearized(Self::green_from_argb(argb));
        let linear_b = Self::linearized(Self::blue_from_argb(argb));
        let matrix = Self::_srgb_to_xyz();
        let x = matrix[0][0] * linear_r + matrix[0][1] * linear_g + matrix[0][2] * linear_b;
        let y = matrix[1][0] * linear_r + matrix[1][1] * linear_g + matrix[1][2] * linear_b;
        let z = matrix[2][0] * linear_r + matrix[2][1] * linear_g + matrix[2][2] * linear_b;
        let white_point = Self::_white_point_d65();
        let x_normalized = x / white_point[0];
        let y_normalized = y / white_point[1];
        let z_normalized = z / white_point[2];
        let fx = Self::_lab_f(x_normalized);
        let fy = Self::_lab_f(y_normalized);
        let fz = Self::_lab_f(z_normalized);
        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);
        return [l, a, b].to_vec();
    }

    /// Converts an L* value to an ARGB representation.
    ///
    /// [lstar] L* in L*a*b*
    /// Returns ARGB representation of grayscale color with lightness
    /// matching L*
    pub fn argb_from_lstar(lstar: f64) -> i64 {
        let y = Self::y_from_lstar(lstar);
        let component = Self::delinearized(y);
        return Self::argb_from_rgb(component, component, component);
    }

    /// Computes the L* value of a color in ARGB representation.
    ///
    /// [argb] ARGB representation of a color
    /// Returns L*, from L*a*b*, coordinate of the color
    pub fn lstar_from_argb(argb: i64) -> f64 {
        let y = Self::xyz_from_argb(argb)[1];
        return 116.0 * Self::_lab_f(y / 100.0) - 16.0;
    }

    /// Converts an L* value to a Y value.
    ///
    /// L* in L*a*b* and Y in XYZ measure the same quantity, luminance.
    ///
    /// L* measures perceptual luminance, a linear scale. Y in XYZ
    /// measures relative luminance, a logarithmic scale.
    ///
    /// [lstar] L* in L*a*b*
    /// Returns Y in XYZ
    pub fn y_from_lstar(lstar: f64) -> f64 {
        return 100.0 * Self::_lab_invf((lstar + 16.0) / 116.0);
    }

    /// Converts a Y value to an L* value.
    ///
    /// L* in L*a*b* and Y in XYZ measure the same quantity, luminance.
    ///
    /// L* measures perceptual luminance, a linear scale. Y in XYZ
    /// measures relative luminance, a logarithmic scale.
    ///
    /// [y] Y in XYZ
    /// Returns L* in L*a*b*
    pub fn lstar_from_y(y: f64) -> f64 {
        return Self::_lab_f(y / 100.0) * 116.0 - 16.0;
    }

    /// Linearizes an RGB component.
    ///
    /// [rgbComponent] 0 <= rgb_component <= 255, represents R/G/B
    /// channel
    /// Returns 0.0 <= output <= 100.0, color channel converted to
    /// linear RGB space
    pub fn linearized(rgb_component: i64) -> f64 {
        let normalized = rgb_component as f64 / 255.0;
        if normalized <= 0.040449936 {
            return normalized / 12.92 * 100.0;
        } else {
            return ((normalized + 0.055) / 1.055).powf(2.4) * 100.0;
        }
    }

    /// Delinearizes an RGB component.
    ///
    /// [rgbComponent] 0.0 <= rgb_component <= 100.0, represents linear
    /// R/G/B channel
    /// Returns 0 <= output <= 255, color channel converted to regular
    /// RGB space
    pub fn delinearized(rgb_component: f64) -> i64 {
        let normalized = rgb_component / 100.0;
        #[allow(unused_assignments)]
        let mut delinearized = 0.0;
        if normalized <= 0.0031308 {
            delinearized = normalized * 12.92;
        } else {
            delinearized = 1.055 * normalized.powf(1.0 / 2.4) - 0.055;
        }
        return MathUtils::clamp_int(0, 255, (delinearized * 255.0).round() as i64);
    }

    /// Returns the standard white point; white on a sunny day.
    ///
    /// Returns The white point
    pub fn white_point_d65() -> Vec<f64> {
        return Self::_white_point_d65();
    }

    fn _lab_f(t: f64) -> f64 {
        let e = 216.0 / 24389.0;
        let kappa = 24389.0 / 27.0;

        if t > e {
            return t.powf(1.0 / 3.0);
        } else {
            return (kappa * t + 16.0) / 116.0;
        }
    }

    pub fn _lab_invf(ft: f64) -> f64 {
        let e = 216.0 / 24389.0;
        let kappa = 24389.0 / 27.0;
        let ft3 = ft * ft * ft;
        if ft3 > e {
            return ft3;
        } else {
            return (116.0 * ft - 16.0) / kappa;
        }
    }
}
