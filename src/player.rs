use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
  Naught,
  Cross
}

impl Player {
  fn to_string(&self) -> &str {
    match self {
      Player::Cross => "X",
      Player::Naught => "O"
    }
  }
}

impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self {
          Player::Naught => write!(f, "O"),
          Player::Cross => write!(f, "X")
      }
  }
}
