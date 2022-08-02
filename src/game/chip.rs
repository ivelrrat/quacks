

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
}

impl std::fmt::Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.color, self.size)
    }
}
