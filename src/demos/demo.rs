use std::error::Error;

pub type DemoResult = Result<(), Box<dyn Error>>;
pub type Demo = fn() -> DemoResult;
