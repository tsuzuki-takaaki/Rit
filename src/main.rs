use crate::error::TgitError;

mod error;

fn main() {
  println!("{}", TgitError::InvalidCommit);
}
