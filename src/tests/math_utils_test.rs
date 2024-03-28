
use crate::utils::math_utils::MathUtils;

#[test]
fn rotation_direction_behaves_correctly() {
    let mut from = 0.0;
    while from < 360.0 {
        let mut to = 7.5;
        while to < 360.0 {
            let expected_answer = _rotation_direction(from, to);
            let actual_answer = MathUtils::rotation_direction(from, to);
            assert_eq!(
                expected_answer, actual_answer,
                "should be {} from {} to {}",
                expected_answer, from, to
            );
            assert_eq!(
                actual_answer.abs(),
                1.0,
                "should be either +1.0 or -1.0\nfrom {} to $to (got {})",
                from,
                actual_answer
            );
            to += 15.0;
        }
        from += 15.0;
    }
}

// Original implementation for MathUtils.rotationDirection.
// Included here to test equivalence with new implementation.
fn _rotation_direction(from: f64, to: f64) -> f64 {
    let a = to - from;
    let b = to - from + 360.0;
    let c = to - from - 360.0;
    let a_abs = a.abs();
    let b_abs = b.abs();
    let c_abs = c.abs();
    if a_abs <= b_abs && a_abs <= c_abs {
        return if a >= 0.0 { 1.0 } else { -1.0 };
    } else if b_abs <= a_abs && b_abs <= c_abs {
        return if b >= 0.0 { 1.0 } else { -1.0 };
    } else {
        return if c >= 0.0 { 1.0 } else { -1.0 };
    }
}
