use std::str::FromStr;
use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error while loading input: {0:?}")]
    Io(#[from] std::io::Error),

    #[error("malformed UTF-8 string in input")]
    Utf8(#[from] FromUtf8Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),

    #[error("no output for {0}/{1} part {}{}", <aoc_common::Part as FromStr>::from_str(.2).unwrap(), .3.as_ref().map(|v| format!(r#" (version "{v}""#)).unwrap_or_default())]
    NoOutput(usize, usize, String, Option<String>),

    #[error("solution for {0}/{1} part {2}{} not registered", .3.clone().map(|v| format!(r#" (version "{v}""#)).unwrap_or_default())]
    NotRegistered(usize, usize, aoc_common::Part, Option<String>),

    #[error("error while parsing '{0}'")]
    Parse(String),
}

impl Error {
    pub fn from_error<E>(e: E) -> Self
    where E: Into<eyre::Report>,
    {
        Error::Other(e.into())
    }
}
