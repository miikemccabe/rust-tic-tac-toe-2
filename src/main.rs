mod board;
mod player;
mod game;
mod io;

fn main() {

    let mut g = game::Game::new();

    print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
    g.display();
    while g.get_status() == game::GameStatus::InProgress {
        g.play();
        // println!("{:?}", g);
    }
}
