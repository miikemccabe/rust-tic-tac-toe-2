mod board;
mod player;
mod game;
mod io;
mod animate;

fn main() {

    let mut g = game::Game::new();

    g.display();
    while g.get_status() == game::GameStatus::InProgress {
        println!("Player {}, take your turn", g.get_current_player());
        g.play();
        g.display();
        println!("{:?}", g);
    }
}
