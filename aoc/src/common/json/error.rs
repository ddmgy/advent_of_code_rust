use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidObjectKey,
    LeftOverInput(Range<usize>),
    ParseNumber(std::num::ParseIntError),
    ParseString(std::string::FromUtf8Error),
    UnbalancedBracket(char),
    UnexpectedChar(char),
    UnexpectedEndOfInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidObjectKey => write!(f, "invalid object key")?,
            Self::LeftOverInput(Range { start, end }) => write!(f, "left over input in range {}..{}", start, end)?,
            Self::ParseNumber(e) => write!(f, "error parsing number: {}", e)?,
            Self::ParseString(e) => write!(f, "error parsing string: {}", e)?,
            Self::UnbalancedBracket(char) => write!(f, "unbalanced bracket: {}", char)?,
            Self::UnexpectedChar(char) => write!(f, "unexpected char: {}", char)?,
            Self::UnexpectedEndOfInput => write!(f, "unexpected end of input")?,
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseNumber(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::ParseString(value)
    }
}
