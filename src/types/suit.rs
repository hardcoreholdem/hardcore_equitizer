use super::stacked_error::StackedError;
use crate::format_stacked_err;

#[derive(Clone)]
pub struct Suit {
    pub value: i32,
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self.value {
            0 => return "c".to_string(),
            1 => return "d".to_string(),
            2 => return "h".to_string(),
            3 => return "s".to_string(),
            _ => panic!("invalid suit"),
        };
    }
}

impl Suit {
    pub fn from_value(value: i32) -> Suit {
        Suit { value }
    }

    pub fn parse(abbr: char) -> Result<Self, StackedError> {
        match abbr {
            's' => return Ok(Self::SPADE),
            'h' => return Ok(Self::HEART),
            'd' => return Ok(Self::DIAMOND),
            'c' => return Ok(Self::CLUB),
            _ => {
                return format_stacked_err!(
                    "Suit::parse({}:{}) invalid suit: {:?}",
                    file!(),
                    line!(),
                    abbr
                );
            }
        };
    }

    pub fn from_i32(value: i32) -> Self {
        Suit { value }
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }

    pub const SPADE: Suit = Suit { value: 3 };
    pub const HEART: Suit = Suit { value: 2 };
    pub const DIAMOND: Suit = Suit { value: 1 };
    pub const CLUB: Suit = Suit { value: 0 };
}
