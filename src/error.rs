use std::io;
use std::fmt;

pub enum TgitError {
  IoError(io::Error),
  NoDirectory,
  InvalidCommit,
  InvalidIndex,
}

impl fmt::Display for TgitError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      &TgitError::IoError(ref e) => e.fmt(f),
      &TgitError::NoDirectory => f.write_str("No Directory Found"),
      &TgitError::InvalidCommit => f.write_str("The commit is invalid"),
      &TgitError::InvalidIndex => f.write_str("The index is corrupt"),
    }
  }
}

impl From<io::Error> for TgitError {
  fn from(value: io::Error) -> Self {
    TgitError::IoError(value)
  }
}
