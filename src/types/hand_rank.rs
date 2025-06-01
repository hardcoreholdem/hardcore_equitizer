use std::ops::Add;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Copy)]
pub struct HandRank {
    value: i32,
}

impl Add<i32> for HandRank {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self::from_value(self.value + rhs)
    }
}

impl HandRank {
    pub const ERROR: Self = Self { value: -1 };
    pub const NUM: Self = Self { value: 7462 };

    pub fn from_value(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }
}
