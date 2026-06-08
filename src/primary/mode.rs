/// Returns the mode of a sequence of 'f64' values.
///
/// Empty input returns 'None'.
///
/// If multiple values have the same highest frequency , the smallest value
/// among them is returned.
///
///# Example
/// ```
/// use rustats::primary::mode;
///
/// let values = [1.0, 2.0, 2.0, 3.0];
/// assert_eq!(mode(&values), Some(2.0));
/// ```
///
/// ```
/// use rustats::primary::mode;
///
/// let values = [1.0, 1.0, 2.0, 2.0];
/// assert_eq!(mode(&values), Some(1.0));
/// ```

pub fn mode(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| a.partial_cmp(b).expect("mode does not support NaN values"));

    let mut curr_mode_value = sorted[0];
    let mut max_count = 1;

    let mut current_value = sorted[0];
    let mut current_count = 1;

    for &value in &sorted[1..] {
        if value == current_value {
            current_count += 1;
        } else {
            if current_count > max_count {
                max_count = current_count;
                curr_mode_value = current_value;
            }
            current_value = value;
            current_count = 1;
        }
    }
    if current_count > max_count {
        curr_mode_value = current_value;
    }
    Some(curr_mode_value)
}
