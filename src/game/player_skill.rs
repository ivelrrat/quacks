use super::{Chip, Board, ExplosionDecision, chip::chip};

pub trait PlayerSkill {
    fn should_draw_chip(&self, round: i32, board: &Board, bag_count: i32) -> bool;
    fn money_or_points(&self, round: i32) -> ExplosionDecision;
    fn buy_chips(&self, round: i32, money: i32) -> Option<Vec<Chip>>;
    fn should_use_flask(&self, round: i32, board: &Board, chip: &Chip, rubies: i32) -> bool;
    fn should_refill_flask(&self, round:i32, rubies: i32) -> bool;
    fn should_buy_droplet(&self, round:i32, rubies: i32) -> bool;
}

pub struct BasicPlayer;

impl PlayerSkill for BasicPlayer {
    fn should_draw_chip(&self, _round: i32, board: &Board, _bag_count: i32) -> bool {
        board.cherry_count < 7
    }

    fn money_or_points(&self, round: i32) -> ExplosionDecision {
        if round > 6 {
            return ExplosionDecision::Points;
        } 

        ExplosionDecision::Money
    }

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