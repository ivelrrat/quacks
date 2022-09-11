mod chip;
mod player;
mod player_skill;
mod spot;
mod game;
mod board;

pub use chip::Chip;
pub use spot::Spot;
pub use game::Game;
pub use board::Board;
pub use player::Player;
pub use player_skill::PlayerSkill;
pub use player_skill::BasicPlayer;
pub use player_skill::BuysGreen;
pub use player_skill::BuysBlue;
pub use player_skill::BuysYellow;

const LAST_ROUND: i32 = 9;

pub enum ExplosionDecision {
    Money,
    Points,
}