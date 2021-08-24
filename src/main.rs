mod board;
mod player;
mod game;
mod io;

fn main() {
    let mut g = game::Game::new();
    while g.get_status() == game::GameStatus::InProgress {
        println!("Player {}, take your turn", g.get_current_player());
        g.play();
        println!("{:?}", g);
    }
}
