#[derive(Debug, PartialEq)]
pub struct Quartiles {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
}

/// Returns the quartiles of a sequence of `f64` values.
///
/// Empty input returns `None`.
///
/// `q2` is the median of the entire dataset.
///
/// `q1` is the median of the lower half of the dataset, and `q3` is the
/// median of the upper half of the dataset.
///
/// For odd-length datasets, the median element is excluded before calculating
/// `q1` and `q3`.
///
/// For even-length datasets, the dataset is split in half and the median of
/// each half is used for `q1` and `q3`.
///
/// For a single-element input, the same value is returned for `q1`, `q2`,
/// and `q3`.
///
/// # Example
///
/// ```
/// use rustats::primary::quartiles;
///
/// let values = [1.0, 2.0, 3.0, 4.0];
/// let result = quartiles(&values).unwrap();
///
/// assert_eq!(result.q1, 1.5);
/// assert_eq!(result.q2, 2.5);
/// assert_eq!(result.q3, 3.5);
/// ```
pub fn quartiles(values: &[f64]) -> Option<Quartiles> {
    if values.is_empty() {
        return None;
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| {
        a.partial_cmp(b)
            .expect("quartiles does not support NaN values")
    });

    let length = sorted.len();

    if length == 1 {
        let value = sorted[0];

        return Some(Quartiles {
            q1: value,
            q2: value,
            q3: value,
        });
    }

    let q2 = median(&sorted, 0, length - 1);
    let mid = length / 2;

    let (q1, q3) = if length % 2 == 0 {
        (
            median(&sorted, 0, mid - 1),
            median(&sorted, mid, length - 1),
        )
    } else {
        (
            median(&sorted, 0, mid - 1),
            median(&sorted, mid + 1, length - 1),
        )
    };

    Some(Quartiles { q1, q2, q3 })
}

fn median(values: &[f64], left: usize, right: usize) -> f64 {
    let length = right - left + 1;
    let mid = left + length / 2;

    if length % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    }
}
