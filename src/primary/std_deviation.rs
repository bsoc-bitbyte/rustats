use super::variance;

/// Returns the standard deviation of a sequence of `f64` values.
///
/// Empty input returns `None`.
///
/// # Example
///
/// ```
/// use rustats::primary::std_deviation;
///
/// let values = [1.0, 2.0, 3.0];
/// let std_dev = std_deviation(&values);
/// assert_eq!(std_dev, Some(1.0));
/// ```



pub fn std_deviation(values: &[f64]) -> Option<f64> {
    let var = variance(values)?;
    Some(var.sqrt())
}

