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
            Red => HexColor::parse("#350201").unwrap(),
            Green => HexColor::parse("#001907").unwrap(),
            Blue => HexColor::parse("#000027").unwrap(),
            Yellow => HexColor::parse("#302500").unwrap(),
            White => HexColor::parse("#1c1c1c").unwrap(),
            Orange => HexColor::parse("#291901").unwrap(),
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
}
