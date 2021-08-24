use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
  Naught,
  Cross
}

impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self {
          Player::Naught => write!(f, "O"),
          Player::Cross => write!(f, "X")
      }
  }
}
