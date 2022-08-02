
use std::error::Error;

use rand::Rng;

use crate::game::ExplosionDecision;
use super::Board;
use super::Chip;
use super::Player;
use super::chip::chip;

pub struct Game {
    pub name: String,
    pub player: Box<dyn Player>,
}

impl Game {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {

        let mut board = Board::new();

        let mut bag = vec![
            chip!{1-white},
            chip!{1-white},
            chip!{1-white},
            chip!{1-white},
            chip!{2-white},
            chip!{2-white},
            chip!{3-white},
            chip!{1-orange},
            chip!{1-green},
        ];

        let mut flask = true;

        let mut total_points = 0;
        let mut total_rubies = 1;

        for i in 1..=9 {

            // round 6 we add a white chip
            if i == 6 {
                bag.push(chip!{1-white});
            }

            board.reset(&mut bag);

            // ✔ player decides if they want to draw a chip
            while self.player.should_draw_chip(i, &board, bag.len() as i32) {

                let chip = match bag.pop() {
                    None => break,
                    Some(chip) => chip,
                };

                board.play(chip);
                
                if board.has_exploded() {
                    break;
                }
                
                if board.is_full() {
                    break;
                }

                // check if chip is white
                if let Some(chip) = board.last_chip().filter(|c| c.color == "white") {
                    // ✔ player decides if they want to use the flask
                    if flask && self.player.should_use_flask(i, &board, chip, total_rubies) {
                        flask = false;
                        board.pop_chip_to_bag(&mut bag);        
                    }
                }
            }
            
            let mut points = 0;
            let mut money = 0;            
            if board.has_exploded() {
                // ✔ player decides money or points if they explode
                match self.player.money_or_points(i) {
                    ExplosionDecision::Money => { money = board.score_money(); }
                    ExplosionDecision::Points => { points = board.score_points(); }
                }
            } else {
                // Roll the die
                match rand::thread_rng().gen_range(0..6) {
                    //doplet
                    0 => {},
                    1 => total_rubies += 1,
                    2 => bag.push(chip!{1-orange}),
                    3 => points +=2,
                    4.. => points += 1,
                    _ => {}
                }

                money = board.score_money();
                points += board.score_points();
            }

            /*
                Add Chip action phase
                - Green
                - Purple
                - Black
            */

            total_rubies += board.score_ruby();

            // Last round convert money & rubies into points
            if i == 9 {
                if money > 0 {
                    points += money / 5;
                }
                
                if total_rubies > 0 {
                    points += total_rubies / 2;
                }
            } else {
                // ✔ player decides what chips to buy
                match self.player.buy_chips(i, money) {
                    (None, None) => {},
                    (None, Some(chip)) => bag.push(chip),
                    (Some(chip), None) => bag.push(chip),
                    (Some(chip1), Some(chip2)) => {
                        bag.push(chip1);
                        bag.push(chip2);
                    },
                }

                // ✔ player decides if they refill their flask
                if total_rubies >=2 && !flask && self.player.should_refill_flask(i, total_rubies) {
                    flask = true;
                    total_rubies -= 2;
                }

                // ◻ player decides if they buy a droplet space
            }
            
            total_points += points;
            println!("{} Remaing chips: {:?}", i, bag);
            println!("{} Board:\n{}", i, board);
            println!("{} cherry count is: {} {}", i, board.cherry_count, if board.cherry_count > 7 {"and you exploded!"} else {"and you are safe!"});
            println!("{} Money this round: {}", i, money);
            println!("{} Points this round: {}", i, points);
            println!("{} Total rubies: {}", i, total_rubies);
            println!("{} Total points is: {}", i, total_points);
        }       

        Ok(())
    }
}