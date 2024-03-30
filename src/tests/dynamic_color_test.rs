#[cfg(test)]
pub mod dynamic_color_test {
    use std::collections::HashMap;

    use crate::{
        contrast::contrast::Contrast,
        dynamiccolor::{
            dynamic_color::DynamicColor, material_dynamic_colors::MaterialDynamicColors,
        },
        hct::hct::Hct,
        scheme::{
            scheme_content::SchemeContent, scheme_fidelity::SchemeFidelity,
            scheme_monochrome::SchemeMonochrome, scheme_tonal_spot::SchemeTonalSpot,
        },
    };

    fn _seed_colors() -> [Hct; 4] {
        [
            Hct::from_int(0xFFFF0000),
            Hct::from_int(0xFFFFFF00),
            Hct::from_int(0xFF00FF00),
            Hct::from_int(0xFF0000FF),
        ]
    }

    const CONTRAST_LEVELS: [f64; 5] = [-1.0, -0.5, 0.0, 0.5, 1.0];

    struct _Pair {
        pub fg_name: String,
        pub bg_name: String,
    }
    impl _Pair {
        //_Pair(this.fgName, this.bgName);
        pub fn _new(fg_name: String, bg_name: String) -> Self {
            Self { fg_name, bg_name }
        }
    }

    fn _colors() -> HashMap<String, DynamicColor> {
        let mut s: HashMap<String, DynamicColor> = HashMap::new();
        s.insert(
            "background".to_string(),
            MaterialDynamicColors::background(),
        );
        s.insert(
            "on_background".to_string(),
            MaterialDynamicColors::on_background(),
        );
        s.insert("surface".to_string(), MaterialDynamicColors::surface());
        s.insert(
            "surface_dim".to_string(),
            MaterialDynamicColors::surface_dim(),
        );
        s.insert(
            "surface_bright".to_string(),
            MaterialDynamicColors::surface_bright(),
        );
        s.insert(
            "surface_container_lowest".to_string(),
            MaterialDynamicColors::surface_container_lowest(),
        );
        s.insert(
            "surface_container_low".to_string(),
            MaterialDynamicColors::surface_container_low(),
        );
        s.insert(
            "surface_container".to_string(),
            MaterialDynamicColors::surface_container(),
        );
        s.insert(
            "surface_container_high".to_string(),
            MaterialDynamicColors::surface_container_high(),
        );
        s.insert(
            "surface_container_highest".to_string(),
            MaterialDynamicColors::surface_container_highest(),
        );
        s.insert(
            "on_surface".to_string(),
            MaterialDynamicColors::on_surface(),
        );
        s.insert(
            "surface_variant".to_string(),
            MaterialDynamicColors::surface_variant(),
        );
        s.insert(
            "on_surface_variant".to_string(),
            MaterialDynamicColors::on_surface_variant(),
        );
        s.insert(
            "inverse_surface".to_string(),
            MaterialDynamicColors::inverse_surface(),
        );
        s.insert(
            "inverse_on_surface".to_string(),
            MaterialDynamicColors::inverse_on_surface(),
        );
        s.insert("outline".to_string(), MaterialDynamicColors::outline());
        s.insert(
            "outline_variant".to_string(),
            MaterialDynamicColors::outline_variant(),
        );
        s.insert("shadow".to_string(), MaterialDynamicColors::shadow());
        s.insert("scrim".to_string(), MaterialDynamicColors::scrim());
        s.insert(
            "surface_tint".to_string(),
            MaterialDynamicColors::surface_tint(),
        );
        s.insert("primary".to_string(), MaterialDynamicColors::primary());
        s.insert(
            "on_primary".to_string(),
            MaterialDynamicColors::on_primary(),
        );
        s.insert(
            "primary_container".to_string(),
            MaterialDynamicColors::primary_container(),
        );
        s.insert(
            "on_primary_container".to_string(),
            MaterialDynamicColors::on_primary_container(),
        );
        s.insert(
            "inverse_primary".to_string(),
            MaterialDynamicColors::inverse_primary(),
        );
        s.insert("secondary".to_string(), MaterialDynamicColors::secondary());
        s.insert(
            "on_secondary".to_string(),
            MaterialDynamicColors::on_secondary(),
        );
        s.insert(
            "secondary_container".to_string(),
            MaterialDynamicColors::secondary_container(),
        );
        s.insert(
            "on_secondary_container".to_string(),
            MaterialDynamicColors::on_secondary_container(),
        );
        s.insert("tertiary".to_string(), MaterialDynamicColors::tertiary());
        s.insert(
            "on_tertiary".to_string(),
            MaterialDynamicColors::on_tertiary(),
        );
        s.insert(
            "tertiary_container".to_string(),
            MaterialDynamicColors::tertiary_container(),
        );
        s.insert(
            "on_tertiary_container".to_string(),
            MaterialDynamicColors::on_tertiary_container(),
        );
        s.insert("error".to_string(), MaterialDynamicColors::error());
        s.insert("on_error".to_string(), MaterialDynamicColors::on_error());
        s.insert(
            "error_container".to_string(),
            MaterialDynamicColors::error_container(),
        );
        s.insert(
            "on_error_container".to_string(),
            MaterialDynamicColors::on_error_container(),
        );
        s
    }

    fn _text_surface_pairs() -> [_Pair; 12] {
        [
            _Pair::_new("on_primary".to_string(), "primary".to_string()),
            _Pair::_new(
                "on_primary_container".to_string(),
                "primary_container".to_string(),
            ),
            _Pair::_new("on_secondary".to_string(), "secondary".to_string()),
            _Pair::_new(
                "on_secondary_container".to_string(),
                "secondary_container".to_string(),
            ),
            _Pair::_new("on_tertiary".to_string(), "tertiary".to_string()),
            _Pair::_new(
                "on_tertiary_container".to_string(),
                "tertiary_container".to_string(),
            ),
            _Pair::_new("on_error".to_string(), "error".to_string()),
            _Pair::_new(
                "on_error_container".to_string(),
                "error_container".to_string(),
            ),
            _Pair::_new("on_background".to_string(), "background".to_string()),
            _Pair::_new(
                "on_surface_variant".to_string(),
                "surface_bright".to_string(),
            ),
            _Pair::_new("on_surface_variant".to_string(), "surface_dim".to_string()),
            _Pair::_new(
                "inverse_on_surface".to_string(),
                "inverse_surface".to_string(),
            ),
        ]
    }

    #[test]
    fn values_are_correct() {
        assert_eq!(
            MaterialDynamicColors::on_primary_container().get_argb(
                &SchemeFidelity::new(Hct::from_int(0xFFFF0000), false, 0.5,).dynamic_scheme
            ),
            (0xFFFFFFFF),
        );
        assert_eq!(
            MaterialDynamicColors::on_secondary_container().get_argb(
                &SchemeContent::new(Hct::from_int(0xFF0000FF), false, 0.5,).dynamic_scheme
            ),
            (0xFFFFFFFF),
        );
        assert_eq!(
            MaterialDynamicColors::on_tertiary_container().get_argb(
                &SchemeContent::new(Hct::from_int(0xFFFFFF00), true, -0.5,).dynamic_scheme
            ),
            (0xffbac040),
        );
        assert_eq!(
            MaterialDynamicColors::inverse_surface().get_argb(
                &SchemeContent::new(Hct::from_int(0xFF0000FF), false, 0.0).dynamic_scheme
            ),
            (0xFF2F2F3B),
        );
        assert_eq!(
            MaterialDynamicColors::inverse_primary().get_argb(
                &SchemeContent::new(Hct::from_int(0xFFFF0000), false, -0.5).dynamic_scheme
            ),
            (0xffff422f),
        );
        assert_eq!(
            MaterialDynamicColors::outline_variant()
                .get_argb(&SchemeContent::new(Hct::from_int(0xFFFFFF00), true, 0.0).dynamic_scheme),
            (0xFF484831),
        );
    }

    #[test]
    fn parametic_testing() {
        // Parametric test, ensuring that dynamic schemes respect contrast
        // between text-surface pairs.
        for color in _seed_colors() {
            for contrast_level in CONTRAST_LEVELS {
                for is_dark in [false, true] {
                    for scheme in [
                        SchemeContent::new(color.clone(), is_dark, contrast_level).dynamic_scheme,
                        SchemeMonochrome::new(color.clone(), is_dark, contrast_level)
                            .dynamic_scheme,
                        SchemeTonalSpot::new(color.clone(), is_dark, contrast_level).dynamic_scheme,
                        SchemeFidelity::new(color.clone(), is_dark, contrast_level).dynamic_scheme,
                    ] {
                        for pair in _text_surface_pairs() {
                            // Expect that each text-surface pair has a
                            // minimum contrast of 4.5 (unreduced contrast), or 3.0
                            // (reduced contrast).
                            let fg_name = pair.fg_name;
                            let bg_name = pair.bg_name;
                            let mut _colors: &mut HashMap<String, DynamicColor> = &mut _colors();
                            let foreground_tone = _colors
                                .get_mut(&fg_name)
                                .unwrap()
                                .get_hct(&scheme)
                                .get_tone();
                            let background_tone = _colors
                                .get_mut(&bg_name)
                                .unwrap()
                                .get_hct(&scheme)
                                .get_tone();
                            let contrast =
                                Contrast::ratio_of_tones(foreground_tone, background_tone);

                            let minimum_requirement = if contrast_level >= 0.0 { 4.5 } else { 3.0 };

                            assert!(
                                contrast >= minimum_requirement,
                                r#"
                                'Scheme: ${}; Seed color: ${}; '
                           'Contrast level: ${}; Dark: ${}',
                            reason: 'Contrast ${}is too low between '
                                  'foreground (${}; ${}) and '
                                  'background (${}; ${})',
                            "#,
                                scheme.source_color_hct.to_string(),
                                color.to_string(),
                                contrast_level,
                                is_dark,
                                contrast,
                                fg_name,
                                foreground_tone.to_string(),
                                bg_name,
                                background_tone.to_string()
                            )
                        }
                    }
                }
            }
        }
    }

    // Tests for fixed colors.
    #[test]
    fn fixed_colors_in_non_monochrome_schemes() {
        let scheme = SchemeTonalSpot::new(Hct::from_int(0xFFFF0000), true, 0.0).dynamic_scheme;

        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::primary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::primary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            80.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_primary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_primary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::secondary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::secondary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            80.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_secondary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_secondary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::tertiary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::tertiary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            80.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_tertiary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_tertiary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
    }

    #[test]
    fn fixed_colors_in_light_monochrome_schemes() {
        let scheme = SchemeMonochrome::new(Hct::from_int(0xFFFF0000), false, 0.0).dynamic_scheme;

        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::primary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            40.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::primary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_primary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_primary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::secondary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            80.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::secondary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            70.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_secondary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_secondary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            25.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::tertiary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            40.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::tertiary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_tertiary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_tertiary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
    }

    #[test]
    fn fixed_colors_in_dark_monochrome_schemes() {
        let scheme = SchemeMonochrome::new(Hct::from_int(0xFFFF0000), true, 0.0).dynamic_scheme;

        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::primary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            40.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::primary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_primary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_primary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::secondary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            80.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::secondary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            70.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_secondary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            10.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_secondary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            25.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::tertiary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            40.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::tertiary_fixed_dim()
                .get_hct(&scheme)
                .get_tone(),
            30.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_tertiary_fixed()
                .get_hct(&scheme)
                .get_tone(),
            100.0,
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            MaterialDynamicColors::on_tertiary_fixed_variant()
                .get_hct(&scheme)
                .get_tone(),
            90.0,
            1.0
        );
    }
}
