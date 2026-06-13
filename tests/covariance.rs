use rustats::primary::covariance;

fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}

#[test]
fn covariance_empty_returns_none() {
    // Covariance requires at least 2 values in each input (denominator is n-1)
    assert_eq!(covariance(&[], &[]), None);
}

#[test]
fn covariance_different_lengths(){
    //Covariance requires both input arrays to have the same length
    let values_x = [1.0, 2.0, 3.0];
    let values_y = [2.0, 3.0];
    assert_eq!(covariance(&values_x, &values_y),None);
}

#[test]
fn covariance_single_element_arrays_returns_none() {
    // Covariance requires at least 2 values in each input (denominator is n-1)
    assert_eq!(covariance(&[5.0], &[9.0]), None);
}

#[test]
fn covariance_two_identical_sample_arrays() {
    // If both input arrays are identical, the covariance should be equal to the variance of either array, which is 0 since all values are the same.
    let values_x = [4.0, 4.0];
    let values_y = [4.0, 4.0];
    let cv = covariance(&values_x, &values_y).expect("two or more values should return Some");
    assert!(approx_eq(cv, 0.0, 1e-12));
}

#[test]
fn covariance_smallest_valid_input() {
    // mean_x = 1.5, mean_y = 3.5, denominator= (-0.5*-0.5)+(0.5*0.5) = 0.5, covariance = denominator / (2-1) = 0.5 / 1 = 0.5
    let values_x = [1.0, 2.0];
    let values_y = [3.0, 4.0];
    let cv = covariance(&values_x, &values_y).expect("two or more values should return Some");
    assert!(approx_eq(cv, 0.5, 1e-12));
}

#[test]
fn covariance_multiple_values() {
    // mean_x = 3.0, mean_y = 7.0, deviations_x = [-2.0, -1.0, 0, 1.0, 2.0], deviations_y = [-2.0, -1.0, 0, 1.0, 2.0, denominator = (-2*-2) + (-1*-1) + (0*0) + (1*1) + (2*2) = 10.0 
    //covariance = denominator / (5-1) = 2.5
    let values_x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let values_y = [5.0, 6.0, 7.0, 8.0, 9.0];
    let cv = covariance(&values_x, &values_y).expect("non-empty input should return Some");
    assert!(approx_eq(cv, 2.5, 1e-12));
}

#[test]
fn covariance_with_negative_values() {
    // mean_x = 0.0, mean_y = 0.0, deviations_x = [-2, -1, 1, 2], deviations_y = [-5.0, -4.0, 4.0, 5.0]  denominator = (-2*-5)+(-1*-4)+(1*4)+(2*5) = 28.0 
    //covariance = 28 / 3 ≈ 9.333...
    let values_x = [-2.0, -1.0, 1.0, 2.0];
    let values_y = [-5.0, -4.0, 4.0, 5.0];
    let cv = covariance(&values_x, &values_y).expect("non-empty input should return Some");
    assert!(approx_eq(cv, 28.0 / 3.0, 1e-12));
}

#[test]
fn covariance_with_decimals() {
    // mean_x = 2.0, mean_y = 2.0, deviations_x = [-1.5, -0.5, 0.5, 1.5], deviations_y = [-1.5, -0.5, 0.5, 1.5], denominator = (-1.5*-1.5)+(-0.5*-0.5)+(0.5*0.5)+(1.5*1.5) = 5.0
    //covariance = 5 / 3 ≈ 1.6667
    let values_x = [0.5, 1.5, 2.5, 3.5];
    let values_y = [0.5, 1.5, 2.5, 3.5];
    let v = covariance(&values_x, &values_y).expect("non-empty input should return Some");
    assert!(approx_eq(v, 5.0 / 3.0, 1e-12));
}

#[test]
fn covariance_negative() {
    // covariance is negative when the deviations of x and y have opposite signs, which means that when x is above its mean, y tends to be below its mean, and vice versa. In this case, the covariance should be negative.
    // mean_x = 0.0, mean_y = 0.0, deviations_x = [-10.0, 0.0, 10.0, 5.0, -5.0], deviations_y = [10.0, 0.0, -10.0, -5.0, 5.0], denominator = (-10*10)+(0*0)+(10*-10)+(5*-5)+(-5*5) = -250.0
    //covariance = -250 / 4 = -62.5
    let values_x = [-10.0, 0.0, 10.0, 5.0, -5.0];
    let values_y = [10.0, 0.0, -10.0, -5.0, 5.0];
    let cv = covariance(&values_x, &values_y).expect("non-empty input should return Some");
    assert!(cv < 0.0);
}

#[test]
fn covariance_positive() {
    // covariance is positive when the deviations of x and y have the same signs, which means that when x is above its mean, y tends to be above its mean, and vice versa. In this case, the covariance should be positive.
    // mean_x = 0.0, mean_y = 0.0, deviations_x = [-10.0, 0.0, 10.0, 5.0, -5.0], deviations_y = [-10.0, 0.0, 10.0, 5.0, -5.0], denominator = (-10*-10)+(0*0)+(10*10)+(5*5)+(-5*-5) = 250.0
    //covariance = 250 / 4 = 62.5
    let values_x = [-10.0, 0.0, 10.0, 5.0, -5.0];
    let values_y = [-10.0, 0.0, 10.0, 5.0, -5.0];
    let cv = covariance(&values_x, &values_y).expect("non-empty input should return Some");
    assert!(cv > 0.0);

}