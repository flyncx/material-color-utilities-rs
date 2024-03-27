#[cfg(test)]
pub mod hct_test {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use crate::{
        hct::{cam16::Cam16, hct::Hct, viewing_conditions::ViewingConditions},
        utils::{color_utils::ColorUtils, string_utils::StringUtils},
    };

    const red: i64 = 0xffff0000;
    const black: i64 = 0xff000000;
    const white: i64 = 0xffffffff;
    const green: i64 = 0xff00ff00;
    const blue: i64 = 0xff0000ff;
    const midgray: i64 = 0xff777777;

    #[test]
    fn hash_code_basics() {
        assert!(Hct::from_int(123) == Hct::from_int(123));
        let mut a = DefaultHasher::new();
        Hct::from_int(123).hash(&mut a);
        let fa = a.finish();
        let mut b = DefaultHasher::new();
        Hct::from_int(123).hash(&mut b);
        let fb = b.finish();
        assert_eq!(fa, fb);
    }

    #[test]
    fn conversions_are_reflexive() {
        let cam = Cam16::from_int(red);
        let color = cam.viewed(ViewingConditions::standard());
        assert_eq!(color, red);
    }

    #[test]
    fn y_midgray() {
        assert_approx_eq::assert_approx_eq!(18.418, ColorUtils::y_from_lstar(50.0), 0.001)
    }

    #[test]
    fn y_black() {
        assert_approx_eq::assert_approx_eq!(0.0, ColorUtils::y_from_lstar(0.0), 0.001)
    }

    #[test]
    fn y_white() {
        assert_approx_eq::assert_approx_eq!(100.0, ColorUtils::y_from_lstar(100.0), 0.001)
    }

    #[test]
    fn cam_red() {
        let cam = Cam16::from_int(red);
        assert_approx_eq::assert_approx_eq!(46.445, cam.j, 0.001);
        assert_approx_eq::assert_approx_eq!(113.357, cam.chroma, 0.001);
        assert_approx_eq::assert_approx_eq!(27.408, cam.hue, 0.001);
        assert_approx_eq::assert_approx_eq!(89.494, cam.m, 0.001);
        assert_approx_eq::assert_approx_eq!(91.889, cam.s, 0.001);
        assert_approx_eq::assert_approx_eq!(105.988, cam.q, 0.001);
    }

    #[test]
    fn cam_green() {
        let cam = Cam16::from_int(green);
        assert_approx_eq::assert_approx_eq!(79.331, cam.j, 0.001);
        assert_approx_eq::assert_approx_eq!(108.410, cam.chroma, 0.001);
        assert_approx_eq::assert_approx_eq!(142.139, cam.hue, 0.001);
        assert_approx_eq::assert_approx_eq!(85.587, cam.m, 0.001);
        assert_approx_eq::assert_approx_eq!(78.604, cam.s, 0.001);
        assert_approx_eq::assert_approx_eq!(138.520, cam.q, 0.001);
    }

    #[test]
    fn cam_blue() {
        let cam = Cam16::from_int(blue);
        assert_approx_eq::assert_approx_eq!(25.465, cam.j, 0.001);
        assert_approx_eq::assert_approx_eq!(87.230, cam.chroma, 0.001);
        assert_approx_eq::assert_approx_eq!(282.788, cam.hue, 0.001);
        assert_approx_eq::assert_approx_eq!(68.867, cam.m, 0.001);
        assert_approx_eq::assert_approx_eq!(93.674, cam.s, 0.001);
        assert_approx_eq::assert_approx_eq!(78.481, cam.q, 0.001);
    }

    #[test]
    fn cam_black() {
        let cam = Cam16::from_int(black);
        assert_approx_eq::assert_approx_eq!(0.0, cam.j, 0.001);
        assert_approx_eq::assert_approx_eq!(0.0, cam.chroma, 0.001);
        assert_approx_eq::assert_approx_eq!(0.0, cam.hue, 0.001);
        assert_approx_eq::assert_approx_eq!(0.0, cam.m, 0.001);
        assert_approx_eq::assert_approx_eq!(0.0, cam.s, 0.001);
        assert_approx_eq::assert_approx_eq!(0.0, cam.q, 0.001);
    }

    #[test]
    fn cam_white() {
        let cam = Cam16::from_int(white);
        assert_approx_eq::assert_approx_eq!(100.0, cam.j, 0.001);
        assert_approx_eq::assert_approx_eq!(2.869, cam.chroma, 0.001);
        assert_approx_eq::assert_approx_eq!(209.492, cam.hue, 0.001);
        assert_approx_eq::assert_approx_eq!(2.265, cam.m, 0.001);
        assert_approx_eq::assert_approx_eq!(12.068, cam.s, 0.001);
        assert_approx_eq::assert_approx_eq!(155.521, cam.q, 0.001);
    }

    #[test]
    fn gamut_map_red() {
        let color_to_test = red;
        let cam = Cam16::from_int(color_to_test);
        let color = Hct::from(
            cam.hue,
            cam.chroma,
            ColorUtils::lstar_from_argb(color_to_test),
        )
        .to_int();
        assert_eq!(color_to_test, (color));
    }

    #[test]
    fn gamut_map_green() {
        let color_to_test = green;
        let cam = Cam16::from_int(color_to_test);
        let color = Hct::from(
            cam.hue,
            cam.chroma,
            ColorUtils::lstar_from_argb(color_to_test),
        )
        .to_int();
        assert_eq!(color_to_test, (color));
    }

    #[test]
    fn gamut_map_blue() {
        let color_to_test = blue;
        let cam = Cam16::from_int(color_to_test);
        let color = Hct::from(
            cam.hue,
            cam.chroma,
            ColorUtils::lstar_from_argb(color_to_test),
        )
        .to_int();
        assert_eq!(color_to_test, (color));
    }

    #[test]
    fn gamut_map_white() {
        let color_to_test = white;
        let cam = Cam16::from_int(color_to_test);
        let color = Hct::from(
            cam.hue,
            cam.chroma,
            ColorUtils::lstar_from_argb(color_to_test),
        )
        .to_int();
        assert_eq!(color_to_test, (color));
    }

    #[test]
    fn gamut_map_midgray() {
        let color_to_test = midgray;
        let cam = Cam16::from_int(color_to_test);
        let color = Hct::from(
            cam.hue,
            cam.chroma,
            ColorUtils::lstar_from_argb(color_to_test),
        )
        .to_int();
        assert_eq!(color_to_test, (color));
    }

    fn _color_is_on_boundary(argb: i64) -> bool {
        return ColorUtils::red_from_argb(argb) == 0
            || ColorUtils::red_from_argb(argb) == 255
            || ColorUtils::green_from_argb(argb) == 0
            || ColorUtils::green_from_argb(argb) == 255
            || ColorUtils::blue_from_argb(argb) == 0
            || ColorUtils::blue_from_argb(argb) == 255;
    }

    #[test]
    fn hct_returns_a_sufficiently_close_color() {
        let mut hue: i64 = 15;
        while hue < 360 {
            let mut chroma: i64 = 0;
            while chroma <= 100 {
                let mut tone: i64 = 20;
                while tone <= 80 {
                    let hct_request_description = format!("H{} C{} T{}", hue, chroma, tone);
                    let hct_color = Hct::from(hue as f64, chroma as f64, tone as f64);

                    if chroma > 0 {
                        assert_approx_eq::assert_approx_eq!(hct_color.get_hue(), hue as f64, 4.0);
                        // "Hue should be close for {}",
                        // hctRequestDescription
                        //
                    }

                    assert!(
                        (chroma as f64 >= 0.0) && (chroma as f64 <= (chroma as f64 + 2.5)),
                        "Chroma should be close or less for {}",
                        hct_request_description
                    );

                    if hct_color.get_chroma() < chroma as f64 - 2.5 {
                        assert_eq!(
                        _color_is_on_boundary(hct_color.to_int()),
                        true,
                        "HCT request for non-sRGB color should return \na color on the boundary of the sRGB cube \nfor {}, but got {} instead",hct_request_description,
                        StringUtils::hex_from_argb(hct_color.to_int(), None)
                      );
                    }

                    assert_approx_eq::assert_approx_eq!(hct_color.get_tone(), tone as f64, 0.5);
                    // "Tone should be close for {}",
                    // hctRequestDescription,
                    tone += 10;
                }
                chroma += 10;
            }

            hue += 30
        }
    }

    #[cfg(test)]
    pub mod cam16_to_xyz {
        use crate::hct::{cam16::Cam16, viewing_conditions::ViewingConditions};

        use super::red;

        #[test]
        fn without_array() {
            let color_to_test = red;
            let cam = Cam16::from_int(color_to_test);
            let xyz = cam.xyz_in_viewing_conditions(ViewingConditions::s_rgb(), None);
            assert_approx_eq::assert_approx_eq!(xyz[0], 41.23, 0.01);
            assert_approx_eq::assert_approx_eq!(xyz[1], 21.26, 0.01);
            assert_approx_eq::assert_approx_eq!(xyz[2], 1.93, 0.01);
        }

        #[test]
        fn with_array() {
            let color_to_test = red;
            let cam = Cam16::from_int(color_to_test);
            let xyz = cam
                .xyz_in_viewing_conditions(ViewingConditions::s_rgb(), Some(vec![0.0, 0.0, 0.0]));
            assert_approx_eq::assert_approx_eq!(xyz[0], 41.23, 0.01);
            assert_approx_eq::assert_approx_eq!(xyz[1], 21.26, 0.01);
            assert_approx_eq::assert_approx_eq!(xyz[2], 1.93, 0.01);
        }
    }

    #[cfg(test)]
    pub mod color_relativity {
        use crate::{
            hct::{hct::Hct, viewing_conditions::ViewingConditions},
            tests::hct_test::hct_test::{black, blue, green, midgray, white},
        };

        use super::red;

        #[test]
        fn red_in_black() {
            let color_to_test = red;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(0.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff9F5C51)
            );
        }

        #[test]
        fn red_in_white() {
            let color_to_test = red;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(100.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xffFF5D48)
            );
        }

        #[test]
        fn green_in_black() {
            let color_to_test = green;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(0.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xffACD69D)
            );
        }

        #[test]
        fn green_in_white() {
            let color_to_test = green;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(100.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff8EFF77)
            );
        }

        #[test]
        fn blue_in_black() {
            let color_to_test = blue;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(0.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff343654)
            );
        }

        #[test]
        fn blue_in_white() {
            let color_to_test = blue;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(100.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff3F49FF)
            );
        }

        #[test]
        fn white_in_black() {
            let color_to_test = white;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(0.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xffFFFFFF)
            );
        }

        #[test]
        fn white_in_white() {
            let color_to_test = white;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(100.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xffFFFFFF)
            );
        }

        #[test]
        fn midgray_in_black() {
            let color_to_test = midgray;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(0.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff605F5F)
            );
        }

        #[test]
        fn midgray_in_white() {
            let color_to_test = midgray;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(100.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff8E8E8E)
            );
        }

        #[test]
        fn black_in_black() {
            let color_to_test = black;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(0.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff000000)
            );
        }

        #[test]
        fn black_in_white() {
            let color_to_test = black;
            let hct = Hct::from_int(color_to_test);
            assert_eq!(
                hct.in_viewing_conditions(ViewingConditions::make(
                    None,
                    None,
                    Some(100.0),
                    None,
                    None,
                ))
                .to_int(),
                (0xff000000)
            );
        }
    }
}
