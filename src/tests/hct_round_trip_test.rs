
use crate::hct::hct::Hct;

#[test]
#[ignore]
fn hct_preserves_original_color() {
    for argb in 0xFF000000..=0xFFFFFFFF {
        let hct = Hct::from_int(argb);
        let reconstructed_argb =
            Hct::from(hct.get_hue(), hct.get_chroma(), hct.get_tone()).to_int();

        assert_eq!(reconstructed_argb, (argb));
    }
}
