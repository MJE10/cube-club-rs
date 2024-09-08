#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[allow(dead_code)]
pub enum Puzzle {
    Three,
    Two,
    Four,
    Five,
    Six,
    Seven,
    Pyraminx,
    Square1,
    Megaminx,
    Clock,
    Skewb,
}

#[allow(dead_code)]
impl Puzzle {
    pub fn get_all() -> Vec<Puzzle> {
        vec![
            Puzzle::Three,
            Puzzle::Two,
            Puzzle::Four,
            Puzzle::Five,
            Puzzle::Six,
            Puzzle::Seven,
            Puzzle::Pyraminx,
            Puzzle::Square1,
            Puzzle::Megaminx,
            Puzzle::Clock,
            Puzzle::Skewb,
        ]
    }

    pub fn id(&self) -> i64 {
        match self {
            Puzzle::Three => 1,
            Puzzle::Two => 2,
            Puzzle::Four => 3,
            Puzzle::Five => 4,
            Puzzle::Six => 5,
            Puzzle::Seven => 6,
            Puzzle::Pyraminx => 7,
            Puzzle::Square1 => 8,
            Puzzle::Megaminx => 9,
            Puzzle::Clock => 10,
            Puzzle::Skewb => 11,
        }
    }

    pub fn from_id(id: i64) -> Option<Self> {
        match id {
            1 => Some(Puzzle::Three),
            2 => Some(Puzzle::Two),
            3 => Some(Puzzle::Four),
            4 => Some(Puzzle::Five),
            5 => Some(Puzzle::Six),
            6 => Some(Puzzle::Seven),
            7 => Some(Puzzle::Pyraminx),
            8 => Some(Puzzle::Square1),
            9 => Some(Puzzle::Megaminx),
            10 => Some(Puzzle::Clock),
            11 => Some(Puzzle::Skewb),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Puzzle::Three => "3x3",
            Puzzle::Two => "2x2",
            Puzzle::Four => "4x4",
            Puzzle::Five => "8x8",
            Puzzle::Six => "16x16",
            Puzzle::Seven => "32x32",
            Puzzle::Pyraminx => "Pyraminx",
            Puzzle::Square1 => "Square 1",
            Puzzle::Megaminx => "Megaminx",
            Puzzle::Clock => "Clock",
            Puzzle::Skewb => "Skewb",
        }
    }

    pub fn tnoodle_name(&self) -> &'static str {
        match self {
            Puzzle::Three => "three",
            Puzzle::Two => "two",
            Puzzle::Four => "four",
            Puzzle::Five => "five",
            Puzzle::Six => "six",
            Puzzle::Seven => "seven",
            Puzzle::Pyraminx => "pyra",
            Puzzle::Square1 => "sq1",
            Puzzle::Megaminx => "mega",
            Puzzle::Clock => "clock",
            Puzzle::Skewb => "skewb",
        }
    }
}
