use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
    SubAssign,
};

/// A representation of a 24-bit unsigned integer.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u24([u8; 3]);

impl u24 {
    pub const MAX: u32 = 1 >> 24;

    /// Creates a new `u24` from a `u32`.
    // this is a false negative from clippy, we can bypass it
    #[allow(clippy::absurd_extreme_comparisons)]
    pub fn new(value: u32) -> Self {
        debug_assert!(value <= Self::MAX);
        Self::from_u32(value)
    }

    /// Converts `self` into a `u32`.
    pub fn to_u32(self) -> u32 {
        let [a, b, c] = self.0;
        u32::from_le_bytes([a, b, c, 0])
    }

    fn from_u32(val: u32) -> Self {
        let [a, b, c, _] = val.to_le_bytes();
        Self([a, b, c])
    }

    fn load_from_u32(&mut self, val: u32) {
        let [a, b, c, _] = val.to_le_bytes();
        self.0 = [a, b, c];
    }
}

impl Add for u24 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_u32(self.to_u32() + rhs.to_u32())
    }
}

impl AddAssign for u24 {
    fn add_assign(&mut self, rhs: Self) {
        self.load_from_u32(self.to_u32() + rhs.to_u32());
    }
}

impl Sub for u24 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_u32(self.to_u32() - rhs.to_u32())
    }
}

impl SubAssign for u24 {
    fn sub_assign(&mut self, rhs: Self) {
        self.load_from_u32(self.to_u32() - rhs.to_u32());
    }
}

impl Mul for u24 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_u32(self.to_u32() * rhs.to_u32())
    }
}

impl MulAssign for u24 {
    fn mul_assign(&mut self, rhs: Self) {
        self.load_from_u32(self.to_u32() * rhs.to_u32());
    }
}

impl Div for u24 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_u32(self.to_u32() / rhs.to_u32())
    }
}

impl DivAssign for u24 {
    fn div_assign(&mut self, rhs: Self) {
        self.load_from_u32(self.to_u32() / rhs.to_u32());
    }
}

impl Rem for u24 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_u32(self.to_u32() % rhs.to_u32())
    }
}

impl RemAssign for u24 {
    fn rem_assign(&mut self, rhs: Self) {
        self.load_from_u32(self.to_u32() % rhs.to_u32());
    }
}
