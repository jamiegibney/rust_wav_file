use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
    SubAssign,
};

/// A representation of a 24-bit signed integer.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct i24([u8; 3]);

impl i24 {
    pub const MIN: i32 = -(1 << 23);
    pub const MAX: i32 = 1 << 23;

    /// Creates a new `i24` from a `i32`.
    // this is a false negative from clippy, we can bypass it
    #[allow(clippy::absurd_extreme_comparisons)]
    pub fn new(value: i32) -> Self {
        debug_assert!(value <= Self::MAX);
        Self::from_i32(value)
    }

    /// Converts `self` into a `u32`.
    pub fn to_i32(self) -> i32 {
        let [a, b, c] = self.0;
        i32::from_le_bytes([a, b, c, 0])
    }

    fn from_i32(val: i32) -> Self {
        let [a, b, c, _] = val.to_le_bytes();
        Self([a, b, c])
    }

    fn load_from_i32(&mut self, val: i32) {
        let [a, b, c, _] = val.to_le_bytes();
        self.0 = [a, b, c];
    }
}

impl Add for i24 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_i32(self.to_i32() + rhs.to_i32())
    }
}

impl AddAssign for i24 {
    fn add_assign(&mut self, rhs: Self) {
        self.load_from_i32(self.to_i32() + rhs.to_i32());
    }
}

impl Sub for i24 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_i32(self.to_i32() - rhs.to_i32())
    }
}

impl SubAssign for i24 {
    fn sub_assign(&mut self, rhs: Self) {
        self.load_from_i32(self.to_i32() - rhs.to_i32());
    }
}

impl Mul for i24 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_i32(self.to_i32() * rhs.to_i32())
    }
}

impl MulAssign for i24 {
    fn mul_assign(&mut self, rhs: Self) {
        self.load_from_i32(self.to_i32() * rhs.to_i32());
    }
}

impl Div for i24 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_i32(self.to_i32() / rhs.to_i32())
    }
}

impl DivAssign for i24 {
    fn div_assign(&mut self, rhs: Self) {
        self.load_from_i32(self.to_i32() / rhs.to_i32());
    }
}

impl Rem for i24 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_i32(self.to_i32() % rhs.to_i32())
    }
}

impl RemAssign for i24 {
    fn rem_assign(&mut self, rhs: Self) {
        self.load_from_i32(self.to_i32() % rhs.to_i32());
    }
}
