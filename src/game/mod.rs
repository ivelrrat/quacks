mod player;
mod chip;
mod spot;



use std::{fs::File, process};

pub use player::Player;
pub use chip::Chip;
pub use spot::Spot;

#[derive(Debug)]
pub struct Game {
    pub name: String,
}

pub fn load_board() -> Vec<Spot> {
    let mut reader = csv::Reader::from_reader(File::open("src/game/board.csv").expect("A CSV file"));

    let mut board = Vec::new();
    
    for result in reader.records() {
        
        match result {
            Ok(record) => board.push(Spot::new(
                record[0].parse::<i32>().expect("An integer money value"),
                record[1].parse::<i32>().expect("An integer point value"),
                &record[2] == "1")),
            Err(err) => {
                println!("Error reading CSV from board.csv: {}", err);
                process::exit(1);
            }
        }
    }

    return board;
}