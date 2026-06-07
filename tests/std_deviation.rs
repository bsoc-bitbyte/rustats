use rustats::primary::std_deviation;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}

#[test]
fn std_deviation_empty_returns_none() {
    assert_eq!(std_deviation(&[]), None);
}

#[test]
fn std_deviation_single_value_returns_none() {
    assert_eq!(std_deviation(&[5.0]), None);
}

#[test]
fn std_deviation_two_identical_values() {
    let values = [3.0, 3.0];
    let sd = std_deviation(&values).expect("two or more values should return Some");
    assert!(approx_eq(sd, 0.0, 1e-12));
}

#[test]
fn std_deviation_two_values() {
    let values = [2.0, 4.0];
    let sd = std_deviation(&values).expect("two or more values should return Some");
    assert!(approx_eq(sd, 2.0_f64.sqrt(), 1e-12));
}

#[test]
fn std_deviation_multiple_values() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];
    let sd = std_deviation(&values).expect("non-empty input should return Some");
    assert!(approx_eq(sd, 2.5_f64.sqrt(), 1e-12));
}

#[test]
fn std_deviation_all_same_value() {
    let values = [9.0, 9.0, 9.0, 9.0];
    let sd = std_deviation(&values).expect("non-empty input should return Some");
    assert!(approx_eq(sd, 0.0, 1e-12));
}

#[test]
fn std_deviation_with_negative_values() {
    let values = [-2.0, -1.0, 1.0, 2.0];
    let sd = std_deviation(&values).expect("non-empty input should return Some");
    assert!(approx_eq(sd, (10.0_f64 / 3.0).sqrt(), 1e-12));
}

#[test]
fn std_deviation_is_non_negative() {
    let values = [-10.0, 0.0, 10.0, 5.0, -5.0];
    let sd = std_deviation(&values).expect("non-empty input should return Some");
    assert!(sd >= 0.0);
}

#[test]
fn std_deviation_is_sqrt_of_variance() {
    use rustats::primary::variance;
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let var = variance(&values).expect("non-empty input should return Some");
    let sd = std_deviation(&values).expect("non-empty input should return Some");
    assert!(approx_eq(sd, var.sqrt(), 1e-12));
}

#[test]
fn std_deviation_known_result() {
    // Sample variance = 32/7, std deviation = sqrt(32/7) ≈ 2.1381...
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let sd = std_deviation(&values).expect("non-empty input should return Some");
    assert!(approx_eq(sd, (32.0_f64 / 7.0).sqrt(), 1e-12));
}
