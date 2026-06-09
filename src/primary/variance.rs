/// Returns the arithmetic mean of a sequence of `f64` values.
///
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::variance;
///
/// let values = [1.0, 2.0, 3.0];
/// let var = variance(&values);
/// assert_eq!(var, Some(0.6666666666666666));
/// ```
/// 

use super::mean;


pub fn variance(values: &[f64]) -> Option<f64> {
    let mut count: u64 = 0;
    let mut square_sum: f64 = 0.0;
    let avg = mean(&values)?;

    for value   in values {
        square_sum += (*value-avg)*(*value-avg);
        count += 1;
    }

    if count < 2{
        None
    }

    else {
       Some((square_sum)/ (count - 1)  as f64) 
    }
        
}
