mod game;

use game::Player;
use game::Chip;
use game::Spot;
use rand::prelude::SliceRandom;
use rand::thread_rng;

fn main() {

    let game = game::Game {
        name: "Quacks".to_string(),
    };

    let player = Player {
        name: "Levi".to_string(),
    };

    println!("{:?} {:?}", game, player);

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

    let mut board = game::load_board();

    println!("Quacks of Quedlinburg - Sim");

    bag.shuffle(&mut thread_rng());

    let mut current: usize = 0;
    let mut cherry_count = 0;
    while let Some(chip) = bag.pop() {
        println!("chip: {}", chip);

        current += chip.size as usize;

        // board.get(current).chip = Some(chip);

        if chip.color == "white" {
            cherry_count += chip.size;
        }

        board[current].chip = Some(chip);

        if cherry_count >= 7 {
            break;
        }
    }

    println!("Remaing chips: {:?}", bag);
    // println!("Board: {:?}", board);

    board.iter().enumerate().for_each(|(i, spot)| {
        let rub = if i == current && spot.ruby == true {"RUBY"} else {""};
        if i <= current {
            println!("{}: {} {} {}",
                i, 
                spot,
                match &spot.chip {
                    Some(chip) => {format!("- {}", &chip)},
                    None => {"".to_string()}
                },
                rub
            );
        }
    });

    println!("cherry count is: {} {}", cherry_count, if cherry_count > 7 {"and you exploded!"} else {"and you are safe!"} )


}
