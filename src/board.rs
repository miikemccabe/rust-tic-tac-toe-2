use std::fmt;
use crate::player::Player;

const GRID: [Option<Player>; 9] = [None; 9];

#[derive(Debug)]
pub struct Board {
  grid: [Option<Player>; 9]
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
  pub fn get_cell(&self, cell: usize) -> &Option<Player> {
    if cell < self.grid.len() {
      &self.grid[cell]
    } else {
      &None
    }
  }
  pub fn set_cell(&mut self, cell: usize, value: Player) -> Result<(), String> {
    if cell < self.grid.len() {
      if self.get_cell(cell).is_none() {
        self.grid[cell] = Some(value);
        Ok(())
      } else {
        Err(String::from("Cell has already been set"))
      }
    } else {
      Err(format!("Can't set cell at {}", cell))
    }
  }
  pub fn is_full(&self)-> bool {
    let mut full = true;
    for cell in &self.grid {
      if cell.is_none() {
        full = false;
        break;
      }
    }
    full
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let a1 = self.get_cell(0).map_or_else(|| " ".to_string(), |p| p.to_string());
    let a2 = self.get_cell(1).map_or_else(|| " ".to_string(), |p| p.to_string());
    let a3 = self.get_cell(2).map_or_else(|| " ".to_string(), |p| p.to_string());

    let b1 = self.get_cell(3).map_or_else(|| " ".to_string(), |p| p.to_string());
    let b2 = self.get_cell(4).map_or_else(|| " ".to_string(), |p| p.to_string());
    let b3 = self.get_cell(5).map_or_else(|| " ".to_string(), |p| p.to_string());

    let c1 = self.get_cell(6).map_or_else(|| " ".to_string(), |p| p.to_string());
    let c2 = self.get_cell(7).map_or_else(|| " ".to_string(), |p| p.to_string());
    let c3 = self.get_cell(8).map_or_else(|| " ".to_string(), |p| p.to_string());

    writeln!(f, "     1           2          3     ")?;
    writeln!(f, "┏━━━━━━━━━━━┳━━━━━━━━━━━┳━━━━━━━━━━━┓")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃     {}     ┃     {}     ┃     {}     ┃  A", a1, a2, a3)?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┣━━━━━━━━━━━╋━━━━━━━━━━━╋━━━━━━━━━━━┫")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃     {}     ┃     {}     ┃     {}     ┃  B", b1, b2, b3)?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┣━━━━━━━━━━━╋━━━━━━━━━━━╋━━━━━━━━━━━┫")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃     {}     ┃     {}     ┃     {}     ┃  C", c1, c2, c3)?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┗━━━━━━━━━━━┻━━━━━━━━━━━┻━━━━━━━━━━━┛")?;
    Ok(())
  }
}
