use std::error::Error;

use perm_macro::pub_handlers;
// standard error:

pub type ResultHandler<T> = Result<T, Box<dyn Error>>;
pub_handlers!();