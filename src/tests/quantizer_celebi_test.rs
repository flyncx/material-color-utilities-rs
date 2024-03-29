use crate::quantize::{quantizer::Quantizer, quantizer_celebi::QuantizerCelebi};

const RED: i64 = 0xffff0000;
const GREEN: i64 = 0xff00ff00;
const BLUE: i64 = 0xff0000ff;
// const WHITE: i64 = 0xffffffff;
// const RANDOM: i64 = 0xff426088;
const MAX_COLORS: i64 = 256;

#[test]
fn _1_r() {
    let celebi = &mut QuantizerCelebi {};
    let result = celebi.quantize(&[RED].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&RED).is_some());
}

#[test]
fn _1_g() {
    let celebi = &mut QuantizerCelebi {};
    let result = celebi.quantize(&[GREEN].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&GREEN).is_some());
}

#[test]
fn _1_b() {
    let celebi = &mut QuantizerCelebi {};
    let result = celebi.quantize(&[BLUE].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&BLUE).is_some());
}

#[test]
fn _5_b() {
    let celebi = &mut QuantizerCelebi {};
    let result = celebi.quantize(&[BLUE, BLUE, BLUE, BLUE, BLUE].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 1);
    assert_eq!(true, colors.get(&BLUE).is_some());
}

#[test]
fn _1_r_1_g_1_b() {
    let celebi = &mut QuantizerCelebi {};
    let result = celebi.quantize(&[RED, GREEN, BLUE].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 3);
    assert_eq!(true, colors.get(&RED).is_some());
    assert_eq!(true, colors.get(&GREEN).is_some());
    assert_eq!(true, colors.get(&BLUE).is_some());
}

#[test]
fn _2_r_3_g() {
    let celebi = &mut QuantizerCelebi {};
    let result = celebi.quantize(&[RED, RED, GREEN, GREEN, GREEN].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), 2);
    assert_eq!(true, colors.get(&RED).is_some());
    assert_eq!(true, colors.get(&GREEN).is_some());
}
