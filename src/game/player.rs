use rand::Rng;
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

    pub fn should_draw_chip(&self, round: i32) -> bool {
        self.skill.should_draw_chip(round, &self.board, self.bag.len() as i32)
    }

    pub fn handle_buy_chips_decision(&mut self, round: i32) {
        if let Some(mut chips) = self.skill.buy_chips(round, self.money) {
            self.bag.append(&mut chips);
        }
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

    pub fn handle_buy_droplet_decision(&mut self, round:i32) {
        while self.rubies >=2 && self.skill.should_buy_droplet(round, self.rubies) {
            self.rubies -= 2;
            self.board.droplet += 1;
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
}