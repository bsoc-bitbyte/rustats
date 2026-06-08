/// Returns the median (middle) value of a sequence of `f64` values
///
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::median;
///
/// let values = [4.0, 3.0, 1.0, 3.0];
/// let med = median(&values);
/// assert_eq!(med, Some(3.0));
/// ```

pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut sorted = values.to_vec();
    let n = sorted.len();

    sorted.sort_by(|a, b| {
        a.partial_cmp(b)
            .expect("Median cannot be computed for NaN values!")
    });

    let m;
    if n % 2 == 0 {
        let i1 = n / 2;
        m = (sorted[i1 - 1] + sorted[i1]) / 2.0;
    } else {
        let i2 = (n + 1) / 2;
        m = sorted[i2 - 1];
    }

    Some(m)
}
