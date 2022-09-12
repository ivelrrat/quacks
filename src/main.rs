mod game;

use std::{process, env};
use game::{Player, Game};
use game::BasicPlayer;
// use game::BuysBlue;
// use game::BuysYellow;
// use game::BuysPurple;
// use game::BuysGreen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_games = args.get(1).map_or(1000, |arg| arg.parse().expect("Enter an integer for max games"));
        
    println!("Quacks of Quedlinburg - Sim");

    let mut points = [0,0,0,0];
    for _i in 0..max_games {
        let mut game = Game {
            name: "Quacks".to_string(),
            players: vec![
                Player::new(BasicPlayer {}),
                Player::new(BasicPlayer {}),
                Player::new(BasicPlayer {}),
                Player::new(BasicPlayer {}),
                // Player::new(BuysGreen {}),
                // Player::new(BuysYellow {}),
                // Player::new(BuysBlue {}),
                // Player::new(BuysPurple {}),
            ],
        };
    
        if let Err(err) = game.run() {
            println!("{}", err);
            process::exit(1);
        }
    
        for (i, player) in game.players.iter().enumerate() {
            points[i] += player.points;
        }
    }

    for point in points {
        println!("AVG: {point}/{max_games}={}", point as f64 /max_games as f64);
    }
}
