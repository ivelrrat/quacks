use rand::Rng;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use super::{Board, Chip, chip::chip, PlayerSkill, ExplosionDecision, LAST_ROUND};

pub struct Player {
    pub board: Board,
    pub bag: Vec<Chip>,
    pub flask: bool,
    pub points: i32,
    pub rubies: i32,
    pub skill: Box<dyn PlayerSkill>,
    pub money: i32,
}

impl Player {
    pub fn new(skill: impl PlayerSkill +'static) -> Self 
    {
        Self { 
            board: Board::new(),
            bag:  vec![
                    chip!{1-white},
                    chip!{1-white},
                    chip!{1-white},
                    chip!{1-white},
                    chip!{2-white},
                    chip!{2-white},
                    chip!{3-white},
                    chip!{1-green},
                    chip!{1-orange},
                ],
            flask: false,
            points: 0,
            rubies: 1,
            skill: Box::new(skill),
            money: 0,
        }
    }

    pub fn play(&mut self, chip: Chip) {

        match chip.color.as_str() {
            "yellow" => {
                /*
                    Rule book 1: 
                     If you draw a yellow chip from your bag directly after a white chip, you may put the white
                     chip (regard- less of its value) back into the bag. This applies only if the white chip was drawn
                     directly before the yellow chip.
                */
                
                if let Some(white_chip) = self.board.last_chip().filter(|c| c.color == "white") {
                    let size = white_chip.size;

                    // Technially this is a decsion and should be available in the player_skill trait, but why would you not want to get rid of white?
                    self.board.pop_chip_to_bag(&mut self.bag);
                    self.board.play(Chip::new("was white", size));
                }

                self.board.play(chip);
            },
            "blue" => {
                /*
                    Rule book 1: 
                    1-blue = 1 chip, 2-blue = 2 chips, 4-blue = 4 chips
                    From the chips drawn, you may lay down 1 of them as your next chip. Put the other chips back into
                    the bag. If you do not like what you see, you may put them all back into the bag. If the newly laid
                    chip also has a bonus, it can also be carried out immediately.
                 */

                let size = chip.size as usize;
                self.board.play(chip);
                if self.is_done() {
                    return;
                }

                let base = if self.bag.len() < size {0} else {self.bag.len() - size};

                if let Some(choice) = self.skill.choose_one(&self.bag[base..]) {
                    let bonus_chip = self.bag.remove(base + choice);
                    self.play(bonus_chip);
                }

                self.bag.shuffle(&mut thread_rng());
            },
            _ => {
                self.board.play(chip);
            },
        }
    }

    pub fn should_draw_chip(&self, round: i32) -> bool {
        self.skill.should_draw_chip(round, &self.board, self.bag.len() as i32)
    }

    pub fn handle_flask_descision(&mut self, round: i32) {
        if !self.flask {
            return;
        }

        if let Some(chip) = self.board.last_chip().filter(|c| c.color == "white") {
            if self.skill.should_use_flask(round, &self.board, chip, self.rubies) {
                self.flask = false;
                self.board.pop_chip_to_bag(&mut self.bag);        
            }
        }
    }

    pub fn is_done(&self) -> bool {
        if self.board.has_exploded() {
            return true;
        }
        
        if self.board.is_full() {
            return true;
        }

        return false;
    }

    pub fn reset(&mut self) {
        self.board.reset(&mut self.bag);
        self.money = 0;
    }

     //Handle evaluation phase A (die roll)
     pub fn roll_die(&mut self)  {
        match rand::thread_rng().gen_range(0..6) {
            0 => self.board.droplet += 1,
            1 => self.rubies += 1,
            2 => self.bag.push(chip!{1-orange}),
            3 => self.points +=2,
            4 | 5 => self.points += 1,
            _ => {}
        }
    }

    //Handle evaluation phase C (rubies), D (Victory Points), part of E (money for buying chips)
    pub fn score(&mut self, round: i32) {
        self.rubies += self.board.score_ruby();

        if self.board.has_exploded() {
            // ✔ player decides money or points if they explode
            match self.skill.money_or_points(round) {
                ExplosionDecision::Money => { self.money = self.board.score_money(); }
                ExplosionDecision::Points => { self.points += self.board.score_points(); }
            }
        } else {
            self.money = self.board.score_money();
            self.points += self.board.score_points();
        }

        // Last round convert money & rubies into points
        if round == LAST_ROUND {
            self.points += self.money / 5;
            self.points += self.rubies / 2;
        }
    }

    //Handle evaluation phase E (buy chips)
    pub fn handle_buy_chips_decision(&mut self, round: i32) {
        if let Some(mut chips) = self.skill.buy_chips(round, self.money) {
            self.bag.append(&mut chips);
        }
    }

    //Handle evaluation phase F (buy flask)
    pub fn handle_refill_flask_decision(&mut self, round:i32) {
        if self.flask {
            return;
        }

        if self.rubies < 2 {
            return;
        }

        if self.skill.should_refill_flask(round, self.rubies) {
            self.flask = true;
            self.rubies -= 2;
        }
    }

    //Handle evaluation phase F (buy droplet)
    pub fn handle_buy_droplet_decision(&mut self, round:i32) {
        while self.rubies >=2 && self.skill.should_buy_droplet(round, self.rubies) {
            self.rubies -= 2;
            self.board.droplet += 1;
        }
    }

}