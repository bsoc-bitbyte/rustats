/// Returns the range of the given sequence of f64 values
/// 
/// 
/// Range = max - min
/// 
/// Empty input returns `None`.
/// 
/// # Example
/// 
/// ```
/// use rustats::primary::range;
/// let values = [2.3,4.1,2.2];
/// assert_eq!(range(&values),Some(1.9));
/// 
/// ```


pub fn range(values: &[f64]) -> Option<f64> {

    if values.is_empty(){
        return None;
    }

    let mut min = values[0];
    let mut max = values[0];

    for &val in values{
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    Some(max - min)
}