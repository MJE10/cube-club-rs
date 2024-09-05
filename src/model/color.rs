use hex_color::HexColor;
use rocket::serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Serialize, Deserialize)]
pub enum RubikColor {
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue,
}

use RubikColor::*;

#[allow(dead_code)]
impl RubikColor {
    pub const fn get_all() -> [RubikColor; 6] {
        [White, Yellow, Red, Orange, Green, Blue]
    }

    pub fn solid_color(&self) -> HexColor {
        match self {
            Red => HexColor::parse("#ff0700").unwrap(),
            Green => HexColor::parse("#00fa45").unwrap(),
            Blue => HexColor::parse("#0400ff").unwrap(),
            Yellow => HexColor::parse("#ffd200").unwrap(),
            White => HexColor::parse("#ffffff").unwrap(),
            Orange => HexColor::parse("#ff8000").unwrap(),
        }
    }

    pub fn darkened_color(&self) -> HexColor {
        match self {
            Red => HexColor::parse("#570100").unwrap(),
            Green => HexColor::parse("#003810").unwrap(),
            Blue => HexColor::parse("#000053").unwrap(),
            Yellow => HexColor::parse("#4e3c00").unwrap(),
            White => HexColor::parse("#3b3b3b").unwrap(),
            Orange => HexColor::parse("#583602").unwrap(),
        }
    }

    pub fn closest_color(to: HexColor) -> Self {
        Self::get_all()
            .into_iter()
            .max_by_key(|color| {
                let c = color.solid_color();
                (to.r - c.r) + (to.g - c.g) + (to.b - c.b)
            })
            .unwrap()
    }

    pub fn to_letter(self) -> char {
        match self {
            White => 'w',
            Yellow => 'y',
            Red => 'r',
            Orange => 'o',
            Green => 'g',
            Blue => 'b',
        }
    }

    pub fn from_letter(c: char) -> Option<Self> {
        match c {
            'w' => Some(White),
            'y' => Some(Yellow),
            'r' => Some(Red),
            'o' => Some(Orange),
            'g' => Some(Green),
            'b' => Some(Blue),
            _ => None,
        }
    }
}
