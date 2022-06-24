use std::error::Error;

use crate::demos::console;

pub type DemoResult = Result<(), Box<dyn Error>>;
