mod board;
mod player;
mod game;
mod io;
mod animate;

fn main() {

    let mut g = game::Game::new();

    g.display();
    while g.get_status() == game::GameStatus::InProgress {
        g.play();
        // println!("{:?}", g);
    }
}
