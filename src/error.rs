use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum Error {
    Env { key: &'static str },
    Tokio,
    Kafka,
    Serde,
    Other,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Env { key } => write!(f, "Problem occurred by environment variable `{}` is not set.", key),
            Error::Tokio => write!(f, "Caused by Tokio"),
            Error::Kafka => write!(f, "Caused by Kafka"),
            Error::Serde => write!(f, "Caused by Serde"),
            Error::Other => write!(f, "Problem occurred in a place that is not here..."),
        }
    }
}

impl Context for Error {}