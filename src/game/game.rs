
use std::error::Error;

use super::Board;
use super::Chip;

#[derive(Debug)]
pub struct Game {
    pub name: String,
}

impl Game {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {

        let mut board = Board::new();

        let mut bag = vec![
            Chip::new("white", 1),
            Chip::new("white", 1),
            Chip::new("white", 1),
            Chip::new("white", 1),
            Chip::new("white", 2),
            Chip::new("white", 2),
            Chip::new("white", 3),
            Chip::new("orange", 1),
            Chip::new("green", 1),
        ];

        let mut points = 0;

        for i in 0..9 {
             board.reset(&mut bag);

            while let Some(chip) = bag.pop() {
                board.play(chip);

                if board.cherry_count >= 7 {
                    break;
                }
            }

            let mut r_points = 0;
            let mut money = 0;
            if board.cherry_count > 7 {
                if i < 3 {
                    money = board.spots[board.current_spot+1].money;
                } else {
                    r_points = board.spots[board.current_spot + 1].points;
                }
            } else {
                money = board.spots[board.current_spot+1].money;
                r_points = board.spots[board.current_spot + 1].points;
            }

            if i == 8 && money > 0 {
                r_points += money / 5;
            }
            
            points += r_points;
            println!("{} Remaing chips: {:?}", i, bag);
            println!("{} Board:\n{}", i, board);
            println!("{} cherry count is: {} {}", i, board.cherry_count, if board.cherry_count > 7 {"and you exploded!"} else {"and you are safe!"});
            println!("{} Points this round: {}", i, r_points);
            println!("{} Total points is: {}", i, points);

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

            match money {
                0..=2   => {},
                3       => { 
                    bag.push(Chip::new("orange", 1));
                },
                4..=6   => { 
                    bag.push(Chip::new("red", 1));
                },
                7       => { 
                    bag.push(Chip::new("orange", 1));
                    bag.push(Chip::new("red", 1));
                },
                8..=10  => {
                     bag.push(Chip::new("red", 2)) 
                },
                11..=13 => { 
                    bag.push(Chip::new("orange", 1));
                    bag.push(Chip::new("red", 2));
                },
                14..=16  => {
                    bag.push(Chip::new("red", 4));
                },
                17..    => {
                    bag.push(Chip::new("orange", 1));
                    bag.push(Chip::new("red", 4));
                },
                _       => {},
            }
        }       

        Ok(())
    }
}