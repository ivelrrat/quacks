mod game;

use std::process;
use game::BasicPlayer;

fn main() {
    println!("Quacks of Quedlinburg - Sim");

    let game = game::Game {
        name: "Quacks".to_string(),
        player: Box::new(BasicPlayer {}),
    };

    if let Err(err) = game.run() {
        println!("{}", err);
        process::exit(1);
    }
}
