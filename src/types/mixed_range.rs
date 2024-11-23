use super::card::Card;
use super::combo::{Combo, WeightedCombo};
use super::range::Range;
use super::rank::Rank;
use super::stacked_error::StackedError;
use std::cmp::Ordering;

pub struct MixedRange {
    pub weighted_combos: Vec<WeightedCombo>,
}

impl Range for MixedRange {
    fn iter_weighted_combos(&self) -> impl Iterator<Item = WeightedCombo> + '_ {
        self.weighted_combos.iter().copied()
    }

    fn iter_combos(&self) -> impl Iterator<Item = Combo> + '_ {
        self.weighted_combos.iter().map(|wc| wc.combo)
    }
}

impl From<&str> for MixedRange {
    fn from(desc: &str) -> Self {
        Self::parse(desc).unwrap()
    }
}

impl From<&String> for MixedRange {
    fn from(desc: &String) -> Self {
        Self::parse(desc).unwrap()
    }
}

impl MixedRange {
    pub fn parse(desc: &str) -> Result<Self, StackedError> {
        let mut combos = Vec::new();

        for token in desc.split(',') {
            if token.len() < 2 {
                return Err(StackedError::new(format!(
                    "invalid range description: {:?}",
                    desc
                )));
            }

            match token.strip_suffix("+") {
                Some(token) => combos.extend(Self::parse_plus(token)?),
                None => {
                    if token.contains("-") {
                        let parts = token.split('-').collect::<Vec<&str>>();
                        if parts.len() != 2 {
                            return Err(StackedError::new(format!(
                                "invalid range token: {:?}",
                                token
                            )));
                        }
                        combos.extend(Self::parse_range(parts[0], parts[1])?);
                    } else {
                        combos.extend(Self::parse_normal(token)?);
                    }
                }
            }
        }

        Ok(Self {
            weighted_combos: combos,
        })
    }

    pub fn parse_plus(token: &str) -> Result<Vec<WeightedCombo>, StackedError> {
        let mut res = Vec::new();

        let rank1 = Rank::parse(&token[0..1]).unwrap();
        let rank2 = Rank::parse(&token[1..2]).unwrap();

        match token.len() {
            2 => match rank1.cmp(&rank2) {
                Ordering::Less => {
                    return Err(StackedError::new(format!(
                        "invalid range token: {:?}",
                        token
                    )));
                }
                Ordering::Equal => {
                    let bottom_rank = rank1;
                    if bottom_rank == Rank::ACE {
                        return Err(StackedError::new(format!(
                            "invalid range token: {:?}",
                            token
                        )));
                    }
                    for rank_value in bottom_rank.value..=Rank::VALUE_A {
                        let rank = Rank::from_value(rank_value);
                        for suit1_value in 0..4 {
                            for suit2_value in (suit1_value + 1)..4 {
                                let combo = Combo::new(
                                    Card::from_rank_suit_value(rank.value, suit1_value),
                                    Card::from_rank_suit_value(rank.value, suit2_value),
                                );
                                res.push(combo.with_weight(1.0));
                            }
                        }
                    }
                }
                Ordering::Greater => {
                    return Err(StackedError::new(format!(
                        "invalid range token: {:?}",
                        token
                    )));
                }
            },
            3 => {
                match rank1.cmp(&rank2) {
                    Ordering::Less => {
                        return Err(StackedError::new(format!(
                            "invalid range token: {:?}",
                            token
                        )));
                    }
                    Ordering::Equal => {
                        return Err(StackedError::new(format!(
                            "invalid range token: {:?}",
                            token
                        )));
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
                                let combo = Combo::new(
                                    Card::from_rank_suit_value(rank1.value, suit_value),
                                    Card::from_rank_suit_value(cur_rank_value, suit_value),
                                );
                                res.push(combo.with_weight(1.0));
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

                                    let combo = Combo::new(
                                        Card::from_rank_suit_value(rank1.value, suit1_value),
                                        Card::from_rank_suit_value(cur_rank_value, suit2_value),
                                    );
                                    res.push(combo.with_weight(1.0));
                                }
                            }
                        }
                    }
                    _ => {
                        return Err(StackedError::new(format!(
                            "invalid range token: {:?}",
                            token
                        )));
                    }
                }
            }
            _ => {
                return Err(StackedError::new(format!(
                    "invalid range token: {:?}",
                    token
                )));
            }
        }

        Ok(res)
    }

    pub fn parse_normal(token: &str) -> Result<Vec<WeightedCombo>, StackedError> {
        let mut res = Vec::new();
        let rank1 = Rank::parse(&token[0..1]).unwrap();
        let rank2 = Rank::parse(&token[1..2]).unwrap();

        match token.len() {
            2 => match rank1.cmp(&rank2) {
                Ordering::Less => {
                    return Err(StackedError::new(format!(
                        "invalid range token: {:?}",
                        token
                    )));
                }
                Ordering::Equal => {
                    let rank = rank1;
                    for suit1_value in 0..4 {
                        for suit2_value in (suit1_value + 1)..4 {
                            let combo = Combo::new(
                                Card::from_rank_suit_value(rank.value, suit1_value),
                                Card::from_rank_suit_value(rank.value, suit2_value),
                            );
                            res.push(combo.with_weight(1.0));
                        }
                    }
                }
                Ordering::Greater => {
                    for suit1_value in 0..4 {
                        for suit2_value in 0..4 {
                            let combo = Combo::new(
                                Card::from_rank_suit_value(rank1.value, suit1_value),
                                Card::from_rank_suit_value(rank2.value, suit2_value),
                            );
                            res.push(combo.with_weight(1.0));
                        }
                    }
                }
            },
            3 => match &token[2..3] {
                "s" => {
                    if rank1 <= rank2 {
                        return Err(StackedError::new(format!(
                            "invalid range token: {:?}",
                            token
                        )));
                    }
                    for suit_value in 0..4 {
                        let combo = Combo::new(
                            Card::from_rank_suit_value(rank1.value, suit_value),
                            Card::from_rank_suit_value(rank2.value, suit_value),
                        );
                        res.push(combo.with_weight(1.0));
                    }
                }
                "o" => {
                    if rank1 <= rank2 {
                        return Err(StackedError::new(format!(
                            "invalid range token: {:?}",
                            token
                        )));
                    }
                    for suit1_value in 0..4 {
                        for suit2_value in 0..4 {
                            if suit1_value == suit2_value {
                                continue;
                            }
                            let combo = Combo::new(
                                Card::from_rank_suit_value(rank1.value, suit1_value),
                                Card::from_rank_suit_value(rank2.value, suit2_value),
                            );
                            res.push(combo.with_weight(1.0));
                        }
                    }
                }
                _ => {
                    return Err(StackedError::new(format!(
                        "invalid range token: {:?}",
                        token
                    )));
                }
            },
            _ => {
                return Err(StackedError::new(format!(
                    "invalid range token: {:?}",
                    token
                )));
            }
        }

        Ok(res)
    }

    pub fn parse_range(
        from_token: &str,
        to_token: &str,
    ) -> Result<Vec<WeightedCombo>, StackedError> {
        let mut res = Vec::new();

        let from_rank1 = Rank::parse(&from_token[0..1]).unwrap();
        let from_rank2 = Rank::parse(&from_token[1..2]).unwrap();
        let to_rank1 = Rank::parse(&to_token[0..1]).unwrap();
        let to_rank2 = Rank::parse(&to_token[1..2]).unwrap();

        match from_token.len() {
            2 => match from_rank1.cmp(&from_rank2) {
                Ordering::Less => {
                    return Err(StackedError::new(format!(
                        "invalid range tokens: {:?}-{:?}",
                        from_token, to_token
                    )));
                }
                Ordering::Equal => {
                    let from_rank = from_rank1;
                    if to_rank1 != to_rank2 {
                        return Err(StackedError::new(format!(
                            "invalid range tokens: {:?}-{:?}",
                            from_token, to_token
                        )));
                    }
                    let to_rank = to_rank1;
                    if from_rank - to_rank <= 1 {
                        return Err(StackedError::new(format!(
                            "invalid range tokens: {:?}-{:?}",
                            from_token, to_token
                        )));
                    }
                    for cur_rank_value in to_rank.value..=from_rank.value {
                        for suit1_value in 0..4 {
                            for suit2_value in (suit1_value + 1)..4 {
                                let combo = Combo::new(
                                    Card::from_rank_suit_value(cur_rank_value, suit1_value),
                                    Card::from_rank_suit_value(cur_rank_value, suit2_value),
                                );
                                res.push(combo.with_weight(1.0));
                            }
                        }
                    }
                }
                Ordering::Greater => {
                    if from_rank1 != to_rank1 {
                        return Err(StackedError::new(format!(
                            "invalid range tokens: {:?}-{:?}",
                            from_token, to_token
                        )));
                    }
                    let left_rank = from_rank1;

                    if from_rank2 - to_rank2 <= 1 {
                        return Err(StackedError::new(format!(
                            "invalid range tokens: {:?}-{:?}",
                            from_token, to_token
                        )));
                    }

                    for right_rank_value in to_rank2.value..=from_rank2.value {
                        for left_suit_value in 0..4 {
                            for right_suit_value in 0..4 {
                                let combo = Combo::new(
                                    Card::from_rank_suit_value(left_rank.value, left_suit_value),
                                    Card::from_rank_suit_value(right_rank_value, right_suit_value),
                                );
                                res.push(combo.with_weight(1.0));
                            }
                        }
                    }
                }
            },

            3 => {
                if from_token[2..3] != to_token[2..3] {
                    return Err(StackedError::new(format!(
                        "invalid range tokens: {:?}-{:?}",
                        from_token, to_token
                    )));
                }

                if from_rank1 <= from_rank2 {
                    return Err(StackedError::new(format!(
                        "invalid range tokens: {:?}-{:?}",
                        from_token, to_token
                    )));
                }

                if from_rank1 != to_rank1 {
                    return Err(StackedError::new(format!(
                        "invalid range tokens: {:?}-{:?}",
                        from_token, to_token
                    )));
                }

                let rank1 = from_rank1;

                if from_rank2 - to_rank2 <= 1 {
                    return Err(StackedError::new(format!(
                        "invalid range tokens: {:?}-{:?}",
                        from_token, to_token
                    )));
                }

                match &from_token[2..3] {
                    "s" => {
                        for cur_rank2_value in to_rank2.value..=from_rank2.value {
                            for suit_value in 0..4 {
                                let combo = Combo::new(
                                    Card::from_rank_suit_value(rank1.value, suit_value),
                                    Card::from_rank_suit_value(cur_rank2_value, suit_value),
                                );
                                res.push(combo.with_weight(1.0));
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
                                    let combo = Combo::new(
                                        Card::from_rank_suit_value(rank1.value, suit1_value),
                                        Card::from_rank_suit_value(cur_rank2_value, suit2_value),
                                    );
                                    res.push(combo.with_weight(1.0));
                                }
                            }
                        }
                    }
                    _ => {
                        return Err(StackedError::new(format!(
                            "invalid range tokens: {:?}-{:?}",
                            from_token, to_token
                        )));
                    }
                }
            }
            _ => {
                return Err(StackedError::new(format!(
                    "invalid range tokens: {:?}-{:?}",
                    from_token, to_token
                )));
            }
        }

        Ok(res)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for &WeightedCombo { combo, weight: _ } in &self.weighted_combos {
            for &WeightedCombo {
                combo: other_combo,
                weight: _,
            } in &other.weighted_combos
            {
                if combo == other_combo {
                    return false;
                }
            }
        }
        true
    }

    pub fn combos(&self) -> impl Iterator<Item = Combo> + '_ {
        self.weighted_combos.iter().map(|wc| wc.combo)
    }

    pub fn contain_combo(&self, combo: &Combo) -> bool {
        self.weighted_combos.iter().any(|wc| wc.combo == *combo)
    }
}
