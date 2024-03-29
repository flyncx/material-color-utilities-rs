use crate::quantize::{quantizer::Quantizer, quantizer_wu::QuantizerWu};

const RED: i64 = 0xffff0000;
const GREEN: i64 = 0xff00ff00;
const BLUE: i64 = 0xff0000ff;
// const WHITE: i64 = 0xffffffff;
// const RANDOM: i64 = 0xff426088;
const MAX_COLORS: i64 = 256;

#[test]
fn _1_rando() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[0xff141216].to_vec(), MAX_COLORS, None);
    let colors: Vec<i64> = result.color_to_count.keys().cloned().collect();
    assert_eq!(colors.len(), (1));
    assert_eq!(colors[0], (0xff141216));
}

#[test]
fn _1r() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[RED].to_vec(), MAX_COLORS, None);
    let colors: Vec<i64> = result.color_to_count.keys().cloned().collect();
    assert_eq!(colors.len(), (1));
    assert_eq!(colors[0], (RED));
}
#[test]
fn _1g() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[GREEN].to_vec(), MAX_COLORS, None);
    let colors: Vec<i64> = result.color_to_count.keys().cloned().collect();
    assert_eq!(colors.len(), (1));
    assert_eq!(colors[0], (GREEN));
}

#[test]
fn _1b() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[BLUE].to_vec(), MAX_COLORS, None);
    let colors: Vec<i64> = result.color_to_count.keys().cloned().collect();
    assert_eq!(colors.len(), (1));
    assert_eq!(colors[0], (BLUE));
}

#[test]
fn _5b() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[BLUE, BLUE, BLUE, BLUE, BLUE].to_vec(), MAX_COLORS, None);
    let colors: Vec<i64> = result.color_to_count.keys().cloned().collect();
    assert_eq!(colors.len(), (1));
    assert_eq!(colors[0], (BLUE));
}

#[test]
fn _2r_3g() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[RED, RED, GREEN, GREEN, GREEN].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), (2));
    assert_eq!(true, colors.get(&GREEN).is_some());
    assert_eq!(true, colors.get(&RED).is_some());
}

#[test]
fn _1r_1g_1b() {
    let mut wu = QuantizerWu::new();
    let result = wu.quantize(&[RED, GREEN, BLUE].to_vec(), MAX_COLORS, None);
    let colors = result.color_to_count;
    assert_eq!(colors.len(), (3));
    assert_eq!(true, colors.get(&RED).is_some());
    assert_eq!(true, colors.get(&GREEN).is_some());
    assert_eq!(true, colors.get(&BLUE).is_some());
}
