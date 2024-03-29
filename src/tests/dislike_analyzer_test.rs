use crate::{dislike::dislike_analyzer::DislikeAnalyzer, hct::hct::Hct};

#[test]
fn monk_skin_tone_scale_colors_liked() {
    // From https://skintone.google#/get-started
    let monk_skin_tone_scale_colors = [
        0xfff6ede4, 0xfff3e7db, 0xfff7ead0, 0xffeadaba, 0xffd7bd96, 0xffa07e56, 0xff825c43,
        0xff604134, 0xff3a312a, 0xff292420,
    ];
    for color in monk_skin_tone_scale_colors {
        assert_eq!(DislikeAnalyzer::is_disliked(&Hct::from_int(color)), false);
    }
}

#[test]
fn bile_colors_disliked() {
    let unlikable = [0xff95884B, 0xff716B40, 0xffB08E00, 0xff4C4308, 0xff464521];
    for color in unlikable {
        assert_eq!(
            DislikeAnalyzer::is_disliked(&Hct::from_int(color)),
            true,
            "{} was likable",
            color
        );

        //reason: '');
    }
}

#[test]
fn bile_colors_became_likable() {
    let unlikable = [0xff95884B, 0xff716B40, 0xffB08E00, 0xff4C4308, 0xff464521];
    for color in unlikable {
        let hct = Hct::from_int(color);
        assert_eq!(DislikeAnalyzer::is_disliked(&hct), true);
        let likable = DislikeAnalyzer::fix_if_disliked(&hct);
        assert_eq!(DislikeAnalyzer::is_disliked(&likable), false);
    }
}

#[test]
fn tone_67_not_disliked() {
    let color = Hct::from(100.0, 50.0, 67.0);
    assert_eq!(DislikeAnalyzer::is_disliked(&color), false);
    assert_eq!(
        DislikeAnalyzer::fix_if_disliked(&color).to_int(),
        color.to_int()
    );
}
