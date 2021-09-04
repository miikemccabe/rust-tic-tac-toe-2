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
    let cells_as_strings: Vec<String> = self.grid.iter().map(
      |c| {
        c.map_or_else(|| " ".to_string(),|p| p.to_string())
      }
    ).collect();

    writeln!(f, "     1           2          3     ")?;
    writeln!(f, "┏━━━━━━━━━━━┳━━━━━━━━━━━┳━━━━━━━━━━━┓")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃     {}     ┃     {}     ┃     {}     ┃  A", cells_as_strings[0], cells_as_strings[1], cells_as_strings[2])?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┣━━━━━━━━━━━╋━━━━━━━━━━━╋━━━━━━━━━━━┫")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃     {}     ┃     {}     ┃     {}     ┃  B", cells_as_strings[3], cells_as_strings[4], cells_as_strings[5])?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┣━━━━━━━━━━━╋━━━━━━━━━━━╋━━━━━━━━━━━┫")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃     {}     ┃     {}     ┃     {}     ┃  C", cells_as_strings[6], cells_as_strings[7], cells_as_strings[8])?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┃           ┃           ┃           ┃")?;
    writeln!(f, "┗━━━━━━━━━━━┻━━━━━━━━━━━┻━━━━━━━━━━━┛")?;
    Ok(())
  }
}
