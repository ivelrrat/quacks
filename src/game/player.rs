use super::{Board, Chip, chip::chip, PlayerSkill};

pub struct Player {
    pub board: Board,
    pub bag: Vec<Chip>,
    pub flask: bool,
    pub total_points: i32,
    pub total_rubies: i32,
    pub skill: Box<dyn PlayerSkill>,
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
            total_points: 0,
            total_rubies: 1,
            skill: Box::new(skill),
        }
    }

    pub fn should_draw_chip(&self, round: i32) -> bool {
        self.skill.should_draw_chip(round, &self.board, self.bag.len() as i32)
    }

    pub fn money_or_points(&self, round: i32) -> super::ExplosionDecision {
        self.skill.money_or_points(round)
    }

    pub fn buy_chips(&self, round: i32, money: i32) -> Option<Vec<Chip>> {
        self.skill.buy_chips(round, money)
    }

    pub fn handle_flask_descision(&mut self, round: i32) {
        if !self.flask {
            return;
        }

        if let Some(chip) = self.board.last_chip().filter(|c| c.color == "white") {
            if self.skill.should_use_flask(round, &self.board, chip, self.total_rubies) {
                self.flask = false;
                self.board.pop_chip_to_bag(&mut self.bag);        
            }
        }
    }

    pub fn should_refill_flask(&self, round:i32) -> bool {
        self.skill.should_refill_flask(round, self.total_rubies)
    }

    pub fn should_buy_droplet(&self, round:i32) -> bool {
        self.skill.should_buy_droplet(round, self.total_rubies)
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
}