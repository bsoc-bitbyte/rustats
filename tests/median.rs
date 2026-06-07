use rustats::primary::median;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}

#[test]
fn median_empty_returns_none() {
    assert_eq!(median(&[]), None);
}

#[test]
fn median_single_value() {
    assert_eq!(median(&[7.0]), Some(7.0));
}

#[test]
fn median_odd_count_returns_middle() {
    let values = [3.0, 1.0, 4.0, 2.0, 5.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 3.0, 1e-12));
}

#[test]
fn median_even_count_averages_middle_two() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 2.5, 1e-12));
}

#[test]
fn median_already_sorted_input() {
    let values = [10.0, 20.0, 30.0, 40.0, 50.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 30.0, 1e-12));
}

#[test]
fn median_reverse_sorted_input() {
    let values = [50.0, 40.0, 30.0, 20.0, 10.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 30.0, 1e-12));
}

#[test]
fn median_with_negative_values() {
    let values = [3.0, -1.0, 0.0, -5.0, 4.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 0.0, 1e-12));
}

#[test]
fn median_with_decimals() {
    let values = [0.3, 0.1, 0.2];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 0.2, 1e-12));
}

#[test]
fn median_with_duplicate_values() {
    let values = [2.0, 2.0, 2.0, 2.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 2.0, 1e-12));
}

#[test]
fn median_two_elements_averages_both() {
    let values = [7.0, 3.0];
    let m = median(&values).expect("non-empty input should return Some");
    assert!(approx_eq(m, 5.0, 1e-12));
}

#[test]
fn median_does_not_mutate_original_slice() {
    let values = [3.0, 1.0, 2.0];
    let _ = median(&values);
    assert_eq!(values, [3.0, 1.0, 2.0]);
}
