use crate::quantize::quantizer_wsmeans::QuantizerWsmeans;

const RED: i64 = 0xffff0000;
const GREEN: i64 = 0xff00ff00;
const BLUE: i64 = 0xff0000ff;
// const WHITE: i64 = 0xffffffff;
// const RANDOM: i64 = 0xff426088;
const MAX_COLORS: i64 = 256;

#[test]
fn _1_rando() {
    let result =
        QuantizerWsmeans::quantize(&[0xff141216].to_vec(), MAX_COLORS, None, None, None, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&0xff141216).is_some());
}

#[test]
fn _1_r() {
    let result = QuantizerWsmeans::quantize(&[RED].to_vec(), MAX_COLORS, None, None, None, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&RED).is_some());
}

#[test]
fn _1_g() {
    let result = QuantizerWsmeans::quantize(&[GREEN].to_vec(), MAX_COLORS, None, None, None, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&GREEN).is_some());
}

#[test]
fn _1_b() {
    let result = QuantizerWsmeans::quantize(&[BLUE].to_vec(), MAX_COLORS, None, None, None, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&BLUE).is_some());
}

#[test]
fn _5_b() {
    let result = QuantizerWsmeans::quantize(
        &[BLUE, BLUE, BLUE, BLUE, BLUE].to_vec(),
        MAX_COLORS,
        None,
        None,
        None,
        None,
    );
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&BLUE).is_some());
}
