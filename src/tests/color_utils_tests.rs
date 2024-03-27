#[cfg(test)]
pub mod color_utils_tests {
    use crate::utils::color_utils::ColorUtils;

    fn _range(start: f64, stop: f64, case_count: i64) -> Vec<f64> {
        let step_size: f64 = (stop - start) / (case_count - 1) as f64;

        (0..case_count)
            .map(|index| start + step_size * index as f64)
            .collect()
    }
    fn _get_rgb_range() -> Vec<i64> {
        _range(0.0, 255.0, 8)
            .iter()
            .map(|element| element.round() as i64)
            .collect()
    }
    fn _get_full_rgb_range() -> Vec<i64> {
        (0..256).map(|index| index).collect()
    }

    #[test]
    fn range_integrity() {
        let range = _range(3.0, 9999.0, 1234);
        for i in 0..1234 {
            assert_approx_eq::assert_approx_eq!(range[i], 3.0 + 8.1070559611 * i as f64, 1e-5);
        }
    }

    #[test]
    fn y_to_lstar_to_y() {
        for y in _range(0.0, 100.0, 1001) {
            assert_approx_eq::assert_approx_eq!(
                ColorUtils::y_from_lstar(ColorUtils::lstar_from_y(y)),
                y,
                1e-5
            );
        }
    }

    #[test]
    fn lstar_to_y_to_lstar() {
        for lstar in _range(0.0, 100.0, 1001) {
            assert_approx_eq::assert_approx_eq!(
                ColorUtils::lstar_from_y(ColorUtils::y_from_lstar(lstar)),
                lstar,
                1e-5
            );
        }
    }

    #[test]
    fn y_from_lstar() {
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(0.0), 0.0, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(0.1), 0.0110705, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(0.2), 0.0221411, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(0.3), 0.0332116, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(0.4), 0.0442822, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(0.5), 0.0553528, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(1.0), 0.1107056, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(2.0), 0.2214112, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(3.0), 0.3321169, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(4.0), 0.4428225, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(5.0), 0.5535282, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(8.0), 0.8856451, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(10.0), 1.1260199, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(15.0), 1.9085832, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(20.0), 2.9890524, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(25.0), 4.4154767, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(30.0), 6.2359055, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(40.0), 11.2509737, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(50.0), 18.4186518, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(60.0), 28.1233342, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(70.0), 40.7494157, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(80.0), 56.6812907, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(90.0), 76.3033539, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(95.0), 87.6183294, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(99.0), 97.4360239, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::y_from_lstar(100.0), 100.0, 1e-5);
    }

    #[test]
    fn lstar_from_y() {
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.0), 0.0, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.1), 0.9032962, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.2), 1.8065925, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.3), 2.7098888, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.4), 3.6131851, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.5), 4.5164814, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(0.88564), 8.0, 1e-4);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(1.0), 8.9914424, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(2.0), 15.4872443, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(3.0), 20.0438970, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(4.0), 23.6714419, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(5.0), 26.7347653, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(10.0), 37.8424304, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(15.0), 45.6341970, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(20.0), 51.8372115, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(25.0), 57.0754208, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(30.0), 61.6542222, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(40.0), 69.4695307, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(50.0), 76.0692610, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(60.0), 81.8381891, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(70.0), 86.9968642, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(80.0), 91.6848609, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(90.0), 95.9967686, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(95.0), 98.0335184, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(99.0), 99.6120372, 1e-5);
        assert_approx_eq::assert_approx_eq!(ColorUtils::lstar_from_y(100.0), 100.0, 1e-5);
    }

    #[test]
    fn y_continuity() {
        let epsilon = 1e-6;
        let delta = 1e-8;
        let left = 8.0 - delta;
        let mid = 8.0;
        let right = 8.0 + delta;
        assert_approx_eq::assert_approx_eq!(
            ColorUtils::y_from_lstar(left),
            ColorUtils::y_from_lstar(mid),
            epsilon
        );
        assert_approx_eq::assert_approx_eq!(
            ColorUtils::y_from_lstar(right),
            ColorUtils::y_from_lstar(mid),
            epsilon
        );
    }

    #[test]
    fn rgb_to_xyz_to_rgb() {
        for r in _get_rgb_range() {
            for g in _get_full_rgb_range() {
                for b in _get_rgb_range() {
                    let argb = ColorUtils::argb_from_rgb(r, g, b);
                    let xyz = ColorUtils::xyz_from_argb(argb);
                    let converted = ColorUtils::argb_from_xyz(xyz[0], xyz[1], xyz[2]);
                    assert_approx_eq::assert_approx_eq!(
                        ColorUtils::red_from_argb(converted) as f64,
                        r as f64,
                        1.5
                    );
                    assert_approx_eq::assert_approx_eq!(
                        ColorUtils::green_from_argb(converted) as f64,
                        g as f64,
                        1.5
                    );
                    assert_approx_eq::assert_approx_eq!(
                        ColorUtils::blue_from_argb(converted) as f64,
                        b as f64,
                        1.5
                    );
                }
            }
        }
    }

    #[test]
    fn rgb_to_lab_to_rgb() {
        for r in _get_rgb_range() {
            for g in _get_rgb_range() {
                for b in _get_rgb_range() {
                    let argb = ColorUtils::argb_from_rgb(r, g, b);
                    let lab = ColorUtils::lab_from_argb(argb);
                    let converted = ColorUtils::argb_from_lab(lab[0], lab[1], lab[2]);
                    assert_approx_eq::assert_approx_eq!(
                        ColorUtils::red_from_argb(converted) as f64,
                        r as f64,
                        1.5
                    );
                    assert_approx_eq::assert_approx_eq!(
                        ColorUtils::green_from_argb(converted) as f64,
                        g as f64,
                        1.5
                    );
                    assert_approx_eq::assert_approx_eq!(
                        ColorUtils::blue_from_argb(converted) as f64,
                        b as f64,
                        1.5
                    );
                }
            }
        }
    }

    #[test]
    fn rgb_to_lstar_to_rgb() {
        for component in _get_full_rgb_range() {
            let argb = ColorUtils::argb_from_rgb(component, component, component);
            let lstar = ColorUtils::lstar_from_argb(argb);
            let converted = ColorUtils::argb_from_lstar(lstar);
            assert_eq!(converted, argb);
        }
    }

    #[test]
    fn rgb_to_lstar_to_y_commutes() {
        for r in _get_rgb_range() {
            for g in _get_rgb_range() {
                for b in _get_rgb_range() {
                    let argb = ColorUtils::argb_from_rgb(r, g, b);
                    let lstar = ColorUtils::lstar_from_argb(argb);
                    let y = ColorUtils::y_from_lstar(lstar);
                    let y2 = ColorUtils::xyz_from_argb(argb)[1];
                    assert_approx_eq::assert_approx_eq!(y, y2, 1e-5);
                }
            }
        }
    }

    #[test]
    fn lstar_to_rgb_to_y_commutes() {
        for lstar in _range(0.0, 100.0, 1001) {
            let argb = ColorUtils::argb_from_lstar(lstar);
            let y = ColorUtils::xyz_from_argb(argb)[1];
            let y2 = ColorUtils::y_from_lstar(lstar);
            assert_approx_eq::assert_approx_eq!(y, y2, 1.0);
        }
    }

    #[test]
    fn linearize_delinearize() {
        for rgb_component in _get_full_rgb_range() {
            let converted = ColorUtils::delinearized(ColorUtils::linearized(rgb_component));
            assert_eq!(converted, rgb_component);
        }
    }
}
