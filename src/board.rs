use std::fmt;
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

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let a1 = self.get_cell((0,0)).map_or_else(|| " ".to_string(), |p| p.to_string());
    let a2 = self.get_cell((0,1)).map_or_else(|| " ".to_string(), |p| p.to_string());
    let a3 = self.get_cell((0,2)).map_or_else(|| " ".to_string(), |p| p.to_string());

    let b1 = self.get_cell((1,0)).map_or_else(|| " ".to_string(), |p| p.to_string());
    let b2 = self.get_cell((1,1)).map_or_else(|| " ".to_string(), |p| p.to_string());
    let b3 = self.get_cell((1,2)).map_or_else(|| " ".to_string(), |p| p.to_string());

    let c1 = self.get_cell((2,0)).map_or_else(|| " ".to_string(), |p| p.to_string());
    let c2 = self.get_cell((2,1)).map_or_else(|| " ".to_string(), |p| p.to_string());
    let c3 = self.get_cell((2,2)).map_or_else(|| " ".to_string(), |p| p.to_string());

    writeln!(f, "     1           2          3     ")?;
    writeln!(f, "┏━━━━━━━━━━┳━━━━━━━━━━┳━━━━━━━━━━┓")?;
    writeln!(f, "┃          ┃          ┃          ┃")?;
    writeln!(f, "┃     {}    ┃     {}    ┃     {}    ┃  A", a1, a2, a3)?;
    writeln!(f, "┃          ┃          ┃          ┃")?;
    writeln!(f, "┣━━━━━━━━━━╋━━━━━━━━━━╋━━━━━━━━━━┫")?;
    writeln!(f, "┃          ┃          ┃          ┃")?;
    writeln!(f, "┃     {}    ┃     {}    ┃     {}    ┃  B", b1, b2, b3)?;
    writeln!(f, "┃          ┃          ┃          ┃")?;
    writeln!(f, "┣━━━━━━━━━━╋━━━━━━━━━━╋━━━━━━━━━━┫")?;
    writeln!(f, "┃          ┃          ┃          ┃")?;
    writeln!(f, "┃     {}    ┃     {}    ┃     {}    ┃  C", c1, c2, c3)?;
    writeln!(f, "┃          ┃          ┃          ┃")?;
    writeln!(f, "┗━━━━━━━━━━┻━━━━━━━━━━┻━━━━━━━━━━┛")?;
    Ok(())
  }
}
