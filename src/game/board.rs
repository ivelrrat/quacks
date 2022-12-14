use super::Chip;
use super::Spot;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Board {
    pub spots: Vec<Spot>,
    pub current_spot: usize,
    pub droplet: usize,
    pub cherry_count: i32,
    pub orange_count: i32,
}

impl Board {
    pub fn play(&mut self, chip: Chip) {
       
        let mut value = chip.size;

        match chip.color.as_str() {
            "white" => {
                self.cherry_count += chip.size;
            },
            "red" => {
                /* 
                    Rule book 1: 
                    If there 1 or 2 orange chips in your pot, move the red chip an additional 1 space forward.
                    If there are 3 or more orange chips in your pot, move it an additional 2 spaces forward.
                */
                match self.orange_count {
                    1 | 2 => value +=1,
                    3.. => value +=2,
                    _ => ()
                };
            },
            "orange" => {
                self.orange_count += 1;
            },
            _ => (),
        }

        self.current_spot += value as usize;
        if self.current_spot >= self.spots.len() - 2 {
            self.current_spot = self.spots.len() - 2
        }

        self.spots[self.current_spot].chip = Some(chip);
    }

    pub fn new() -> Self {
        Self {
            spots: load_board(),
            current_spot: 0,
            droplet: 0,
            cherry_count: 0,
            orange_count: 0,
        }
    }

    pub fn reset(&mut self, bag: &mut Vec<Chip>) {
        for spot in &mut self.spots {
            if let Some(chip) = spot.chip.take().filter(|c| c.is_player_chip()) {
                bag.push(chip);
            }
        }

        self.orange_count = 0;
        self.cherry_count = 0;
        self.current_spot = 0;
        self.play(Chip::new("droplet", self.droplet as i32));
        bag.shuffle(&mut thread_rng());
    }

    pub fn score_money(&self) -> i32 {
        self.spots[self.current_spot+1].money
    }

    pub fn score_points(&self) -> i32 {
        self.spots[self.current_spot + 1].points
    }

    pub fn score_ruby(&self) -> i32 {
        if self.spots[self.current_spot + 1].ruby {1} else {0}
    }

    pub fn has_exploded(&self) -> bool {
        self.cherry_count > 7
    }
    
    pub fn is_full(&self) -> bool {
        self.current_spot >= self.spots.len() - 2
    }

    pub fn pop_chip_to_bag(&mut self, bag: &mut Vec<Chip>) {
        if let Some(chip) = self.spots[self.current_spot].chip.take() {
            self.current_spot -= chip.size as usize;
            bag.push(chip);
            bag.shuffle(&mut thread_rng());
        }
    }

    pub fn last_chip(&self) -> Option<&Chip> {
        self.spots[self.current_spot].chip.as_ref()
    }

    pub fn last_two_chips(&self) -> [Option<&Chip>; 2] {
        let mut second = self.current_spot - 1;
        while self.spots[second].chip.is_none() {
            second -= 1;

            if second == 0 {
                break;
            }
        }

        [self.spots[self.current_spot].chip.as_ref(), self.spots[second].chip.as_ref()]
    }

    pub fn all_white_chips(&self) -> Vec<&Chip> {

        let mut white_chips = Vec::<&Chip>::new();
        for spot in &self.spots {
            if let Some(chip) = spot.chip.as_ref().filter(|c| c.color == "white") {
                white_chips.push(chip);
            }
        }

        return white_chips;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        self.spots[0..=(self.current_spot + 1)].iter().enumerate().for_each(|(i, spot)| {
            writeln!(f, "{}: {}", i, spot).expect("Failed to write message");
        });
        Ok(())
    }
}

fn load_board() -> Vec<Spot> {
    return vec![
        Spot::new(0,0,false),
        Spot::new(1,0,false),
        Spot::new(2,0,false),
        Spot::new(3,0,false),
        Spot::new(4,0,false),
        Spot::new(5,0,true),
        Spot::new(6,1,false),
        Spot::new(7,1,false),
        Spot::new(8,1,false),
        Spot::new(9,1,true),
        Spot::new(10,2,false),
        Spot::new(11,2,false),
        Spot::new(12,2,false),
        Spot::new(13,2,true),
        Spot::new(14,3,false),
        Spot::new(15,3,false),
        Spot::new(15,3,true),
        Spot::new(16,3,false),
        Spot::new(16,4,false),
        Spot::new(17,4,false),
        Spot::new(17,4,true),
        Spot::new(18,4,false),
        Spot::new(18,5,false),
        Spot::new(19,5,false),
        Spot::new(19,5,true),
        Spot::new(20,5,false),
        Spot::new(20,6,false),
        Spot::new(21,6,false),
        Spot::new(21,6,true),
        Spot::new(22,7,false),
        Spot::new(22,7,true),
        Spot::new(23,7,false),
        Spot::new(23,8,false),
        Spot::new(24,8,false),
        Spot::new(24,8,true),
        Spot::new(25,9,false),
        Spot::new(25,9,true),
        Spot::new(26,9,false),
        Spot::new(26,10,false),
        Spot::new(27,10,false),
        Spot::new(27,10,true),
        Spot::new(28,11,false),
        Spot::new(28,11,true),
        Spot::new(29,11,false),
        Spot::new(29,12,false),
        Spot::new(30,12,false),
        Spot::new(30,12,true),
        Spot::new(31,12,false),
        Spot::new(31,13,false),
        Spot::new(32,13,false),
        Spot::new(32,13,true),
        Spot::new(33,14,false),
        Spot::new(33,14,true),
        Spot::new(35,15,false),
    ];
}
