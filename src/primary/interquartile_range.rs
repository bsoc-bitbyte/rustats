use crate::primary::quartiles;

pub fn interquartile_range(values: &[f64]) -> Option<f64> {
    let quartiles = quartiles(values)?;

    Some(quartiles.q3 - quartiles.q1)
}