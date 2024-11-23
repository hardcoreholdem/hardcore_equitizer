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
}

#[derive(Copy, Clone)]
pub struct WeightedCombo {
    pub combo: Combo,
    pub weight: f64,
}
