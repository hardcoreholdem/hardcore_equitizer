use super::rank::Rank;
use super::suit::Suit;
use crate::{format_stacked_err, stack_error};

use std::fmt;

use super::stacked_error::StackedError;

#[derive(PartialEq, Copy, Clone, Hash, Eq, PartialOrd)]
pub struct Card {
    value: i32, // rank << 2 | suit
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        return format!("{}{}", self.rank().to_string(), self.suit().to_string());
    }
}

impl Card {
    pub fn parse(abbr: &str) -> Result<Self, StackedError> {
        if abbr.len() != 2 {
            return format_stacked_err!("invalid card: `{}`", abbr);
        }

        let rank = Rank::parse(&abbr[0..1]).map_err(stack_error!(
            "Card::parse({}:{}) invalid card: {}",
            file!(),
            line!(),
            abbr
        ))?;
        let suit = Suit::parse(abbr.chars().nth(1).unwrap()).map_err(stack_error!(
            "Card::parse({}:{}) invalid card: {}",
            file!(),
            line!(),
            abbr
        ))?;

        Ok(Self {
            value: rank.value << 2 | suit.value,
        })
    }

    pub fn suit(self) -> Suit {
        Suit::new(self.value & 0b11)
    }

    pub fn rank(self) -> Rank {
        Rank::new(self.value >> 2)
    }

    pub fn from_value(value: i32) -> Self {
        Self { value }
    }

    pub fn value(self) -> i32 {
        self.value
    }

    pub fn from_rank_suit_value(rank_value: i32, suit_value: i32) -> Self {
        Self::from_value(rank_value << 2 | suit_value)
    }
}
