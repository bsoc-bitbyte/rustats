use rustats::primary::range;


fn approx_eq(left: f64, right: f64, epsilon: f64) -> bool {
    (left - right).abs() <= epsilon
}


#[test]
fn range_empty_returns_none(){
    assert_eq!(range(&[]),None);
}

#[test]
fn range_single_value_returns_zero(){
    assert_eq!(range(&[7.0]), Some(0.0));
}

#[test]
fn range_normal_case(){
    let values = [5.0,2.0,10.0];
    let r = range(&values).expect("Non empty input should return Some");
    assert!(approx_eq(r, 8.0, 1e-12));
}

#[test]
fn range_negative_numbers(){
    let values = [-5.0,3.0,4.0,-1.0];
    let r = range(&values).expect("Non empty input should return Some");
    assert!(approx_eq(r,9.0,1e-12));
}


#[test]
fn range_if_all_equal_returns_zero(){
    let values = [3.0,3.0,3.0];
    let r = range(&values).expect("Non empty input should return Some");
    assert!(approx_eq(r,0.0,1e-12));
}

#[test]
fn range_if_all_decimals(){
    let values = [1.2,3.4,3.3,7.3];
    let r = range(&values).expect("Non empty input should return Some");
    assert!(approx_eq(r,6.1,1e-12));
}


#[test]
fn range_if_there_are_negative_decimals(){
    let values = [-1.3,-2.2,-4.3];
    let r = range(&values).expect("Non empty input should return Some");
    assert!(approx_eq(r,3.0,1e-12));
}

#[test]
fn range_if_there_is_mix_of_signed_decimals_and_floats(){
    let values = [1.2,-3.2,7.0,3.0,-4.2];
    let r = range(&values).expect("Non empty input should return Some");
    assert!(approx_eq(r,11.2,1e-12));
}
