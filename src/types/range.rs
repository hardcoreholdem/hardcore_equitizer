use super::combo::Combo;
use super::combo::WeightedCombo;

pub trait Range {
    fn iter_weighted_combos(&self) -> impl Iterator<Item = WeightedCombo> + '_;
    fn iter_combos(&self) -> impl Iterator<Item = Combo> + '_;
}
