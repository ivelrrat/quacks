use std::error::Error;
use super::{Chip, chip::chip, Player, LAST_ROUND};

pub struct Game {
    pub name: String,
    pub players: Vec<Player>,
}

impl Game {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        
        for i in 1..=LAST_ROUND {            
            for player in &mut self.players {
                
                // An additional white chip is added in round 6
                if i == 6 {
                    player.bag.push(chip!{1-white});
                }

                player.reset();
            }
            
            let mut players: Vec<&mut Player> = self.players.iter_mut().collect();
            while !players.is_empty() {
                players.retain_mut(|player| {
                    if !player.should_draw_chip(i) {
                        return false;
                    }

                    let chip = match player.bag.pop() {
                        None => {
                            return false;
                        },
                        Some(chip) => chip,
                    };
    
                    player.play(chip);
                    if player.is_done() {
                        return false;
                    }

                    player.handle_flask_descision(i);

                    return true;
                });
            }

            for player in &mut self.players {

                //TODO: check for the player in the futhest spot to roll die.
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
    ✔ Green
        In Evaluation Phase B, you receive 1 ruby for every green chip that was either the last chip or next to last.

    ✔ Purple
         In Evaluation Phase B, count up the purple chips in your pot. If there is 1 purple chip, you receive 1 victory point.
         If there are 2 purple chips, you receive 1 victory point and 1 ruby.
         If there are 3 or more purple chips, you receive 2 victory points and you may move your droplet 1 space forward.
         There is no added bonus for 4 or more chips. However, it is always possible to use a lower action. For example, you
         can take the bonus for 2 purple chips even though you have 3 chips.

    ◻ Black
*/
fn chip_actions(player: &mut Player) {

    /*
        GREEN ACTION
    */
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

    /*
        PURPLE ACTION
    */
    let purple_count: i32 = player.board
    .spots
    .iter()
    .map(|spot| 
        match spot.chip.as_ref() {
            Some(c) => if c.color == "purple" {1} else {0},
            None => 0,
        }                        
    ).sum();

    match purple_count {
        1 => player.points += 1,
        2 => {
            player.points += 1;
            player.rubies += 1;
        },
        3.. => {
            player.points += 2;
            player.board.droplet += 1;
        },
        _ => ()
    }
}