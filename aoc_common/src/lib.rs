#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Part {
    A,
    B,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Other(#[from] eyre::Report),

    #[error(r#"failed to parse Part"#)]
    ParsePartError(),
}

impl Error {
    pub fn from_error<E>(e: E) -> Self
    where E: Into<eyre::Report>,
    {
        Error::Other(e.into())
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::A => "A",
            Self::B => "B",
        })
    }
}

impl std::str::FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParsePartError())
        }
    }
}
