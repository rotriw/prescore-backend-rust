
use std::error::Error;
// standard error:


pub type ResultHandler<T> = Result<T, Box<dyn Error>>;
pub mod exam;