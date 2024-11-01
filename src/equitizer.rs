use crate::types::Suit;

use super::hand_ranker::HandRanker;
use super::types::Card;
use super::types::Range;
use std::collections::HashMap;
use std::io::{BufRead, Write};
pub struct Equitizer<'a> {
    hand_ranker: &'a HandRanker,
    cache: HashMap<(Card, Card, Card, Card), f64>,
}

impl<'a> Equitizer<'a> {
    const CACHE_FILENAME: &'static str = "data/equitizer_cache.txt";

    pub fn new(hand_ranker: &'a HandRanker) -> Self {
        let mut cache = HashMap::new();

        match std::fs::File::open(Self::CACHE_FILENAME) {
            Ok(file) => {
                for line in std::io::BufReader::new(file).lines() {
                    let line = line.unwrap();
                    let parts = line.split_whitespace().collect::<Vec<&str>>();
                    if parts.len() != 3 {
                        panic!("Invalid line in cache file: {}", line);
                    }
                    let h1 = parts[0];
                    if h1.len() != 4 {
                        panic!("Invalid line in cache file: {}", line);
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

        Self { hand_ranker, cache }
    }

    pub fn range_vs_range(&mut self, hero_range: &Range, villain_range: &Range) -> f64 {
        let mut sum = 0.0;
        let mut cnt = 0.0;

        for &hero_hand in &hero_range.combos {
            for &villain_hand in &villain_range.combos {
                if hero_hand.0 == villain_hand.0
                    || hero_hand.0 == villain_hand.1
                    || hero_hand.1 == villain_hand.0
                    || hero_hand.1 == villain_hand.1
                {
                    continue;
                }

                sum += self.hand_vs_hand(hero_hand, villain_hand);
                cnt += 1.0;
            }
        }

        sum / cnt
    }

    pub fn hand_vs_hand(&mut self, mut hero: (Card, Card), mut villain: (Card, Card)) -> f64 {
        if hero.0 > hero.1 {
            std::mem::swap(&mut hero.0, &mut hero.1);
        }
        if villain.0 > villain.1 {
            std::mem::swap(&mut villain.0, &mut villain.1);
        }

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

        let equity = self.calc_hand_vs_hand(hero, villain);
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

    pub fn calc_hand_vs_hand(&mut self, hero: (Card, Card), villain: (Card, Card)) -> f64 {
        let mut win = 0;
        let mut lose = 0;
        let mut tie = 0;

        let deck = (0..52)
            .map(|c| Card::from_value(c))
            .filter(|&c| c != hero.0 && c != hero.1 && c != villain.0 && c != villain.1)
            .collect::<Vec<_>>();

        for i4 in 0..deck.len() {
            for i3 in 0..i4 {
                for i2 in 0..i3 {
                    for i1 in 0..i2 {
                        for i0 in 0..i1 {
                            let board = [deck[i0], deck[i1], deck[i2], deck[i3], deck[i4]];
                            let hero = [
                                hero.0, hero.1, board[0], board[1], board[2], board[3], board[4],
                            ];
                            let villain = [
                                villain.0, villain.1, board[0], board[1], board[2], board[3],
                                board[4],
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

    pub fn query_eq(&mut self, hero: &str, villain: &str) -> f64 {
        let hero_range = Range::parse(hero);
        let villain_range = Range::parse(villain);
        self.range_vs_range(&hero_range, &villain_range)
    }

    pub fn query_prob(&mut self, blocks: &str, range: &str) -> f64 {
        const C_50_2: f64 = 50.0 * 49.0 / 2.0;

        let range = Range::parse(range);
        let blocks = Range::parse(blocks);

        let mut res = Vec::new();

        for &blocker_hand in &blocks.combos {
            let mut cnt = 0;
            for &hand in &range.combos {
                if blocker_hand.0 != hand.0
                    && blocker_hand.0 != hand.1
                    && blocker_hand.1 != hand.0
                    && blocker_hand.1 != hand.1
                {
                    cnt += 1;
                }
            }
            res.push(cnt as f64 / C_50_2);
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

    pub fn query_prob_and_eq(&mut self, lhs: &str, rhs: &str) -> (f64, f64) {
        let prob = self.query_prob(lhs, rhs);
        let eq = self.query_eq(lhs, rhs);

        (prob, eq)
    }
}
