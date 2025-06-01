use super::types::Card;
use super::types::HandRank;
use super::types::Rank;
use super::types::Suit;
use permutohedron::LexicalPermutation;
use std::cmp::max;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

const SUIT_CNT_TABLE: [usize; 4609] = [
    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0,
    2, 2, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 3, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 0, 0, 0, 1, 1, 1, 3, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 4, 4, 0, 0, 0, 4, 4, 4, 4, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 4, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 4, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 4, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    4,
];

const POW_13_2: usize = 13 * 13;
const POW_13_3: usize = POW_13_2 * 13;
const POW_13_4: usize = POW_13_3 * 13;
const POW_13_5: usize = POW_13_4 * 13;
const POW_13_6: usize = POW_13_5 * 13;
const POW_13_7: usize = POW_13_6 * 13;

pub struct HandRanker {
    suited: Vec<HandRank>,
    offsuited5: Vec<HandRank>,
    offsuited7: Vec<HandRank>,
}

impl HandRanker {
    pub fn new(mut data_dir: &str) -> Result<Self, ()> {
        if data_dir.is_empty() {
            data_dir = "data";
        }

        std::fs::create_dir_all(data_dir).unwrap();

        for c in 0..7 {
            for d in 0..7 {
                for h in 0..7 {
                    for s in 0..7 {
                        if c + d + h + s > 6 {
                            continue;
                        }
                        if c + d + h + s < 5 {
                            continue;
                        }

                        let suit_cnt_hash = c | (d << 3) | (h << 6) | (s << 9);
                        let suit_cnt = SUIT_CNT_TABLE[suit_cnt_hash];

                        if c >= 5 {
                            if suit_cnt != Suit::CLUB.as_usize() + 1 {
                                panic!("suited7 error");
                            }
                        } else if d >= 5 {
                            if suit_cnt != Suit::DIAMOND.as_usize() + 1 {
                                panic!("suited7 error");
                            }
                        } else if h >= 5 {
                            if suit_cnt != Suit::HEART.as_usize() + 1 {
                                panic!("suited7 error");
                            }
                        } else if s >= 5 {
                            if suit_cnt != Suit::SPADE.as_usize() + 1 {
                                panic!("suited7 error");
                            }
                        } else {
                            if suit_cnt != 0 {
                                panic!("suited7 error");
                            }
                        }
                    }
                }
            }
        }

        let mut result = Self {
            suited: vec![HandRank::ERROR; 1 << 13],
            offsuited5: vec![HandRank::ERROR; POW_13_5],
            offsuited7: vec![HandRank::ERROR; POW_13_7],
        };

        match result.load_data(data_dir) {
            Ok(_) => Ok(result),
            Err(_) => {
                println!("failed to load data from `{}`", data_dir);
                result.calc_data();
                result.save_data(data_dir).unwrap();
                Ok(result)
            }
        }
    }

    pub fn get7_offsuited(&self, ranks: [Rank; 7]) -> HandRank {
        let mut hash = 0;
        for r in ranks {
            hash = hash * 13 + r.as_usize();
        }
        self.offsuited7[hash]
    }

    pub fn get7(&self, cards: [Card; 7]) -> HandRank {
        let ranks = [
            cards[0].rank(),
            cards[1].rank(),
            cards[2].rank(),
            cards[3].rank(),
            cards[4].rank(),
            cards[5].rank(),
            cards[6].rank(),
        ];
        let suits = [
            cards[0].suit(),
            cards[1].suit(),
            cards[2].suit(),
            cards[3].suit(),
            cards[4].suit(),
            cards[5].suit(),
            cards[6].suit(),
        ];

        let mut suit_cnt_hash = 0;
        for s in &suits {
            suit_cnt_hash += 1 << (s.as_usize() * 3);
        }

        if SUIT_CNT_TABLE[suit_cnt_hash] != 0 {
            let mut suited_hash = [0; 4];
            for i in 0..7 {
                suited_hash[suits[i].clone().as_usize()] |= 1 << ranks[i].as_usize();
            }
            let s = SUIT_CNT_TABLE[suit_cnt_hash] - 1;
            return self.suited[suited_hash[s]].clone();
        } else {
            self.get7_offsuited(ranks)
        }
    }

    pub fn get5_offsuited(&self, ranks: [Rank; 5]) -> HandRank {
        let mut hash = 0;
        for r in ranks {
            hash = hash * 13 + r.as_usize();
        }
        self.offsuited5[hash]
    }

    pub fn get5(&self, cards: [Card; 5]) -> HandRank {
        let bitand_suit_value = cards[0].suit().value
            & cards[1].suit().value
            & cards[2].suit().value
            & cards[3].suit().value
            & cards[4].suit().value;

        if bitand_suit_value == 0 {
            // offsuited
            let ranks = [
                cards[0].rank(),
                cards[1].rank(),
                cards[2].rank(),
                cards[3].rank(),
                cards[4].rank(),
            ];
            self.get5_offsuited(ranks)
        } else {
            // suited
            let mut suited_hash = 0;
            for i in 0..5 {
                suited_hash |= 1 << cards[i].rank().as_usize();
            }
            self.suited[suited_hash]
        }
    }

    fn do_process(
        &mut self,
        hand_rank: &mut HandRank,
        offsuited_details: &mut Vec<([i32; 5], HandRank)>,
        need_flush: bool,
        r0_value: i32,
        r1_value: i32,
        r2_value: i32,
        r3_value: i32,
        r4_value: i32,
    ) {
        // println!(
        //     "do_process: {:?}",
        //     (r0_value, r1_value, r2_value, r3_value, r4_value)
        // );
        if need_flush {
            let hh = (1 << r0_value)
                | (1 << r1_value)
                | (1 << r2_value)
                | (1 << r3_value)
                | (1 << r4_value);
            self.suited[hh] = hand_rank.clone();
            hand_rank.inc();
        } else {
            let card_rank_values = [r0_value, r1_value, r2_value, r3_value, r4_value];
            offsuited_details.push((card_rank_values, hand_rank.clone()));
            hand_rank.inc();
        }
    }

    fn process_no_pair(
        &mut self,
        hand_rank: &mut HandRank,
        offsuited_details: &mut Vec<([i32; 5], HandRank)>,
        need_flush: bool,
        need_straight: bool,
    ) {
        if need_straight {
            self.do_process(
                hand_rank,
                offsuited_details,
                need_flush,
                Rank::VALUE_2,
                Rank::VALUE_3,
                Rank::VALUE_4,
                Rank::VALUE_5,
                Rank::VALUE_A,
            );
            for r_value in Rank::VALUE_6..=Rank::VALUE_A {
                self.do_process(
                    hand_rank,
                    offsuited_details,
                    need_flush,
                    r_value - 4,
                    r_value - 3,
                    r_value - 2,
                    r_value - 1,
                    r_value,
                );
            }
        } else {
            for r4_value in Rank::VALUE_2..=Rank::VALUE_A {
                for r3_value in Rank::VALUE_2..r4_value {
                    for r2_value in Rank::VALUE_2..r3_value {
                        for r1_value in Rank::VALUE_2..r2_value {
                            for r0_value in Rank::VALUE_2..r1_value {
                                let mut is_straight = false;
                                if r3_value - r0_value == 3 {
                                    if r4_value == r3_value + 1
                                        || (r4_value == Rank::VALUE_A && r0_value == Rank::VALUE_2)
                                    {
                                        is_straight = true;
                                    }
                                }

                                if !is_straight {
                                    self.do_process(
                                        hand_rank,
                                        offsuited_details,
                                        need_flush,
                                        r0_value,
                                        r1_value,
                                        r2_value,
                                        r3_value,
                                        r4_value,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn calc_data(&mut self) {
        if HandRank::ERROR.value() >= 0 {
            panic!("ERROR must be negative");
        }

        let mut hand_rank = HandRank::from_value(0);
        let mut offsuited_details = Vec::new();

        self.process_no_pair(&mut hand_rank, &mut offsuited_details, false, false);

        println!(
            "finished process high cards, hand_rank={:?}, offsuited_details.len(): {}",
            hand_rank,
            offsuited_details.len()
        );

        {
            // one pair
            for rr_value in Rank::VALUE_2..=Rank::VALUE_A {
                for r2_value in Rank::VALUE_2..=Rank::VALUE_A {
                    if r2_value == rr_value {
                        continue;
                    }

                    for r1_value in Rank::VALUE_2..r2_value {
                        if r1_value == rr_value {
                            continue;
                        }

                        for r0_value in Rank::VALUE_2..r1_value {
                            if r0_value == rr_value {
                                continue;
                            }

                            let card_ranks = [rr_value, rr_value, r0_value, r1_value, r2_value];

                            offsuited_details.push((card_ranks, hand_rank.clone()));
                            hand_rank.inc();
                        }
                    }
                }
            }
        }

        println!(
            "finished process pair, hand_rank={:?}, offsuited_details.len(): {}",
            hand_rank,
            offsuited_details.len()
        );

        // two pair
        {
            for rr1 in Rank::VALUE_2..=Rank::VALUE_A {
                for rr0 in Rank::VALUE_2..rr1 {
                    for r in Rank::VALUE_2..=Rank::VALUE_A {
                        if r == rr1 || r == rr0 {
                            continue;
                        }

                        let card_ranks = [rr0, rr0, rr1, rr1, r];
                        offsuited_details.push((card_ranks, hand_rank.clone()));
                        hand_rank.inc();
                    }
                }
            }
        }

        println!(
            "finished process two pair, hand_rank={:?}, offsuited_details.len(): {}",
            hand_rank,
            offsuited_details.len()
        );

        // 三条
        {
            for rrr_value in Rank::VALUE_2..=Rank::VALUE_A {
                for r2_value in Rank::VALUE_2..=Rank::VALUE_A {
                    if r2_value == rrr_value {
                        continue;
                    }

                    for r1_value in Rank::VALUE_2..r2_value {
                        if r1_value == rrr_value {
                            continue;
                        }

                        let card_ranks = [rrr_value, rrr_value, rrr_value, r1_value, r2_value];
                        offsuited_details.push((card_ranks, hand_rank.clone()));
                        hand_rank.inc();
                    }
                }
            }
        }

        println!(
            "finished process 三条, hand_rank={:?}, offsuited_details.len(): {}",
            hand_rank,
            offsuited_details.len()
        );

        self.process_no_pair(&mut hand_rank, &mut offsuited_details, false, true);

        println!(
            "finished process 顺子, hand_rank={:?}, offsuited_details.len(): {}",
            hand_rank,
            offsuited_details.len()
        );

        self.process_no_pair(&mut hand_rank, &mut offsuited_details, true, false);

        println!(
            "finished process 同花, hand_rank={:?}, offsuited_details.len(): {}",
            hand_rank,
            offsuited_details.len()
        );

        // 葫芦
        {
            for rrr_value in Rank::VALUE_2..=Rank::VALUE_A {
                for rr_value in Rank::VALUE_2..=Rank::VALUE_A {
                    if rr_value == rrr_value {
                        continue;
                    }

                    let card_ranks = [rrr_value, rrr_value, rrr_value, rr_value, rr_value];
                    offsuited_details.push((card_ranks, hand_rank.clone()));
                    hand_rank.inc();
                }
            }
        }

        // 四条
        {
            for rrrr in Rank::VALUE_2..=Rank::VALUE_A {
                for r in Rank::VALUE_2..=Rank::VALUE_A {
                    if r == rrrr {
                        continue;
                    }

                    let card_ranks = [rrrr, rrrr, rrrr, rrrr, r];
                    offsuited_details.push((card_ranks, hand_rank.clone()));
                    hand_rank.inc();
                }
            }
        }

        self.process_no_pair(&mut hand_rank, &mut offsuited_details, true, true);

        if hand_rank != HandRank::NUM {
            panic!("hand_rank != NUM");
        }

        for s in (0 as usize)..((1 << 13) as usize) {
            for i in 0..13 {
                if test_bit(s, i) {
                    let ns = flip_bit(s, i);
                    self.suited[s] = max(self.suited[s].clone(), self.suited[ns].clone());
                }
            }
        }

        println!("finished process suited");

        for (mut r01234_values, hand_rank) in offsuited_details {
            println!("r01234={:?}", r01234_values);

            {
                r01234_values.sort();
                loop {
                    let mut hash5 = 0;
                    for r_value in r01234_values {
                        hash5 = hash5 * 13 + r_value;
                    }
                    self.offsuited5[hash5 as usize] = hand_rank;

                    if !r01234_values.next_permutation() {
                        break;
                    }
                }
            }

            for r6_value in Rank::VALUE_2..=Rank::VALUE_A {
                for r5_value in Rank::VALUE_2..=r6_value {
                    let mut r0123456_values = [
                        r01234_values[0],
                        r01234_values[1],
                        r01234_values[2],
                        r01234_values[3],
                        r01234_values[4],
                        r5_value,
                        r6_value,
                    ];
                    r0123456_values.sort();

                    loop {
                        let mut hash7 = 0;
                        for r_value in r0123456_values {
                            hash7 = hash7 * 13 + r_value;
                        }
                        self.offsuited7[hash7 as usize] =
                            max(self.offsuited7[hash7 as usize], hand_rank);

                        if !r0123456_values.next_permutation() {
                            break;
                        }
                    }
                }
            }
        }

        println!("calc_data_done");
    }

    fn save_data(&self, data_dir: &str) -> Result<(), ()> {
        if data_dir.is_empty() {
            println!("empty data_dir");
            return Err(());
        }

        let mut offsuited7_sum: i64 = 0;
        let mut suited_sum: i64 = 0;

        // offsuited5
        let mut offsuited5_sum: i64 = 0;
        {
            let filename = format!("{}/offsuited5.bin", data_dir);
            let mut fout = File::create(filename).unwrap();

            for (i, v) in self.offsuited5.iter().enumerate() {
                let v = v.value() as i16;
                fout.write_all(v.to_le_bytes().as_slice()).unwrap();
                offsuited5_sum += (v as i64) * (i as i64);
            }
        }

        // offsuited7
        {
            let filename = format!("{}/offsuited7.bin", data_dir);
            let mut fout = File::create(filename).unwrap();

            for (i, v) in self.offsuited7.iter().enumerate() {
                let v = v.value() as i16;
                fout.write_all(v.to_le_bytes().as_slice()).unwrap();
                offsuited7_sum += (v as i64) * (i as i64);
            }
        }

        // suited
        {
            let filename = format!("{}/suited.bin", data_dir);
            let mut fout = File::create(filename).unwrap();

            for (i, v) in self.suited.iter().enumerate() {
                let v = v.value() as i16;
                fout.write_all(v.to_le_bytes().as_slice()).unwrap();
                suited_sum += (v as i64) * (i as i64);
            }
        }

        println!(
            "offsuited5_sum={} offsuited7_sum={} suited_sum={}",
            offsuited5_sum, offsuited7_sum, suited_sum
        );

        Ok(())
    }

    fn load_data(&mut self, data_dir: &str) -> Result<(), ()> {
        if data_dir.is_empty() {
            println!("empty data_dir");
            return Err(());
        }

        // offsuited5
        {
            let mut offsuited5_sum: i64 = 0;

            let filename = format!("{}/offsuited5.bin", data_dir);
            let fin = File::open(filename).map_err(|_| ())?;
            let mut reader: BufReader<File> = BufReader::new(fin);

            for i in 0..self.offsuited5.len() {
                let mut v = [0; 2];
                reader.read_exact(&mut v).unwrap();
                let v = i16::from_le_bytes(v);
                self.offsuited5[i] = HandRank::from_value(v as i32);
                offsuited5_sum += (v as i64) * (i as i64);
            }

            if offsuited5_sum != 156362476943832 {
                println!("checksum_failed offsuited5_sum={}", offsuited5_sum);
                return Err(());
            }
        }

        // offsuited7
        {
            let mut offsuited7_sum: i64 = 0;

            let filename = format!("{}/offsuited7.bin", data_dir);
            let fin = File::open(filename).map_err(|_| ())?;
            let mut reader: BufReader<File> = BufReader::new(fin);

            for i in 0..self.offsuited7.len() {
                let mut v = [0; 2];
                reader.read_exact(&mut v).unwrap();
                let v = i16::from_le_bytes(v);
                self.offsuited7[i] = HandRank::from_value(v as i32);
                offsuited7_sum += (v as i64) * (i as i64);
            }

            if offsuited7_sum != 7479833936848761882 {
                println!("checksum_failed offsuited7_sum={}", offsuited7_sum);
                return Err(());
            }
        }

        // suited
        {
            let mut suited_sum: i64 = 0;

            let filename = format!("{}/suited.bin", data_dir);
            let fin = File::open(filename).map_err(|_| ())?;
            let mut reader: BufReader<File> = BufReader::new(fin);

            for i in 0..self.suited.len() {
                let mut v = [0; 2];
                reader.read_exact(&mut v).unwrap();
                let v = i16::from_le_bytes(v); // 注意这里不能用 u16, 否则会生成 65535
                self.suited[i] = HandRank::from_value(v as i32);
                suited_sum += (v as i64) * (i as i64);
            }

            if suited_sum != 217636760248 {
                println!("checksum_failed suited_sum={}", suited_sum);
                return Err(());
            }
        }

        Ok(())
    }
}

fn test_bit(s: usize, i: u8) -> bool {
    (s & (1 << i)) != 0
}

fn flip_bit(s: usize, i: u8) -> usize {
    s ^ (1 << i)
}
