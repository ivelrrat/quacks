// mod game;

use super::Chip;

#[derive(Debug)]
pub struct Spot {
    pub money: i32,
    pub points: i32,
    pub ruby: bool,
    pub chip: Option<Chip>,
}

impl Spot {
    pub fn new(money: i32, points: i32, ruby: bool) -> Self {
        Self {
            money,
            points,
            ruby,
            chip: None
        }
    }
}

impl std::fmt::Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chip = self.chip
                                .as_ref()
                                .map_or("".to_string(), |c| format!("- {}", c));

        write!(f, "({})[{}]{} {chip}", self.money, self.points, if self.ruby {"*"} else {" "})
    }
}
