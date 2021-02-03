use std::error::Error;

pub type CrudResult<T> = Result<T, Box<dyn Error>>;