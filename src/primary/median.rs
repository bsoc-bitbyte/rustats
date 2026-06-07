/// Returns the median of a sequence of `f64` values.
///
/// For an odd number of values, returns the middle value after sorting.
/// For an even number of values, returns the average of the two middle values.
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::median;
///
/// let values = [1.0, 3.0, 2.0];
/// let m = median(&values);
/// assert_eq!(m, Some(2.0));
/// ```
pub fn median(values: &[f64]) -> Option<f64> {
      if values.is_empty() {
                return None;
      }

    let mut sorted = values.to_vec();
      sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted.len();
      if len % 2 == 1 {
                Some(sorted[len / 2])
      } else {
                Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
      }
}
