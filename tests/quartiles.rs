use rustats::primary::quartiles;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    return (left - right).abs() <= epsilon;
}

fn check_quartiles(values: &[f64], q1: f64, q2: f64, q3: f64) {
    let result = quartiles(values).expect("non empty input should return Some");

    assert!(approx_eq(result.q1, q1, 1e-12));
    assert!(approx_eq(result.q2, q2, 1e-12));
    assert!(approx_eq(result.q3, q3, 1e-12));
}

#[test]
fn quartile_empty_input_check() {
    assert!(quartiles(&[]).is_none());
}

#[test]
fn quartiles_single_element_returns_same_values() {
    check_quartiles(&[5.0], 5.0, 5.0, 5.0);
}

#[test]
fn quartiles_two_elements() {
    check_quartiles(&[2.0, 8.0], 2.0, 5.0, 8.0);
}

#[test]
fn quartiles_three_elements_excludes_median() {
    check_quartiles(&[1.0, 2.0, 3.0], 1.0, 2.0, 3.0);
}

#[test]
fn quartiles_even_length_four() {
    check_quartiles(&[1.0, 2.0, 3.0, 4.0], 1.5, 2.5, 3.5);
}

#[test]
fn quartiles_odd_length_five_excludes_median() {
    check_quartiles(&[1.0, 2.0, 3.0, 4.0, 5.0], 1.5, 3.0, 4.5);
}

#[test]
fn quartiles_even_length_six() {
    check_quartiles(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2.0, 3.5, 5.0);
}

#[test]
fn quartiles_unsorted_input() {
    check_quartiles(&[5.0, 1.0, 3.0, 2.0, 4.0], 1.5, 3.0, 4.5);
}

#[test]
fn quartiles_with_duplicate_values() {
    check_quartiles(&[1.0, 2.0, 2.0, 2.0, 3.0], 1.5, 2.0, 2.5);
}

#[test]
fn quartiles_all_same_values() {
    check_quartiles(&[4.0, 4.0, 4.0, 4.0], 4.0, 4.0, 4.0);
}

#[test]
fn quartiles_with_negative_values() {
    check_quartiles(&[-5.0, -3.0, -1.0, 1.0, 3.0], -4.0, -1.0, 2.0);
}

#[test]
fn quartiles_with_decimal_values() {
    check_quartiles(&[0.5, 1.5, 2.5, 3.5], 1.0, 2.0, 3.0);
}
