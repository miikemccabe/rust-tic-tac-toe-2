use crate::player::Player;
use crate::animate::staggered_display;

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

  fn print_row(&self, row: usize) {
      let row_label = match row {
          0 => String::from("A"),
          1 => String::from("B"),
          2 => String::from("C"),
          _ => String::from("")
      };
      println!("|          |          |          |");
      println!("|     {}    |     {}    |     {}    |   {}",
        self.cell_as_string((row, 0)),
        self.cell_as_string((row, 1)),
        self.cell_as_string((row, 2)),
        row_label
      );
      println!("|          |          |          |");
  }

  fn format_row(&self, row: usize) -> String {
      let row_label = match row {
          0 => String::from("A"),
          1 => String::from("B"),
          2 => String::from("C"),
          _ => String::from("")
      };

      let mut s = String::new();
      s.push_str("|          |          |          |\n");
      let data = format!("|     {}    |     {}    |     {}    |   {}\n",
        self.cell_as_string((row, 0)),
        self.cell_as_string((row, 1)),
        self.cell_as_string((row, 2)),
        row_label
      );
      s.push_str(&data);
      s.push_str("|          |          |          |\n");
      s
  }

  fn cell_as_string(&self, coords: (usize, usize)) -> String {
    let cell = self.get_cell(coords);
    match cell {
      Some(player) => player.to_string(),
      None => String::from(" ")
    }
  }

  pub fn display(&self) {
      let mut s = String::new();
      s.push_str("     1           2          3     \n");
      s.push_str("+----------+----------+----------+\n");
      s.push_str(&self.format_row(0));
      s.push_str("+----------+----------+----------+\n");
      s.push_str(&self.format_row(1));
      s.push_str("+----------+----------+----------+\n");
      s.push_str(&self.format_row(2));
      s.push_str("+----------+----------+----------+");
      staggered_display(&s);
  }
}
