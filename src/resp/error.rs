use std::fmt::{self};

#[derive(Debug, PartialEq)]

pub enum ErrMessages {
    MissingBulkString,
    EmptyInput,
    UnknownInput(String),
    ParseError(String),
    UnexpectedVariant,
}

impl fmt::Display for ErrMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrMessages::MissingBulkString => write!(f, "Bulk string cannot be empty or null!"),
            ErrMessages::EmptyInput => write!(f, "Input cannot be empty!"),
            ErrMessages::UnexpectedVariant => write!(f, "Unexpected variant!"),
            ErrMessages::UnknownInput(details) => write!(f, "Unknown input: {}", details),
            ErrMessages::ParseError(details) => write!(
                f,
                "String might be containing unparsable length: {}",
                details
            ),
        }
    }
}

impl std::error::Error for ErrMessages {}
