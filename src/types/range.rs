use super::card::Card;
use super::rank::Rank;
use std::cmp::Ordering;

pub struct Range {
    pub combos: Vec<(Card, Card)>,
}

impl From<&str> for Range {
    fn from(desc: &str) -> Self {
        Self::parse(desc)
    }
}

impl Range {
    pub fn parse(desc: &str) -> Self {
        let mut combos: Vec<(Card, Card)> = Vec::new();

        for token in desc.split(',') {
            if token.len() < 2 {
                panic!("invalid range description: {}", desc);
            }

            match token.strip_suffix("+") {
                Some(token) => Self::parse_plus(token, &mut combos),
                None => {
                    if token.contains("-") {
                        let parts = token.split('-').collect::<Vec<&str>>();
                        if parts.len() != 2 {
                            panic!("invalid range token: {}", token);
                        }
                        Self::parse_range(parts[0], parts[1], &mut combos)
                    } else {
                        Self::parse_normal(token, &mut combos)
                    }
                }
            }
        }

        Self { combos }
    }

    pub fn parse_plus(token: &str, combos: &mut Vec<(Card, Card)>) {
        let rank1 = Rank::parse(&token[0..1]).unwrap();
        let rank2 = Rank::parse(&token[1..2]).unwrap();

        match token.len() {
            2 => match rank1.cmp(&rank2) {
                Ordering::Less => {
                    panic!("invalid range token: {}", token);
                }
                Ordering::Equal => {
                    let bottom_rank = rank1;
                    if bottom_rank == Rank::ACE {
                        panic!("invalid range token: {}", token);
                    }
                    for rank_value in bottom_rank.value..=Rank::VALUE_A {
                        let rank = Rank::from_value(rank_value);
                        for suit1_value in 0..4 {
                            for suit2_value in (suit1_value + 1)..4 {
                                combos.push((
                                    Card::from_rank_suit_value(rank.value, suit1_value),
                                    Card::from_rank_suit_value(rank.value, suit2_value),
                                ));
                            }
                        }
                    }
                }
                Ordering::Greater => {
                    panic!("invalid range token: {}", token);
                }
            },
            3 => {
                match rank1.cmp(&rank2) {
                    Ordering::Less => {
                        panic!("invalid range token: {}", token);
                    }
                    Ordering::Equal => {
                        panic!("invalid range token: {}", token);
                    }
                    Ordering::Greater => {}
                }

                match token.as_bytes()[2] {
                    b's' => {
                        for cur_rank_value in rank2.value..=Rank::VALUE_A {
                            if cur_rank_value == rank1.value {
                                continue;
                            }
                            for suit_value in 0..4 {
                                combos.push((
                                    Card::from_rank_suit_value(rank1.value, suit_value),
                                    Card::from_rank_suit_value(cur_rank_value, suit_value),
                                ));
                            }
                        }
                    }
                    b'o' => {
                        for cur_rank_value in rank2.value..=Rank::VALUE_A {
                            if cur_rank_value == rank1.value {
                                continue;
                            }
                            for suit1_value in 0..4 {
                                for suit2_value in 0..4 {
                                    if suit1_value == suit2_value {
                                        continue;
                                    }

                                    combos.push((
                                        Card::from_rank_suit_value(rank1.value, suit1_value),
                                        Card::from_rank_suit_value(cur_rank_value, suit2_value),
                                    ));
                                }
                            }
                        }
                    }
                    _ => panic!("invalid range token: {}", token),
                }
            }
            _ => panic!("invalid range token: {}", token),
        }
    }

    pub fn parse_normal(token: &str, combos: &mut Vec<(Card, Card)>) {
        let rank1 = Rank::parse(&token[0..1]).unwrap();
        let rank2 = Rank::parse(&token[1..2]).unwrap();

        match token.len() {
            2 => match rank1.cmp(&rank2) {
                Ordering::Less => {
                    panic!("invalid range token: {}", token);
                }
                Ordering::Equal => {
                    let rank = rank1;
                    for suit1_value in 0..4 {
                        for suit2_value in (suit1_value + 1)..4 {
                            combos.push((
                                Card::from_rank_suit_value(rank.value, suit1_value),
                                Card::from_rank_suit_value(rank.value, suit2_value),
                            ));
                        }
                    }
                }
                Ordering::Greater => {
                    for suit1_value in 0..4 {
                        for suit2_value in 0..4 {
                            combos.push((
                                Card::from_rank_suit_value(rank1.value, suit1_value),
                                Card::from_rank_suit_value(rank2.value, suit2_value),
                            ));
                        }
                    }
                }
            },
            3 => match &token[2..3] {
                "s" => {
                    if rank1 <= rank2 {
                        panic!("invalid range token: {}", token);
                    }
                    for suit_value in 0..4 {
                        combos.push((
                            Card::from_rank_suit_value(rank1.value, suit_value),
                            Card::from_rank_suit_value(rank2.value, suit_value),
                        ));
                    }
                }
                "o" => {
                    if rank1 <= rank2 {
                        panic!("invalid range token: {}", token);
                    }
                    for suit1_value in 0..4 {
                        for suit2_value in 0..4 {
                            if suit1_value == suit2_value {
                                continue;
                            }
                            combos.push((
                                Card::from_rank_suit_value(rank1.value, suit1_value),
                                Card::from_rank_suit_value(rank2.value, suit2_value),
                            ));
                        }
                    }
                }
                _ => panic!("invalid range token: {}", token),
            },
            _ => panic!("invalid range token: {}", token),
        }
    }

    pub fn parse_range(from_token: &str, to_token: &str, combos: &mut Vec<(Card, Card)>) {
        let from_rank1 = Rank::parse(&from_token[0..1]).unwrap();
        let from_rank2 = Rank::parse(&from_token[1..2]).unwrap();
        let to_rank1 = Rank::parse(&to_token[0..1]).unwrap();
        let to_rank2 = Rank::parse(&to_token[1..2]).unwrap();

        match from_token.len() {
            2 => match from_rank1.cmp(&from_rank2) {
                Ordering::Less => {
                    panic!("invalid range tokens: {}-{}", from_token, to_token);
                }
                Ordering::Equal => {
                    let from_rank = from_rank1;
                    if to_rank1 != to_rank2 {
                        panic!("invalid range tokens: {}-{}", from_token, to_token);
                    }
                    let to_rank = to_rank1;
                    if from_rank - to_rank <= 1 {
                        panic!("invalid range tokens: {}-{}", from_token, to_token);
                    }
                    for cur_rank_value in to_rank.value..=from_rank.value {
                        for suit1_value in 0..4 {
                            for suit2_value in (suit1_value + 1)..4 {
                                combos.push((
                                    Card::from_rank_suit_value(cur_rank_value, suit1_value),
                                    Card::from_rank_suit_value(cur_rank_value, suit2_value),
                                ));
                            }
                        }
                    }
                }
                Ordering::Greater => {
                    if from_rank1 != to_rank1 {
                        panic!("invalid range tokens: {}-{}", from_token, to_token);
                    }
                    let left_rank = from_rank1;

                    if from_rank2 - to_rank2 <= 1 {
                        panic!("invalid range tokens: {}-{}", from_token, to_token);
                    }

                    for right_rank_value in to_rank2.value..=from_rank2.value {
                        for left_suit_value in 0..4 {
                            for right_suit_value in 0..4 {
                                combos.push((
                                    Card::from_rank_suit_value(left_rank.value, left_suit_value),
                                    Card::from_rank_suit_value(right_rank_value, right_suit_value),
                                ));
                            }
                        }
                    }
                }
            },

            3 => {
                if from_token[2..3] != to_token[2..3] {
                    panic!("invalid range tokens: {}-{}", from_token, to_token);
                }

                if from_rank1 <= from_rank2 {
                    panic!("invalid range tokens: {}-{}", from_token, to_token);
                }

                if from_rank1 != to_rank1 {
                    panic!("invalid range tokens: {}-{}", from_token, to_token);
                }

                let rank1 = from_rank1;

                if from_rank2 - to_rank2 <= 1 {
                    panic!("invalid range tokens: {}-{}", from_token, to_token);
                }

                match &from_token[2..3] {
                    "s" => {
                        for cur_rank2_value in to_rank2.value..=from_rank2.value {
                            for suit_value in 0..4 {
                                combos.push((
                                    Card::from_rank_suit_value(rank1.value, suit_value),
                                    Card::from_rank_suit_value(cur_rank2_value, suit_value),
                                ));
                            }
                        }
                    }
                    "o" => {
                        for cur_rank2_value in to_rank2.value..=from_rank2.value {
                            for suit1_value in 0..4 {
                                for suit2_value in 0..4 {
                                    if suit1_value == suit2_value {
                                        continue;
                                    }
                                    combos.push((
                                        Card::from_rank_suit_value(rank1.value, suit1_value),
                                        Card::from_rank_suit_value(cur_rank2_value, suit2_value),
                                    ));
                                }
                            }
                        }
                    }
                    _ => {
                        panic!("invalid range tokens: {}-{}", from_token, to_token);
                    }
                }
            }
            _ => {
                panic!("invalid range tokens: {}-{}", from_token, to_token);
            }
        }
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.combos
            .iter()
            .all(|combo| !other.combos.contains(combo))
    }
}
