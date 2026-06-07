use rustats::primary::mode;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}

#[test]
fn mode_empty_returns_none() {
    assert_eq!(mode(&[]), None);
}

#[test]
fn mode_single_value() {
    assert_eq!(mode(&[4.0]), Some(4.0));
}

#[test]
fn mode_clear_winner() {
    let values = [1.0, 2.0, 2.0, 3.0, 2.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 2.0, 1e-12));
}

#[test]
fn mode_tie_returns_smallest() {
    let values = [1.0, 1.0, 2.0, 2.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 1.0, 1e-12));
}

#[test]
fn mode_three_way_tie_returns_smallest() {
    let values = [3.0, 1.0, 2.0, 3.0, 1.0, 2.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 1.0, 1e-12));
}

#[test]
fn mode_all_same_value() {
    let values = [5.0, 5.0, 5.0, 5.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 5.0, 1e-12));
}

#[test]
fn mode_all_unique_values_returns_smallest() {
    let values = [3.0, 1.0, 4.0, 2.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 1.0, 1e-12));
}

#[test]
fn mode_with_negative_values() {
    let values = [-1.0, -1.0, 2.0, 3.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, -1.0, 1e-12));
}

#[test]
fn mode_tie_with_negative_returns_smallest() {
    let values = [-2.0, -2.0, 1.0, 1.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, -2.0, 1e-12));
}

#[test]
fn mode_with_decimals() {
    let values = [0.5, 0.1, 0.5, 0.9, 0.5];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 0.5, 1e-12));
}

#[test]
fn mode_two_elements_same_value() {
    let values = [3.0, 3.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 3.0, 1e-12));
}

#[test]
fn mode_two_elements_different_values_returns_smallest() {
    let values = [7.0, 2.0];
    let m = mode(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 2.0, 1e-12));
}
