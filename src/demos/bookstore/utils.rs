use std::{error::Error, io::stdin, num::ParseIntError, str::FromStr};

pub fn read_selection() -> Result<usize, ParseIntError> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<usize>()
}

pub fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn read_year() -> Result<i32, ParseIntError> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i32>()
}

pub fn parse_line<T>() -> Result<T, T::Err>
where
    T: FromStr,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let number = buffer.trim().parse();
    number
}
