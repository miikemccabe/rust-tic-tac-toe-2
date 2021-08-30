use crate::board::Board;
use crate::player::Player;
use crate::io::{get_choice, Command, Choice};

const WINNING_COMBOS: [[(usize, usize); 3]; 8]  = [
  [(0,0), (0,1), (0,2)],
  [(1,0), (1,1), (1,2)],
  [(2,0), (2,1), (2,2)],
  [(0,0), (1,0), (2,0)],
  [(0,1), (1,1), (2,1)],
  [(0,2), (1,2), (2,2)],
  [(0,0), (1,1), (2,2)],
  [(2,0), (1,1), (0,2)],
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStatus {
  Won(Player),
  Draw,
  Quit,
  InProgress,
}

#[derive(Debug)]
pub struct Game {
  status: GameStatus,
  current_player: Player,
  board: Board,
}

impl Game {
  pub fn new() -> Self {
    Game {
      status: GameStatus::InProgress,
      current_player: Player::Cross,
      board: Board::new(),
    }
  }

  pub fn get_current_player(&self) -> Player {
    self.current_player
  }

  pub fn get_status(&self) -> GameStatus {
    self.status
  }

  pub fn display(&self) {
    print!("{}", &self.board.to_string());
  }

  pub fn play(&mut self) {
    print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
    self.display();
    println!("Player {}, take your turn", self.get_current_player());
    println!();
    println!("c: Clear the board");
    println!("q: quit");
    println!();
    match get_choice() {
      Ok(Choice::Command(cmd)) => self.handle_command(&cmd),
      Ok(Choice::Cell((row, col))) => {
        match self.board.set_cell((row, col), self.current_player) {
          Ok(()) => {
            self.toggle_player();
          },
          Err(err) => {
            println!("{}", err);
          }
        }
      },
      Err(err) => {
        println!("{}", err);
      }
    }
    if let Some(player) = self.find_winner() {
      self.status = GameStatus::Won(*player);
      self.handle_win();
    }
    if self.board.is_full() {
      self.handle_draw();
    }
  }

  fn handle_win(&self) {
    if let GameStatus::Won(player) = self.status {
      print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
      self.display();
      println!();
      println!("Player {} wins!", player);
      println!();
    }
  }

  fn handle_draw(&mut self) {
    self.status = GameStatus::Draw;
    print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
    self.display();
    println!();
    println!("It's a draw!");
    println!();
  }

  fn handle_command(&mut self, cmd: &Command) {
    match cmd {
      Command::Quit => {
        self.status = GameStatus::Quit;
        println!("Player {} quit. Game over.", self.current_player);
      },
      Command::Clear => {
        self.current_player = Player::Cross;
        self.board.clear();
        self.display();
      },
    }
  }

  fn toggle_player(&mut self) {
    self.current_player = match self.current_player {
      Player::Cross => Player::Naught,
      Player::Naught => Player::Cross
    }
  }

  fn find_winner(&self) -> Option<&Player> {
    let mut winner: Option<&Player> = None;

    for combo in WINNING_COMBOS {
      // Get the player at the first cell of each winning combo
      let player = self.board.get_cell((combo[0].0, combo[0].1));
      // Loop over each cell of the 3 cell winning combo
      for cell in combo {
        // Get the value of the cell
        let current = self.board.get_cell((cell.0, cell.1));
        // If this cell matches the current player, we've got a potential winner
        if current.is_some() && player.is_some() && current.unwrap() == player.unwrap() {
          // Tentatively set this as the winner and continue checking the rest of the combo
          winner = current.as_ref();
        } else {
          // If any of the cells in this combo don't match the player this is not a winning combo. Give up on it.
          winner = None;
          break;
        }
      }
      if winner.is_some() {
        // If we have a winner after checking every cell in a combo, then we have a winner!
        break;
      }
    }

    winner
  }
}
