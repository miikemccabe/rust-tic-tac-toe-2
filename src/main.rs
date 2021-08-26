mod board;
mod player;
mod game;
mod io;
mod animate;

fn main() {

    let mut g = game::Game::new();

    while g.get_status() == game::GameStatus::InProgress {
        print!("\x1B[2J\x1B[1;1H");
        g.display();
        println!("Player {}, take your turn", g.get_current_player());
        println!();
        println!("c: Clear the board");
        println!("q: quit");
        println!();
        g.play();
        // println!("{:?}", g);
    }
}
