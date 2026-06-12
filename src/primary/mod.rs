pub mod mean;
pub mod median;
pub mod mode;
pub mod quartiles;
pub mod variance;
pub mod std_deviation;
pub mod range;
pub mod covariance;


pub use mean::mean;
pub use median::median;
pub use mode::mode;
pub use quartiles::{Quartiles, quartiles};
pub use variance::variance;
pub use std_deviation::std_deviation;
pub use covariance::covariance;

pub use range::range;
