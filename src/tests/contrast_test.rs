
use crate::contrast::contrast::Contrast;

#[test]
fn ratio_of_tones_out_of_bounds_input() {
    assert_approx_eq::assert_approx_eq!(21.0, Contrast::ratio_of_tones(-10.0, 110.0), 0.001);
}

#[test]
fn lighter_impossible_ratio_errors() {
    assert_approx_eq::assert_approx_eq!(-1.0, Contrast::lighter(90.0, 10.0), 0.001);
}

#[test]
fn lighter_out_of_bounds_input_above_errors() {
    assert_approx_eq::assert_approx_eq!(-1.0, Contrast::lighter(110.0, 2.0), 0.001);
}

#[test]
fn lighter_out_of_bounds_input_below_errors() {
    assert_approx_eq::assert_approx_eq!(-1.0, Contrast::lighter(-10.0, 2.0), 0.001);
}

#[test]
fn lighter_unsafe_returns_max_tone() {
    assert_approx_eq::assert_approx_eq!(100.0, Contrast::lighter_unsafe(100.0, 2.0), 0.001);
}

#[test]
fn darker_impossible_ratio_errors() {
    assert_approx_eq::assert_approx_eq!(-1.0, Contrast::darker(10.0, 20.0), 0.001);
}

#[test]
fn darker_out_of_bounds_input_above_errors() {
    assert_approx_eq::assert_approx_eq!(-1.0, Contrast::darker(110.0, 2.0), 0.001);
}

#[test]
fn darker_out_of_bounds_input_below_errors() {
    assert_approx_eq::assert_approx_eq!(-1.0, Contrast::darker(-10.0, 2.0), 0.001);
}

#[test]
fn darker_unsafe_returns_min_tone() {
    assert_approx_eq::assert_approx_eq!(0.0, Contrast::darker_unsafe(0.0, 2.0), 0.001);
}
