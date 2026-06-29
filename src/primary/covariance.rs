/// Returns the arithmetic mean of a sequence of `f64` values.
///
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::mean;
///
/// let values_x = [1.0, 2.0, 3.0];
/// let values_y = [4.0, 5.0, 6.0];
/// let cv = covariance(&values_x, &values_y);
/// assert_eq!(cv, Some(1.0));
/// ```
use super::mean;

pub fn covariance(x: &[f64], y: &[f64]) -> Option<f64> {
    let mut denominator: f64 = 0.0;

    if x.len() != y.len(){
        return None;
    }

    if x.len() < 2 {
            return None;
    }

    let mean_x = mean(x)?;
    let mean_y = mean(y)?;

        for i in 0..x.len() {
            denominator += (x[i]-mean_x)*(y[i]-mean_y);
        }
        Some(denominator / (x.len() - 1) as f64)

}
    

