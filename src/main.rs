mod game;

use std::process;

fn main() {
    println!("Quacks of Quedlinburg - Sim");

    let game = game::Game {
        name: "Quacks".to_string(),
    };

    if let Err(err) = game.run() {
        println!("{}", err);
        process::exit(1);
    }
}
