use rustats::primary::interquartile_range;

#[test]
fn empty_returns_none() {
    assert_eq!(interquartile_range(&[]), None);
}

#[test]
fn single_value_returns_zero() {
    assert_eq!(
        interquartile_range(&[10.0]),
        Some(0.0)
    );
}

#[test]
fn odd_length_dataset() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];

    assert_eq!(
        interquartile_range(&values),
        Some(3.0)
    );
}

#[test]
fn even_length_dataset() {
    let values = [1.0, 2.0, 3.0, 4.0];

    assert_eq!(
        interquartile_range(&values),
        Some(2.0)
    );
}