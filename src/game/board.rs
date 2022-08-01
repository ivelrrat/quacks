use super::Chip;
use super::Spot;
use std::{fs::File, process};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Board {
    pub spots: Vec<Spot>,
    pub current_spot: usize,
    pub cherry_count: i32,
    pub orange_count: i32,
}

impl Board {
    pub fn play(&mut self, chip: Chip) {

        if self.current_spot >= self.spots.len() - 2 {
            return;
        }
        
        let mut value = chip.size;

        match chip.color.as_str() {
            "white" => {
                self.cherry_count += chip.size;
            },
            "red" => {
                if self.orange_count >= 3 {
                    value +=2;
                } else if self.orange_count > 0 {
                    value +=1;
                }
            },
            "orange" => {
                self.orange_count += 1;
            }
            _ => {}
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
            cherry_count: 0,
            orange_count: 0,
        }
    }

    pub fn reset(&mut self, bag: &mut Vec<Chip>) {
        for spot in &mut self.spots {
            if let Some(chip) = std::mem::replace(&mut spot.chip, None) {
                bag.push(chip);
            }
        }

        self.orange_count = 0;
        self.cherry_count = 0;
        self.current_spot = 0;
        bag.shuffle(&mut thread_rng());
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        self.spots.iter().enumerate().for_each(|(i, spot)| {
            if i <= self.current_spot + 1 {
                writeln!(
                    f,
                    "{}: {} {}",
                    i,
                    spot,
                    match &spot.chip {
                        Some(chip) => {
                            format!("- {}", &chip)
                        }
                        None => {
                            "".to_string()
                        }
                    }
                ).expect("Failed to write message");
            }
        });
        Ok(())
    }
}

pub fn load_board() -> Vec<Spot> {
    let mut reader =
        csv::Reader::from_reader(File::open("src/game/board.csv").expect("A CSV file"));

    let mut board = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => board.push(Spot::new(
                record[0].parse::<i32>().expect("An integer money value"),
                record[1].parse::<i32>().expect("An integer point value"),
                &record[2] == "1",
            )),
            Err(err) => {
                println!("Error reading CSV from board.csv: {}", err);
                process::exit(1);
            }
        }
    }

    return board;
}
