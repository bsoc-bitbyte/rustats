/// Returns the median of a sequence of `f64` values.
///
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::median;
///
/// let values = [5.0, 1.0, 3.0, 2.0, 4.0];
/// let m = median(&values);
/// assert_eq!(m, Some(3.0));
/// ```
pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut ordered = values.to_vec();
    ordered.sort_by(|left, right| {
        left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mid = ordered.len() / 2;

    if ordered.len() % 2 == 1 {
        ordered.get(mid).copied()
    } else {
        Some((ordered[mid - 1] + ordered[mid]) / 2.0)
    }
}
