use std::error::Error;
use rand::Rng;
use super::{Chip, chip::chip, ExplosionDecision, Player};

pub struct Game {
    pub name: String,
    pub player: Player,
}

impl Game {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {

        for i in 1..=9 {

            let player = &mut self.player;

            // An additional white chip is added in round 6
            if i == 6 {
                player.bag.push(chip!{1-white});
            }

            player.board.reset(&mut player.bag);

            // ✔ player decides if they want to draw a chip
            while player.should_draw_chip(i) {

                let chip = match player.bag.pop() {
                    None => break,
                    Some(chip) => chip,
                };

                player.board.play(chip);
                
                if player.is_done() {
                    break;
                }

                // ✔ player decides if they want to use the flask
                player.handle_flask_descision(i);
            }
            
            let mut points = 0;
            let mut money = 0;            
            if player.board.has_exploded() {
                // ✔ player decides money or points if they explode
                match player.money_or_points(i) {
                    ExplosionDecision::Money => { money = player.board.score_money(); }
                    ExplosionDecision::Points => { points = player.board.score_points(); }
                }
            } else {
                // Roll the die
                match rand::thread_rng().gen_range(0..6) {
                    0 => player.board.droplet += 1,
                    1 => player.total_rubies += 1,
                    2 => player.bag.push(chip!{1-orange}),
                    3 => points +=2,
                    4.. => points += 1,
                    _ => {}
                }

                money = player.board.score_money();
                points += player.board.score_points();
            }

            /*
                Add Chip action phase
                ◻ Green
                ◻ Purple
                ◻ Black
            */

            player.total_rubies += player.board.score_ruby();

            // Last round convert money & rubies into points
            if i == 9 {
                if money > 0 {
                    points += money / 5;
                }
                
                if player.total_rubies > 0 {
                    points += player.total_rubies / 2;
                }
            } else {
                // ✔ player decides what chips to buy
                if let Some(mut chips) = player.buy_chips(i, money) {
                    player.bag.append(&mut chips);
                }              

                // ✔ player decides if they refill their flask
                if player.total_rubies >=2 && !player.flask && player.should_refill_flask(i) {
                    player.flask = true;
                    player.total_rubies -= 2;
                }

                // ✔ player decides if they buy a droplet space
                while player.total_rubies >=2 && player.should_buy_droplet(i) {
                    player.total_rubies -= 2;
                    player.board.droplet += 1;
                }
            }
            
            player.total_points += points;
            println!("\n\nRESULTS - Round {}\n\n", i);
            println!("{} Remaing chips: {:?}", i, player.bag);
            println!("{} Board:\n{}", i, player.board);
            println!("{} cherry count is: {} {}", i, player.board.cherry_count, if player.board.cherry_count > 7 {"and you exploded!"} else {"and you are safe!"});
            println!("{} Money this round: {}", i, money);
            println!("{} Points this round: {}", i, points);
            println!("{} Total rubies: {}", i, player.total_rubies);
            println!("{} Total points is: {}", i, player.total_points);
            println!("{} Droplet: {}", i, player.board.droplet);
        }       

        Ok(())
    }
}