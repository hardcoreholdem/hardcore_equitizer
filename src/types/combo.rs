use super::card::Card;
use std::cmp::Ordering;

#[derive(PartialEq, Copy, Clone)]
pub struct Combo(pub Card, pub Card);

impl Combo {
    pub fn new(left: Card, right: Card) -> Self {
        match left.cmp(&right) {
            Ordering::Greater => Self(left, right),
            Ordering::Less => Self(right, left),
            Ordering::Equal => panic!("left and right cards are the same"),
        }
    }

    pub fn with_weight(self, weight: f64) -> WeightedCombo {
        WeightedCombo {
            combo: self,
            weight,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.0 == other.0 || self.0 == other.1 || self.1 == other.0 || self.1 == other.1
    }
}

#[derive(Copy, Clone)]
pub struct WeightedCombo {
    pub combo: Combo,
    pub weight: f64,
}
