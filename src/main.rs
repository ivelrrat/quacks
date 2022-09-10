mod game;

use std::process;
use game::{Player, Game};
// use game::BasicPlayer;
use game::BuysBlue;
// use game::BuysGreen;

fn main() {
    println!("Quacks of Quedlinburg - Sim");

    const MAX_GAMES: i32 = 1000;
    let mut points = 0;
    for _i in 0..MAX_GAMES {
        let mut game = Game {
            name: "Quacks".to_string(),
            // player: Player::new(BasicPlayer {}),
            player: Player::new(BuysBlue {}),
            // player: Player::new(BuysGreen {}),
        };
    
        if let Err(err) = game.run() {
            println!("{}", err);
            process::exit(1);
        }
    
        points += game.player.points;
        // println!("{}", game.player.points);
    }

    println!("AVG: {points}/{MAX_GAMES}={}", points as f64 /MAX_GAMES as f64);
}
