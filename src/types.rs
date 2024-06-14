#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Money(u64);

impl Money {
    pub fn from_rupees(rupees: u64) -> Self {
        Self(rupees * 100)
    }
    pub fn from_paisae(paisae: u64) -> Self {
        Self(paisae)
    }
    pub fn from_parts(rupees: u64, paisae: u64) -> Self {
        Self(rupees * 100 + paisae)
    }
    pub fn into_parts(self) -> [u64; 2] {
        [self.0 / 100, self.0 % 100]
    }
    pub fn into_paisae(self) -> u64 {
        self.0
    }
}
impl From<f64> for Money {
    fn from(value: f64) -> Self {
        Self((value * 100.) as _)
    }
}
impl From<Money> for f64 {
    fn from(value: Money) -> Self {
        value.0 as f64 / 100.
    }
}
impl std::fmt::Debug for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0 / 100, self.0 % 100)
    }
}
impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "₹{}.{}", self.0 / 100, self.0 % 100)
    }
}
impl std::ops::Add for Money {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Sub for Money {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl std::ops::Mul<u64> for Money {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl std::ops::Div<u64> for Money {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl std::ops::Rem<u64> for Money {
    type Output = Self;
    fn rem(self, rhs: u64) -> Self::Output {
        Self(self.0 % rhs)
    }
}
impl std::ops::AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl std::ops::SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl std::ops::MulAssign<u64> for Money {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}
impl std::ops::DivAssign<u64> for Money {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}
impl std::ops::RemAssign<u64> for Money {
    fn rem_assign(&mut self, rhs: u64) {
        self.0 %= rhs;
    }
}
