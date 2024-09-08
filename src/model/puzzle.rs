#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[allow(dead_code)]
pub enum Puzzle {
    Three,
    Two,
}

#[allow(dead_code)]
impl Puzzle {
    pub fn get_all() -> Vec<Puzzle> {
        vec![Puzzle::Three, Puzzle::Two]
    }

    pub fn id(&self) -> i64 {
        match self {
            Puzzle::Three => 1,
            Puzzle::Two => 2,
        }
    }

    pub fn from_id(id: i64) -> Option<Self> {
        match id {
            1 => Some(Puzzle::Three),
            2 => Some(Puzzle::Two),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Puzzle::Three => "3x3",
            Puzzle::Two => "2x2",
        }
    }

    pub fn tnoodle_name(&self) -> &'static str {
        match self {
            Puzzle::Three => "three",
            Puzzle::Two => "two",
        }
    }
}
