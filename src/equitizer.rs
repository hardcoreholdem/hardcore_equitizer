use crate::types::Suit;

use super::hand_ranker::HandRanker;
use super::types::Card;
use super::types::Combo;
use super::types::PureRange;
use super::types::Range;
use super::types::StackedError;
use std::collections::HashMap;
use std::io::{BufRead, Write};
pub struct Equitizer<'a> {
    hand_ranker: &'a HandRanker,
    cache: HashMap<(Card, Card, Card, Card), f64>,
}

impl<'a> Equitizer<'a> {
    const CACHE_FILENAME: &'static str = "data/equitizer_cache.txt";

    pub fn new(hand_ranker: &'a HandRanker) -> Result<Self, StackedError> {
        let mut cache = HashMap::new();

        match std::fs::File::open(Self::CACHE_FILENAME) {
            Ok(file) => {
                for line in std::io::BufReader::new(file).lines() {
                    let line = line.unwrap();
                    let parts = line.split_whitespace().collect::<Vec<&str>>();
                    if parts.len() != 3 {
                        return Err(StackedError::new(format!(
                            "Invalid line in cache file: {}",
                            line
                        )));
                    }
                    let h1 = parts[0];
                    if h1.len() != 4 {
                        return Err(StackedError::new(format!(
                            "Invalid line in cache file: {}",
                            line
                        )));
                    }
                    let c1 = Card::parse(&h1[0..2]).unwrap();
                    let c2 = Card::parse(&h1[2..4]).unwrap();
                    let h2 = parts[1];
                    if h2.len() != 4 {
                        panic!("Invalid line in cache file: {}", line);
                    }
                    let c3 = Card::parse(&h2[0..2]).unwrap();
                    let c4 = Card::parse(&h2[2..4]).unwrap();
                    let equity = parts[2].parse::<f64>().unwrap();
                    cache.insert((c1, c2, c3, c4), equity);
                }
            }
            Err(_) => {
                println!(
                    "failed to open cache file: {:?}, no action required",
                    Self::CACHE_FILENAME
                );
            }
        }

        Ok(Self { hand_ranker, cache })
    }

    pub fn range_vs_range(&mut self, lhs: &impl Range, rhs: &impl Range) -> f64 {
        let mut sum_eq = 0.0;
        let mut sum_weights = 0.0;

        for lhs_weighted_combo in lhs.iter_weighted_combos() {
            for rhs_weighted_combo in rhs.iter_weighted_combos() {
                if lhs_weighted_combo
                    .combo
                    .intersects(&rhs_weighted_combo.combo)
                {
                    continue;
                }

                let weight = lhs_weighted_combo.weight * rhs_weighted_combo.weight;
                sum_eq +=
                    self.hand_vs_hand(lhs_weighted_combo.combo, rhs_weighted_combo.combo) * weight;
                sum_weights += weight;
            }
        }

        sum_eq / sum_weights
    }

    pub fn hand_vs_hand(&mut self, mut hero: Combo, mut villain: Combo) -> f64 {
        if hero.0.suit() == hero.1.suit() {
            let hero_suit = hero.0.suit();
            if villain.0.suit() == villain.1.suit() {
                if hero.0.suit() == villain.0.suit() {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::CLUB);
                    villain.0 = villain.0.with_suit(&Suit::CLUB);
                    villain.1 = villain.1.with_suit(&Suit::CLUB);
                } else {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::CLUB);
                    villain.0 = villain.0.with_suit(&Suit::DIAMOND);
                    villain.1 = villain.1.with_suit(&Suit::DIAMOND);
                }
            } else {
                if hero_suit == villain.0.suit() {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::CLUB);
                    villain.0 = villain.0.with_suit(&Suit::CLUB);
                    villain.1 = villain.1.with_suit(&Suit::DIAMOND);
                } else if hero_suit == villain.1.suit() {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::CLUB);
                    villain.0 = villain.0.with_suit(&Suit::DIAMOND);
                    villain.1 = villain.1.with_suit(&Suit::CLUB);
                } else {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::CLUB);
                    villain.0 = villain.0.with_suit(&Suit::DIAMOND);
                    villain.1 = villain.1.with_suit(&Suit::HEART);
                }
            }
        } else {
            if villain.0.suit() == villain.1.suit() {
                let villain_suit = villain.0.suit();
                if hero.0.suit() == villain_suit {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                    villain.0 = villain.0.with_suit(&Suit::CLUB);
                    villain.1 = villain.1.with_suit(&Suit::CLUB);
                } else if hero.1.suit() == villain_suit {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                    villain.0 = villain.0.with_suit(&Suit::DIAMOND);
                    villain.1 = villain.1.with_suit(&Suit::DIAMOND);
                } else {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                    villain.0 = villain.0.with_suit(&Suit::HEART);
                    villain.1 = villain.1.with_suit(&Suit::HEART);
                }
            } else {
                if hero.0.suit() == villain.0.suit() {
                    if hero.1.suit() == villain.1.suit() {
                        hero.0 = hero.0.with_suit(&Suit::CLUB);
                        hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                        villain.0 = villain.0.with_suit(&Suit::CLUB);
                        villain.1 = villain.1.with_suit(&Suit::DIAMOND);
                    } else {
                        hero.0 = hero.0.with_suit(&Suit::CLUB);
                        hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                        villain.0 = villain.0.with_suit(&Suit::CLUB);
                        villain.1 = villain.1.with_suit(&Suit::HEART);
                    }
                } else if hero.0.suit() == villain.1.suit() {
                    if hero.1.suit() == villain.0.suit() {
                        hero.0 = hero.0.with_suit(&Suit::CLUB);
                        hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                        villain.0 = villain.0.with_suit(&Suit::DIAMOND);
                        villain.1 = villain.1.with_suit(&Suit::CLUB);
                    } else {
                        hero.0 = hero.0.with_suit(&Suit::CLUB);
                        hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                        villain.0 = villain.0.with_suit(&Suit::HEART);
                        villain.1 = villain.1.with_suit(&Suit::CLUB);
                    }
                } else if hero.1.suit() == villain.0.suit() {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                    villain.0 = villain.0.with_suit(&Suit::DIAMOND);
                    villain.1 = villain.1.with_suit(&Suit::HEART);
                } else if hero.1.suit() == villain.1.suit() {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                    villain.0 = villain.0.with_suit(&Suit::HEART);
                    villain.1 = villain.1.with_suit(&Suit::DIAMOND);
                } else {
                    hero.0 = hero.0.with_suit(&Suit::CLUB);
                    hero.1 = hero.1.with_suit(&Suit::DIAMOND);
                    villain.0 = villain.0.with_suit(&Suit::HEART);
                    villain.1 = villain.1.with_suit(&Suit::SPADE);
                }
            }
        }

        let key = (hero.0, hero.1, villain.0, villain.1);
        match self.cache.get(&key) {
            Some(equity) => return *equity,
            None => {}
        };

        let equity = self.calc_combo_vs_combo(hero, villain);
        self.cache.insert(key, equity);

        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(Self::CACHE_FILENAME)
            .unwrap()
            .write_all(
                format!(
                    "{:?}{:?} {:?}{:?} {:?}\n",
                    hero.0, hero.1, villain.0, villain.1, equity
                )
                .as_bytes(),
            )
            .unwrap();

        equity
    }

    pub fn calc_combo_vs_combo(&mut self, lhs: Combo, rhs: Combo) -> f64 {
        let mut win = 0;
        let mut lose = 0;
        let mut tie = 0;

        let deck = (0..52)
            .map(|c| Card::from_value(c))
            .filter(|&c| c != lhs.0 && c != lhs.1 && c != rhs.0 && c != rhs.1)
            .collect::<Vec<_>>();

        for i4 in 0..deck.len() {
            for i3 in 0..i4 {
                for i2 in 0..i3 {
                    for i1 in 0..i2 {
                        for i0 in 0..i1 {
                            let board = [deck[i0], deck[i1], deck[i2], deck[i3], deck[i4]];
                            let hero = [
                                lhs.0, lhs.1, board[0], board[1], board[2], board[3], board[4],
                            ];
                            let villain = [
                                rhs.0, rhs.1, board[0], board[1], board[2], board[3], board[4],
                            ];

                            let hero_hand_rank = self.hand_ranker.get7(hero);
                            let villain_hand_rank = self.hand_ranker.get7(villain);

                            match hero_hand_rank.cmp(&villain_hand_rank) {
                                std::cmp::Ordering::Greater => win += 1,
                                std::cmp::Ordering::Equal => tie += 1,
                                std::cmp::Ordering::Less => lose += 1,
                            }
                        }
                    }
                }
            }
        }

        let equity = ((tie as f64) * 0.5 + (win as f64)) / ((win + lose + tie) as f64);

        equity
    }

    pub fn query_eq(&mut self, lhs: &impl Range, rhs: &impl Range) -> f64 {
        self.range_vs_range(lhs, rhs)
    }

    pub fn query_sub_prob(
        &mut self,
        blockers: &PureRange,
        sub_rhs: &impl Range,
        full_rhs: &impl Range,
    ) -> f64 {
        let mut res = Vec::new();

        for sub_combo in sub_rhs.iter_combos() {
            if !full_rhs
                .iter_combos()
                .any(|full_combo| sub_combo == full_combo)
            {
                panic!("Hand not in full range");
            }
        }

        for &blocker_combo in &blockers.combos {
            let mut total_sub_weights = 0.0;
            for sub_weighted_combo in sub_rhs.iter_weighted_combos() {
                if blocker_combo.intersects(&sub_weighted_combo.combo) {
                    continue;
                }
                total_sub_weights += sub_weighted_combo.weight;
            }

            let mut total_weights = 0.0;
            for full_weighted_combo in full_rhs.iter_weighted_combos() {
                if blocker_combo.intersects(&full_weighted_combo.combo) {
                    continue;
                }
                total_weights += full_weighted_combo.weight;
            }

            res.push(total_sub_weights / total_weights);
        }

        if res.len() == 0 {
            panic!("No combos found");
        }

        let first_res = res[0].clone();

        for x in &res {
            if *x != first_res {
                panic!("Inconsistent number of combos");
            }
        }

        first_res
    }

    pub fn query_prob(&mut self, blockers: &PureRange, range: &impl Range) -> f64 {
        const C_50_2: f64 = 50.0 * 49.0 / 2.0;

        let mut res = Vec::new();

        for &blocker_combo in &blockers.combos {
            let mut sum_weights = 0.0;
            for weighted_combo in range.iter_weighted_combos() {
                if blocker_combo.intersects(&weighted_combo.combo) {
                    continue;
                }

                sum_weights += weighted_combo.weight;
            }
            res.push(sum_weights / C_50_2);
        }

        if res.len() == 0 {
            panic!("No combos found");
        }

        let first_res = res[0].clone();

        for x in &res {
            if *x != first_res {
                panic!("Inconsistent number of combos");
            }
        }

        first_res
    }

    pub fn query_prob_and_eq(&mut self, lhs: &PureRange, rhs: &impl Range) -> (f64, f64) {
        let prob = self.query_prob(lhs, rhs);
        let eq = self.query_eq(lhs, rhs);

        (prob, eq)
    }

    pub fn query_sub_prob_and_eq(
        &mut self,
        lhs: &PureRange,
        rhs: &impl Range,
        full_rhs: &impl Range,
    ) -> (f64, f64) {
        let sub_prob = self.query_sub_prob(lhs, rhs, full_rhs);
        let eq = self.query_eq(lhs, rhs);

        (sub_prob, eq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_sub_prob() {
        let data_dir = "./data";
        let hand_ranker = HandRanker::new(&data_dir).unwrap();
        let mut equitizer = Equitizer::new(&hand_ranker).unwrap();
        let aa = PureRange::from("AA");
        let aa_kk = PureRange::from("AA,KK");
        let eq = equitizer.query_sub_prob(&aa, &aa, &aa_kk);
        assert_eq!(eq, 1.0 / 7.0);
    }
}
