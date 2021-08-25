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

  fn cell_as_string(&self, coords: (usize, usize)) -> &str {
    let cell = self.get_cell(coords);
    match cell {
      Some(Player::Cross) => "x",
      Some(Player::Naught) => "o",
      None => " "
    }
  }

  pub fn display(&self) {
      println!("     1           2          3     ");
      println!("+----------+----------+----------+");
      self.print_row(0);
      println!("+----------+----------+----------+");
      self.print_row(1);
      println!("+----------+----------+----------+");
      self.print_row(2);
      println!("+----------+----------+----------+");
  }
}
