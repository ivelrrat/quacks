use super::{Chip, Board, ExplosionDecision, chip::chip};

pub trait Player {
    fn should_draw_chip(&self, round: i32, board: &Board, bag_count: i32) -> bool;
    fn money_or_points(&self, round: i32) -> ExplosionDecision;
    fn buy_chips(&self, round: i32, money: i32) -> (Option<Chip>, Option<Chip>);
    fn should_use_flask(&self, round: i32, board: &Board, chip: &Chip, rubies: i32) -> bool;
    fn should_refill_flask(&self, round:i32, rubies: i32) -> bool;
}

pub struct BasicPlayer;

impl Player for BasicPlayer {
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
    fn buy_chips(&self, _round: i32, money: i32) -> (Option<Chip>, Option<Chip>) {
        match money {
            3       => { (Some(chip!{1-orange}), None) },
            4..=6   => { (Some(chip!(1-red)), None) },
            7       => { (Some(chip!(1-orange)), Some(chip!(1-red))) },
            8..=10  => { (Some(chip!(2-red)), None) },
            11..=13 => { (Some(chip!(1-orange)), Some(chip!(2-red))) },
            14..=16 => { (Some(chip!(4-red)), None) },
            17..    => { (Some(chip!(1-orange)), Some(chip!(4-red))) },
            _       => { (None, None) },
        }
    }

    fn should_use_flask(&self, _round: i32, _board: &Board, _chip: &Chip, rubies: i32) -> bool {
        rubies >= 2
    }

    fn should_refill_flask(&self, _round:i32, rubies: i32) -> bool {
        rubies >= 2
    }
}