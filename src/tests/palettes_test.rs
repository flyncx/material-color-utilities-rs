pub mod tonal_palette {
    #[cfg(test)]
    pub mod of_and_from_list_constructors {
        use crate::palettes::tonal_palette::TonalPalette;

        // Regression test for https://github.com/material-foundation/material-color-utilities/issues/56
        #[test]
        fn operator_eq() {
            let a1 = TonalPalette::of(1.0, 1.0);
            let a2 = TonalPalette::of(1.0, 1.0);
            let b1 =
                TonalPalette::from_list(TonalPalette::COMMON_TONES.map(|_| 0xDEADBEEF).to_vec());
            let b2 =
                TonalPalette::from_list(TonalPalette::COMMON_TONES.map(|_| 0xDEADBEEF).to_vec());
            assert_eq!(a1 == b1, false);
            assert_eq!(b1 == a1, false);
            assert_eq!(a1 != b1, true);
            assert_eq!(b1 != a1, true);
            assert_eq!(a1 == a2, true);
            assert_eq!(b1 == b2, true);

            let mut c1 = TonalPalette::from_list(TonalPalette::COMMON_TONES.map(|_| 123).to_vec());

            let mut c2 = TonalPalette::from_list(
                TonalPalette::COMMON_TONES
                    .map(|e| if e < 15 { 456 } else { 123 })
                    .to_vec(),
            );

            assert_eq!(c1.get(50), c2.get(50));
            assert_eq!(c1 == c2, false);
        }
    }

    pub mod of_constructors {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        use crate::{hct::hct::Hct, palettes::tonal_palette::TonalPalette};
        #[test]
        fn tones_of_blue() {
            let hct = Hct::from_int(0xff0000ff);
            let mut tones = TonalPalette::of(hct.get_hue(), hct.get_chroma());

            assert_eq!(tones.get(0), 0xff000000);
            assert_eq!(tones.get(10), 0xff00006e);
            assert_eq!(tones.get(20), 0xff0001ac);
            assert_eq!(tones.get(30), 0xff0000ef);
            assert_eq!(tones.get(40), 0xff343dff);
            assert_eq!(tones.get(50), 0xff5a64ff);
            assert_eq!(tones.get(60), 0xff7c84ff);
            assert_eq!(tones.get(70), 0xff9da3ff);
            assert_eq!(tones.get(80), 0xffbec2ff);
            assert_eq!(tones.get(90), 0xffe0e0ff);
            assert_eq!(tones.get(95), 0xfff1efff);
            assert_eq!(tones.get(99), 0xfffffbff);
            assert_eq!(tones.get(100), 0xffffffff);

            // Tone not in [TonalPalette.commonTones]
            assert_eq!(tones.get(3), 0xff00003c);
        }

        #[test]
        fn as_list() {
            let hct = Hct::from_int(0xff0000ff);
            let mut tones = TonalPalette::of(hct.get_hue(), hct.get_chroma());

            assert_eq!(
                tones.get_as_list(),
                [
                    0xff000000, 0xff00006e, 0xff0001ac, 0xff0000ef, 0xff343dff, 0xff5a64ff,
                    0xff7c84ff, 0xff9da3ff, 0xffbec2ff, 0xffe0e0ff, 0xfff1efff, 0xfffffbff,
                    0xffffffff,
                ],
            );
        }

        #[test]
        fn operator_eq_and_hash_code() {
            let hct_ab = Hct::from_int(0xff0000ff);
            let tones_a = TonalPalette::of(hct_ab.get_hue(), hct_ab.get_chroma());
            let tones_b = TonalPalette::of(hct_ab.get_hue(), hct_ab.get_chroma());
            let hct_c = Hct::from_int(0xff123456);
            let tones_c = TonalPalette::of(hct_c.get_hue(), hct_c.get_chroma());

            assert_eq!(tones_a, tones_b);
            assert!(tones_b != (tones_c));
            let mut a_hash_code = DefaultHasher::new();
            let mut b_hash_code = DefaultHasher::new();
            let mut c_hash_code = DefaultHasher::new();

            tones_a.hash(&mut a_hash_code);
            tones_b.hash(&mut b_hash_code);
            tones_c.hash(&mut c_hash_code);

            assert_eq!(a_hash_code.finish(), b_hash_code.finish());
            assert!(b_hash_code.finish() != (c_hash_code.finish()));
        }
    }

    #[cfg(test)]
    pub mod from_list_constructor {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        use crate::{hct::hct::Hct, palettes::tonal_palette::TonalPalette};

        fn hue_chroma_palette() -> TonalPalette {
            TonalPalette::of(270.0, 36.0)
        }
        fn cached_palette() -> Vec<i64> {
            TonalPalette::COMMON_TONES
                .map(|t| hue_chroma_palette().get(t))
                .to_vec()
        }
        fn broken_palette() -> [i64; 13] {
            [
                cached_palette()[0],
                cached_palette()[1],
                Hct::from(180.0, 24.0, 20.0).to_int(),
                cached_palette()[3],
                cached_palette()[4],
                cached_palette()[5],
                cached_palette()[6],
                cached_palette()[7],
                cached_palette()[8],
                Hct::from(0.0, 12.0, 90.0).to_int(),
                cached_palette()[10],
                cached_palette()[11],
                cached_palette()[12],
            ]
        }
        fn rebuilt_palette() -> TonalPalette {
            TonalPalette::from_list(broken_palette().to_vec())
        }

        #[test]
        fn correctly_deduces_original_hue_and_chroma() {
            assert_approx_eq::assert_approx_eq!(rebuilt_palette().hue, 270.0, 1.0);
            assert_approx_eq::assert_approx_eq!(rebuilt_palette().chroma, 36.0, 1.0);
        }

        #[test]
        fn low_chroma_noise_does_not_affect_the_hue_and_chroma_deduced() {
            let rebuilt_clean_palette = TonalPalette::from_list(cached_palette());
            assert_eq!(rebuilt_palette().hue, rebuilt_clean_palette.hue);
            assert_eq!(rebuilt_palette().chroma, rebuilt_clean_palette.chroma);
        }

        #[test]
        fn returns_cached_colors_when_possible() {
            assert_eq!(rebuilt_palette().get(20), (broken_palette()[2]));
            assert_eq!(rebuilt_palette().get(50), (broken_palette()[5]));
            assert_eq!(rebuilt_palette().get(90), (broken_palette()[9]));
            assert_eq!(rebuilt_palette().get(99), (broken_palette()[11]));
        }

        #[test]
        #[ignore = ""]
        fn correctly_deduces_colors_at_other_tones() {
            assert_eq!(rebuilt_palette().get(15), (hue_chroma_palette().get(15)),);
            assert_eq!(rebuilt_palette().get(53), (hue_chroma_palette().get(53)),);
            assert_eq!(rebuilt_palette().get(78), (hue_chroma_palette().get(78)),);
        }

        #[test]
        fn deduced_colors_have_correct_tone() {
            assert_approx_eq::assert_approx_eq!(
                rebuilt_palette().get_hct(15.0).get_tone(),
                15.0,
                1.0
            );
            assert_approx_eq::assert_approx_eq!(
                rebuilt_palette().get_hct(53.0).get_tone(),
                53.0,
                1.0
            );
            assert_approx_eq::assert_approx_eq!(
                rebuilt_palette().get_hct(78.0).get_tone(),
                78.0,
                1.0
            );
        }

        #[test]
        fn as_list() {
            let ints: Vec<i64> = (0..TonalPalette::COMMON_SIZE).map(|i| i as i64).collect();
            let mut tones = TonalPalette::from_list(ints.clone());
            assert_eq!(tones.get_as_list(), ints);
        }

        #[test]
        fn operator_eq_and_hash_code() {
            // This test confirms that `==` and `hashCode` behave the way they are
            // expected to behave. By defining five palettes, three from hue and
            // chroma, and two from lists, we expect their `hashCode` to be
            // distinct, and that their equality should satisfy the following grid:
            //
            // ==? 1   2   3   4   5
            // 1   YES -   -   YES -
            // 2   -   YES -   -   -
            // 3   -   -   YES -   -
            // 4   YES -   -   YES -
            // 5   -   -   -   -   YES

            let mut palette1 = TonalPalette::of(270.0, 36.0);
            let palette2 = TonalPalette::of(180.0, 36.0);
            let palette3 = TonalPalette::of(270.0, 12.0);

            let palette4 = TonalPalette::from_list(palette1.get_as_list());
            let mut broken_list = palette1.clone().get_as_list();
            broken_list[2] = Hct::from(180.0, 24.0, 20.0).to_int();
            broken_list[9] = Hct::from(0.0, 12.0, 90.0).to_int();
            let palette5 = TonalPalette::from_list(broken_list);

            assert_eq!(palette1, palette1);
            assert!(palette1 != (palette2));
            assert!(palette1 != (palette3));
            assert_eq!(palette1, palette4);
            assert!(palette1 != (palette5));

            assert!(palette2 != (palette1));
            assert_eq!(palette2, palette2);
            assert!(palette2 != (palette3));
            assert!(palette2 != (palette4));
            assert!(palette2 != (palette5));

            assert!(palette3 != (palette1));
            assert!(palette3 != (palette2));
            assert_eq!(palette3, palette3);
            assert!(palette3 != (palette4));
            assert!(palette3 != (palette5));

            assert_eq!(palette4, palette1);
            assert!(palette4 != (palette2));
            assert!(palette4 != (palette3));
            assert_eq!(palette4, palette4);
            assert!(palette4 != (palette5));

            assert!(palette5 != (palette1));
            assert!(palette5 != (palette2));
            assert!(palette5 != (palette3));
            assert!(palette5 != (palette4));
            assert_eq!(palette5, palette5);

            // They should have five distinct hash codes (ignoring hash collision).
            let hash_codes = [palette1, palette2, palette3, palette4, palette5]
                .map(|x| {
                    let mut hasher = DefaultHasher::new();
                    x.hash(&mut hasher);
                    hasher.finish()
                })
                .to_vec();
            assert_eq!(hash_codes.len(), (5));
        }
    }
}

#[cfg(test)]
pub mod core_palette {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use crate::palettes::{core_palette::CorePalette, tonal_palette::TonalPalette};

    #[test]
    fn as_list() {
        let ints: Vec<i64> = (0..(CorePalette::SIZE * TonalPalette::COMMON_SIZE))
            .map(|i| i as i64)
            .collect();
        let mut core_palette = CorePalette::from_list(ints.clone());
        assert_eq!(core_palette.as_list(), ints);
    }

    #[test]
    fn hash_code() {
        let core_palette_a = CorePalette::of(0xff0000ff);
        let core_palette_b = CorePalette::of(0xff0000ff);
        let core_palette_c = CorePalette::of(0xff123456);

        assert_eq!(core_palette_a, core_palette_b);
        assert!(core_palette_b != core_palette_c);
        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        core_palette_a.hash(&mut hasher_a);
        core_palette_b.hash(&mut hasher_b);
        let mut hasher_c = DefaultHasher::new();
        core_palette_c.hash(&mut hasher_c);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
        assert!(hasher_b.finish() != hasher_c.finish());
    }
    #[test]
    fn of_blue() {
        let mut core = CorePalette::of(0xff0000ff);
        assert_eq!(core.primary.get(100), 0xffffffff);
        assert_eq!(core.primary.get(95), 0xfff1efff);
        assert_eq!(core.primary.get(90), 0xffe0e0ff);
        assert_eq!(core.primary.get(80), 0xffbec2ff);
        assert_eq!(core.primary.get(70), 0xff9da3ff);
        assert_eq!(core.primary.get(60), 0xff7c84ff);
        assert_eq!(core.primary.get(50), 0xff5a64ff);
        assert_eq!(core.primary.get(40), 0xff343dff);
        assert_eq!(core.primary.get(30), 0xff0000ef);
        assert_eq!(core.primary.get(20), 0xff0001ac);
        assert_eq!(core.primary.get(10), 0xff00006e);
        assert_eq!(core.primary.get(0), 0xff000000);
        assert_eq!(core.secondary.get(100), 0xffffffff);
        assert_eq!(core.secondary.get(95), 0xfff1efff);
        assert_eq!(core.secondary.get(90), 0xffe1e0f9);
        assert_eq!(core.secondary.get(80), 0xffc5c4dd);
        assert_eq!(core.secondary.get(70), 0xffa9a9c1);
        assert_eq!(core.secondary.get(60), 0xff8f8fa6);
        assert_eq!(core.secondary.get(50), 0xff75758b);
        assert_eq!(core.secondary.get(40), 0xff5c5d72);
        assert_eq!(core.secondary.get(30), 0xff444559);
        assert_eq!(core.secondary.get(20), 0xff2e2f42);
        assert_eq!(core.secondary.get(10), 0xff191a2c);
        assert_eq!(core.secondary.get(0), 0xff000000);
    }

    #[test]
    fn content_of_blue() {
        let mut core = CorePalette::content_of(0xff0000ff);
        assert_eq!(core.primary.get(100), 0xffffffff);
        assert_eq!(core.primary.get(95), 0xfff1efff);
        assert_eq!(core.primary.get(90), 0xffe0e0ff);
        assert_eq!(core.primary.get(80), 0xffbec2ff);
        assert_eq!(core.primary.get(70), 0xff9da3ff);
        assert_eq!(core.primary.get(60), 0xff7c84ff);
        assert_eq!(core.primary.get(50), 0xff5a64ff);
        assert_eq!(core.primary.get(40), 0xff343dff);
        assert_eq!(core.primary.get(30), 0xff0000ef);
        assert_eq!(core.primary.get(20), 0xff0001ac);
        assert_eq!(core.primary.get(10), 0xff00006e);
        assert_eq!(core.primary.get(0), 0xff000000);
        assert_eq!(core.secondary.get(100), 0xffffffff);
        assert_eq!(core.secondary.get(95), 0xfff1efff);
        assert_eq!(core.secondary.get(90), 0xffe0e0ff);
        assert_eq!(core.secondary.get(80), 0xffc1c3f4);
        assert_eq!(core.secondary.get(70), 0xffa5a7d7);
        assert_eq!(core.secondary.get(60), 0xff8b8dbb);
        assert_eq!(core.secondary.get(50), 0xff7173a0);
        assert_eq!(core.secondary.get(40), 0xff585b86);
        assert_eq!(core.secondary.get(30), 0xff40436d);
        assert_eq!(core.secondary.get(20), 0xff2a2d55);
        assert_eq!(core.secondary.get(10), 0xff14173f);
        assert_eq!(core.secondary.get(0), 0xff000000);
    }
}
