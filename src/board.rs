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

  fn format_values(&self, row: usize) -> String {
      let row_label = match row {
          0 => String::from("A"),
          1 => String::from("B"),
          2 => String::from("C"),
          _ => String::from("")
      };

      let mut s = String::new();
      let data = format!("|     {}    |     {}    |     {}    |   {}",
        self.cell_as_string((row, 0)),
        self.cell_as_string((row, 1)),
        self.cell_as_string((row, 2)),
        row_label
      );
      s.push_str(&data);
      s
  }

  fn cell_as_string(&self, coords: (usize, usize)) -> String {
    let cell = self.get_cell(coords);
    match cell {
      Some(player) => player.to_string(),
      None => String::from(" ")
    }
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "     1           2          3     ");
    writeln!(f, "+----------+----------+----------+");
    writeln!(f, "|          |          |          |");
    writeln!(f, "{}", self.format_values(0));
    writeln!(f, "|          |          |          |");
    writeln!(f, "+----------+----------+----------+");
    writeln!(f, "|          |          |          |");
    writeln!(f, "{}", self.format_values(1));
    writeln!(f, "|          |          |          |");
    writeln!(f, "+----------+----------+----------+");
    writeln!(f, "|          |          |          |");
    writeln!(f, "{}", self.format_values(2));
    writeln!(f, "|          |          |          |");
    writeln!(f, "+----------+----------+----------+");
    Ok(())
  }
}
