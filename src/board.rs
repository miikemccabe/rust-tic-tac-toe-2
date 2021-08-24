use crate::player::Player;

const GRID: [[Option<Player>; 3]; 3] = [
  [None, None, None],
  [None, None, None],
  [None, None, None]
];

#[derive(Debug)]
pub struct Board {
  grid: [[Option<Player>; 3]; 3]
}

impl Board {
  pub fn new() -> Board {
    Board {
      grid: GRID
    }
  }
  pub fn clear(&mut self) {
    self.grid = GRID;
  }
  pub fn get_cell(&self, coords: (usize, usize)) -> &Option<Player> {
    let (row, col) = coords;
    if self.coords_in_range(coords) {
      &self.grid[row][col]
    } else {
      &None
    }
  }
  pub fn set_cell(&mut self, coords: (usize, usize), value: Player) -> Result<(), String> {
    let (row, col) = coords;
    if self.coords_in_range(coords) {
      self.grid[row][col] = Some(value);
      Ok(())
    } else {
      Err(format!("Can't set cell at {},{}", row, col))
    }
  }
  fn coords_in_range(&self, coords: (usize, usize)) -> bool {
    let len = &self.grid.len();
    let (row, col) = coords;
    !(row > *len || col > *len)
  }
}
