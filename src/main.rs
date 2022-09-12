mod game;

use std::{process, env};
use game::{Player, Game};
// use game::BasicPlayer;
// use game::BuysBlue;
// use game::BuysYellow;
use game::BuysPurple;
// use game::BuysGreen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_games = args.get(1).map_or(1000, |arg| arg.parse().expect("Enter an integer for max games"));
        
    println!("Quacks of Quedlinburg - Sim");

    let mut points = 0;
    for _i in 0..max_games {
        let mut game = Game {
            name: "Quacks".to_string(),
            // player: Player::new(BasicPlayer {}),
            player: Player::new(BuysPurple {}),
            // player: Player::new(BuysBlue {}),
            // player: Player::new(BuysYellow {}),
            // player: Player::new(BuysGreen {}),
        };
    
        if let Err(err) = game.run() {
            println!("{}", err);
            process::exit(1);
        }
    
        points += game.player.points;
        // println!("{}", game.player.points);
    }

    println!("AVG: {points}/{max_games}={}", points as f64 /max_games as f64);
}
