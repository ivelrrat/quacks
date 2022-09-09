use super::{Chip, Board, ExplosionDecision, chip::chip, LAST_ROUND};

pub trait BaseSkills {
    fn should_draw_chip(&self, round: i32, board: &Board, bag_count: i32) -> bool {
        if board.cherry_count < 5 {
            return true;
        }

        let mut played = [0,0,0];
        for chip in board.all_white_chips() {
            match chip.size {
                1 => played[0] += 1,
                2 => played[1] += 1,
                3 => played[2] += 1,
                _ => (),
            }
        }

        let unplayed = [if round < 6 {4} else {5}-played[0], 2-played[1], 1-played[2]];
        let exploders: i32;

        match board.cherry_count {
            5 => exploders = unplayed[2],
            6 => exploders = unplayed[1..=2].iter().sum(),
            7 => exploders = unplayed.iter().sum(),
            _ => exploders = 0,
        }   
        
        let perc = exploders as f64 / bag_count as f64;

        // println!("{round} {:?} {:?} {:?} {exploders} {perc}", played, unplayed, board.cherry_count);

        perc < 0.25

        // board.cherry_count < 7
    }

    fn money_or_points(&self, round: i32) -> ExplosionDecision {
        if round == LAST_ROUND {
            return ExplosionDecision::Points;
        } 

        ExplosionDecision::Money
    }

    fn should_use_flask(&self, _round: i32, _board: &Board, _chip: &Chip, rubies: i32) -> bool {
        rubies >= 2
    }

    fn should_refill_flask(&self, _round:i32, rubies: i32) -> bool {
        rubies >= 2
    }
    
    fn should_buy_droplet(&self, _round:i32, rubies: i32) -> bool {
        rubies >= 2
    }
}

pub trait BuyChipsSkill {
    fn buy_chips(&self, round: i32, money: i32) -> Option<Vec<Chip>>;
}

pub trait PlayerSkill: BaseSkills + BuyChipsSkill {}


// The BasicPlayer skill buys Red & Orange chips.
// TODO: rename this to "BuysRedChips"
pub struct BasicPlayer;

impl BaseSkills for BasicPlayer {}

impl BuyChipsSkill for BasicPlayer {
    // Orange 1 - 3
            
    // Blue 1 - 5
    // Blue 2 - 10
    // Blue 4 - 19
    
    // Red 1 - 4
    // Red 2 - 8
    // Red 4 - 14

    // Green 1 - 4
    // Green 2 - 8
    // Green 4 - 14
    
    // Black 1 - 10
    
    // Yellow 1 - 8
    // Yellow 2 - 12
    // Yellow 4 - 18
    
    // Purple 1 - 9
    fn buy_chips(&self, _round: i32, money: i32) -> Option<Vec<Chip>> {
        match money {
            3       => { Some(vec![chip!{1-orange}]) },
            4..=6   => { Some(vec![chip!(1-red)]) },
            7       => { Some(vec![chip!(1-orange), chip!(1-red)]) },
            8..=10  => { Some(vec![chip!(2-red)]) },
            11..=13 => { Some(vec![chip!(1-orange), chip!(2-red)]) },
            14..=16 => { Some(vec![chip!(4-red)]) },
            17..    => { Some(vec![chip!(1-orange), chip!(4-red)]) },
            _       => { None },
        }
    }
}

impl PlayerSkill for BasicPlayer {}

pub struct BuysGreen {}

impl BaseSkills for BuysGreen {}

impl BuyChipsSkill for BuysGreen {
    // Green 1 - 4
    // Green 2 - 8
    // Green 4 - 14
    fn buy_chips(&self, _round: i32, money: i32) -> Option<Vec<Chip>> {
        match money {
            3       => { Some(vec![chip!{1-orange}]) },
            4..=6   => { Some(vec![chip!(1-green)]) },
            7       => { Some(vec![chip!(1-orange), chip!(1-green)]) },
            8..=10  => { Some(vec![chip!(2-green)]) },
            11..=13 => { Some(vec![chip!(1-orange), chip!(2-green)]) },
            14..=16 => { Some(vec![chip!(4-green)]) },
            17..    => { Some(vec![chip!(1-orange), chip!(4-green)]) },
            _       => { None },
        }
    }
}

impl PlayerSkill for BuysGreen {}