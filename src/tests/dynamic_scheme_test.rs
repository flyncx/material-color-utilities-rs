
use crate::{dynamiccolor::dynamic_scheme::DynamicScheme, hct::hct::Hct};

#[test]
fn _0_length_input() {
    let hue = DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0), [].to_vec(), [].to_vec());
    assert_approx_eq::assert_approx_eq!(hue, 43.0, 1.0);
}

#[test]
fn _1_length_input_no_rotation() {
    let hue =
        DynamicScheme::get_rotated_hue(Hct::from(43.0, 16.0, 16.0), [0.0].to_vec(), [0.0].to_vec());
    assert_approx_eq::assert_approx_eq!(hue, 43.0, 1.0);
}

#[test]
#[should_panic]
fn input_length_mismatch_asserts() {
    DynamicScheme::get_rotated_hue(
        Hct::from(43.0, 16.0, 16.0),
        [0.0, 1.0].to_vec(),
        [0.0].to_vec(),
    );
}

#[test]
fn on_boundary_rotation_correct() {
    let hue = DynamicScheme::get_rotated_hue(
        Hct::from(43.0, 16.0, 16.0),
        [0.0, 42.0, 360.0].to_vec(),
        [0.0, 15.0, 0.0].to_vec(),
    );
    assert_approx_eq::assert_approx_eq!(hue, 43.0 + 15.0, 1.0);
}

#[test]
fn rotation_result_larger_than_360_degrees_wraps() {
    let hue = DynamicScheme::get_rotated_hue(
        Hct::from(43.0, 16.0, 16.0),
        [0.0, 42.0, 360.0].to_vec(),
        [0.0, 480.0, 0.0].to_vec(),
    );
    assert_approx_eq::assert_approx_eq!(hue, 163.0, 1.0);
}
