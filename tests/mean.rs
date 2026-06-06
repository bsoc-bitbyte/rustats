use rustats::primary::mean;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}

#[test]
fn mean_empty_returns_none() {
    // Pass an empty slice instead of std::iter::empty
    assert_eq!(mean(&[]), None);
}

#[test]
fn mean_single_value() {
    // Pass a slice with a single element instead of std::iter::once
    assert_eq!(mean(&[5.0]), Some(5.0));
}

#[test]
fn mean_multiple_values() {
    // Arrays can be easily passed as slices by taking a reference
    let values = [1.0, 2.0, 3.0, 4.0];
    let avg = mean(&values).expect("non-empty input should return Some");
    assert!(approx_eq(avg, 2.5, 1e-12));
}

#[test]
fn mean_with_negative_values() {
    let values = [-2.0, 4.0, -6.0, 8.0];
    let avg = mean(&values).expect("non-empty input should return Some");
    assert!(approx_eq(avg, 1.0, 1e-12));
}

#[test]
fn mean_with_decimals() {
    let values = [0.1, 0.2, 0.3];
    // No need to use .iter().copied() anymore, just pass the reference
    let avg = mean(&values).expect("non-empty input should return Some");
    assert!(approx_eq(avg, 0.2, 1e-12));
}

