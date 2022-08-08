mod game;

use std::process;
use game::{BasicPlayer, Player, Game};

fn main() {
    println!("Quacks of Quedlinburg - Sim");

    let mut game = Game {
        name: "Quacks".to_string(),
        player: Player::new(BasicPlayer {}),
    };

    if let Err(err) = game.run() {
        println!("{}", err);
        process::exit(1);
    }
}
