use std::error::Error;
use super::{Chip, chip::chip, Player, LAST_ROUND};

pub struct Game {
    pub name: String,
    pub player: Player,
}

impl Game {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {

        for i in 1..=LAST_ROUND {
            let player = &mut self.player;

            // An additional white chip is added in round 6
            if i == 6 {
                player.bag.push(chip!{1-white});
            }

            player.reset();
            while player.should_draw_chip(i) {
                let chip = match player.bag.pop() {
                    None => break,
                    Some(chip) => chip,
                };

                player.board.play(chip);
                if player.is_done() {
                    break;
                }
                player.handle_flask_descision(i);
            }
            
            if !player.board.has_exploded() {
                player.roll_die();
            }

            chip_actions(player);

            player.score(i);

            if i < LAST_ROUND {
                player.handle_buy_chips_decision(i);
                player.handle_refill_flask_decision(i);
                player.handle_buy_droplet_decision(i);
            }
            
            // println!("\n\nRESULTS - Round {}\n\n", i);
            // println!("{} Remaing chips: {:?}", i, player.bag);
            // println!("{} Board:\n{}", i, player.board);
            // println!("{} cherry count is: {} {}", i, player.board.cherry_count, if player.board.cherry_count > 7 {"and you exploded!"} else {"and you are safe!"});
            // println!("{} Money this round: {}", i, player.money);
            // // println!("{} Points this round: {}", i, points);
            // println!("{} Total rubies: {}", i, player.rubies);
            // println!("{} Total points is: {}", i, player.points);
            // println!("{} Droplet: {}", i, player.board.droplet);
        }       

        Ok(())
    }
}

/*
    B. Chip Actions
    ◻ Green
        In Evaluation Phase B, you receive 1 ruby for every green chip that was either the last chip or next to last.
    ◻ Purple
    ◻ Black
*/
fn chip_actions(player: &mut Player) {
    let green_count: i32 = player.board
        .last_two_chips()
        .iter()
        .map(|item| 
            match item {
                Some(c) => if c.color == "green" {1} else {0},
                None => 0,
            }                        
        ).sum();


    player.rubies += green_count;
}