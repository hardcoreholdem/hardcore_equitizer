use super::stacked_error::StackedError;
use crate::format_stacked_err;
use std::ops::Add;
use std::ops::Sub;

#[derive(PartialEq, PartialOrd)]
pub struct Rank {
    pub value: i32,
}

impl Add<i32> for Rank {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value + rhs,
        }
    }
}

impl Sub<Rank> for Rank {
    type Output = i32;

    fn sub(self, rhs: Rank) -> Self::Output {
        self.value - rhs.value
    }
}

impl ToString for Rank {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Rank {
    pub fn new(value: i32) -> Rank {
        Rank { value }
    }

    pub fn parse(abbr: &str) -> Result<Self, StackedError> {
        let value = match abbr {
            "2" => 0,
            "3" => 1,
            "4" => 2,
            "5" => 3,
            "6" => 4,
            "7" => 5,
            "8" => 6,
            "9" => 7,
            "T" => 8,
            "J" => 9,
            "Q" => 10,
            "K" => 11,
            "A" => 12,
            _ => {
                return format_stacked_err!(
                    "Rank::parse({}:{}) invalid rank: {:?}",
                    file!(),
                    line!(),
                    abbr
                )
            }
        };

        Ok(Self { value })
    }

    pub fn to_str(&self) -> &str {
        match self.value {
            0 => return "2",
            1 => return "3",
            2 => return "4",
            3 => return "5",
            4 => return "6",
            5 => return "7",
            6 => return "8",
            7 => return "9",
            8 => return "T",
            9 => return "J",
            10 => return "Q",
            11 => return "K",
            12 => return "A",
            _ => panic!("invalid rank: {}", self.value),
        };
    }

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }
}

pub const RANK_VALUE_A: i32 = 12;
pub const RANK_VALUE_2: i32 = 0;
pub const RANK_VALUE_3: i32 = 1;
pub const RANK_VALUE_4: i32 = 2;
pub const RANK_VALUE_5: i32 = 3;
pub const RANK_VALUE_6: i32 = 4;
