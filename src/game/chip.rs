macro_rules! chip {
    ($value:tt-$color:tt) => {
        Chip::new(stringify!($color), $value)
    };
}

pub(crate) use chip;

#[derive(Debug)]
pub struct Chip {
    pub color: String,
    pub size: i32,
}

impl Chip {
    pub fn new(color: &str, size: i32) -> Self {
        Self {
            color: color.to_string(),
            size,
        }
    }

    /*
        Current non-player chips are "droplet" and "was white" and eventually "rat tails".
    */
    pub fn is_player_chip(&self) -> bool {
        static COLORS: [&str; 8] = ["white", "orange", "blue", "red", "yellow", "black", "green", "purple"];
        COLORS.contains(&self.color.as_str())
    }
}

impl std::fmt::Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.color, self.size)
    }
}
