use std::str::FromStr;

use crate::error::Error;

pub fn input_bytes(year: usize, day: usize) -> Result<Vec<u8>, Error> {
    Ok(std::fs::read(format!(
        "{}/data/input/y{year}/day{day:02}.txt",
        std::env::current_dir().unwrap().display(),
    ))?)
}

pub fn input_string(year: usize, day: usize) -> Result<String, Error> {
    Ok(String::from_utf8(input_bytes(year, day)?)?)
}

pub fn parse_input<T>(input: &str, sep: Option<&str>) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Into<eyre::Report>,
{
    match sep {
        Some(sep) => input
            .trim()
            .split(sep)
            .map(|s| s.parse().map_err(Error::from_error))
            .collect(),
        None => input
            .lines()
            .map(|s| s.parse().map_err(Error::from_error))
            .collect(),
    }
}

pub fn parse_input_bytes(input: &[u8], sep: Option<u8>) -> Result<Vec<&[u8]>, Error> {
    let sep = sep.unwrap_or(b'\n');

    Ok(input
        .strip_suffix(&[sep])
        .unwrap_or(input)
        .split(|&b| b == sep)
        .collect()
    )
}
