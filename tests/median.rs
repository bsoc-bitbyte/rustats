use rustats::primary::median;

#[test]
fn median_empty_returns_none() {
    assert_eq!(median(&[]), None);
}

#[test]
fn median_single_value() {
    assert_eq!(median(&[42.0]), Some(42.0));
}

#[test]
fn median_odd_count_uses_sorted_middle() {
    let values = [5.0, 1.0, 3.0, 2.0, 4.0];
    let result = median(&values).expect("non-empty input should return Some");
    assert_eq!(result, 3.0);
}

#[test]
fn median_even_count_averages_middle_values() {
    let values = [10.0, 1.0, 3.0, 2.0];
    let result = median(&values).expect("non-empty input should return Some");
    assert_eq!(result, 2.5);
}

#[test]
fn median_with_negatives_and_decimals() {
    let values = [2.5, -1.5, 0.5, 1.5];
    let result = median(&values).expect("non-empty input should return Some");
    assert_eq!(result, 1.0);
}
