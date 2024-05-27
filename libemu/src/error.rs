use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
  RomSize,
  CustomError(String),
}

impl Error {
  pub fn description(&self) -> &str {
    match self {
      Error::RomSize => "Invalid ROM size",
      Error::CustomError(message) => message,
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}
