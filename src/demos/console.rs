use std::{io::stdin, str::FromStr};

pub fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn parse_line<T>() -> Result<T, T::Err>
where
    T: FromStr,
{
    let line = read_line();
    let number = line.trim().parse();
    number
}

pub fn pause() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
}
