use std::fmt;

const CROSS: &str = "⨯";
const NAUGHT: &str = "○";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
  Naught,
  Cross
}

impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self {
          Player::Naught => write!(f, "{}", NAUGHT),
          Player::Cross => write!(f, "{}", CROSS)
      }
  }
}
