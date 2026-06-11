pub mod mean;
pub mod median;
pub mod mode;
pub mod quartiles;
pub mod variance;
pub mod std_deviation;
pub mod interquartile_range;
pub mod range;


pub use mean::mean;
pub use median::median;
pub use mode::mode;
pub use quartiles::{Quartiles, quartiles};
pub use variance::variance;
pub use std_deviation::std_deviation;
pub use interquartile_range::interquartile_range;
pub use range::range;