use rustats::primary::variance;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}

#[test]
fn variance_empty_returns_none() {
    assert_eq!(variance(&[]), None);
}

#[test]
fn variance_single_value_returns_none() {
    // Sample variance requires at least 2 values (denominator is n-1)
    assert_eq!(variance(&[5.0]), None);
}

#[test]
fn variance_two_identical_values() {
    let values = [4.0, 4.0];
    let v = variance(&values).expect("two or more values should return Some");
    assert!(approx_eq(v, 0.0, 1e-12));
}

#[test]
fn variance_two_values() {
    // Mean = 3.0, deviations = [-1, 1], squared = [1, 1], sum = 2, / (2-1) = 2.0
    let values = [2.0, 4.0];
    let v = variance(&values).expect("two or more values should return Some");
    assert!(approx_eq(v, 2.0, 1e-12));
}

#[test]
fn variance_multiple_values() {
    // Mean = 3.0, deviations = [-2,-1,0,1,2], squared = [4,1,0,1,4], sum = 10, / (5-1) = 2.5
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];
    let v = variance(&values).expect("non-empty input should return Some");
    assert!(approx_eq(v, 2.5, 1e-12));
}

#[test]
fn variance_with_negative_values() {
    // Mean = 0.0, deviations = [-2, -1, 1, 2], squared = [4, 1, 1, 4], sum = 10, / 3 ≈ 3.333...
    let values = [-2.0, -1.0, 1.0, 2.0];
    let v = variance(&values).expect("non-empty input should return Some");
    assert!(approx_eq(v, 10.0 / 3.0, 1e-12));
}

#[test]
fn variance_with_decimals() {
    // Mean = 2.0, deviations = [-1.5, -0.5, 0.5, 1.5], squared = [2.25, 0.25, 0.25, 2.25]
    // sum = 5.0, / (4-1) ≈ 1.6667
    let values = [0.5, 1.5, 2.5, 3.5];
    let v = variance(&values).expect("non-empty input should return Some");
    assert!(approx_eq(v, 5.0 / 3.0, 1e-12));
}

#[test]
fn variance_all_same_value() {
    let values = [7.0, 7.0, 7.0, 7.0];
    let v = variance(&values).expect("non-empty input should return Some");
    assert!(approx_eq(v, 0.0, 1e-12));
}

#[test]
fn variance_is_non_negative() {
    // Variance can never be negative, regardless of the input values
    let values = [-10.0, 0.0, 10.0, 5.0, -5.0];
    let v = variance(&values).expect("non-empty input should return Some");
    assert!(v >= 0.0);
}

#[test]
fn variance_known_result() {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let v = variance(&values).expect("non-empty input should return Some");
    assert!(approx_eq(v, 32.0 / 7.0, 1e-12));
}
