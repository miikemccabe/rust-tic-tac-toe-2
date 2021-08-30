use crate::board::Board;
use crate::player::Player;
use crate::io::{get_choice, Command, Choice};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameStatus {
  Won(Player),
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
    match get_choice() {
      Ok(Choice::Command(cmd)) => self.handle_command(&cmd),
      Ok(Choice::Cell((row, col))) => {
        match self.board.set_cell((row, col), self.current_player) {
          Ok(()) => {
            print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
            self.toggle_player();
            self.display();
            println!("Player {}, take your turn", self.get_current_player());
            println!();
            println!("c: Clear the board");
            println!("q: quit");
            println!();
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
      },
    }
  }

  fn toggle_player(&mut self) {
    self.current_player = match self.current_player {
      Player::Cross => Player::Naught,
      Player::Naught => Player::Cross
    }
  }
}
