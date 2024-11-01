use std::ops::Add;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct HandRank(i32);

impl Add<i32> for HandRank {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl HandRank {
    pub const ERROR: Self = Self(-1);
    pub const NUM: Self = Self(7462);

    pub fn from_value(value: i32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn inc(&mut self) {
        self.0 += 1;
    }
}
