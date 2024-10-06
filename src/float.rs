use crate::prelude::*;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Float(pub UnderlyingFloat);

impl Float {
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}

impl PartialEq for Float {
    /// Checks two Floats for almost equality
    ///
    /// ```
    /// use rtlib::float::Float;
    /// let a = Float(0.1);
    /// let b = Float(0.1000001);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// use rtlib::float::Float;
    /// let a = Float(0.1);
    /// let b = Float(0.2);
    /// assert_ne!(a, b);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
    }
}

impl PartialEq<UnderlyingFloat> for Float {
    /// Checks two Floats for almost equality
    ///
    /// ```
    /// use rtlib::float::Float;
    /// let a = Float(0.1);
    /// let b = 0.1000001;
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// use rtlib::float::Float;
    /// let a = Float(0.1);
    /// let b = 0.2;
    /// assert_ne!(a, b);
    /// ```
    fn eq(&self, other: &f64) -> bool {
        *self == Float(*other)
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.0 < other.0 {
            return Some(std::cmp::Ordering::Less);
        }
        if self.0 > other.0 {
            return Some(std::cmp::Ordering::Greater);
        }
        None
    }
}

impl PartialOrd<f64> for Float {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        PartialOrd::<Float>::partial_cmp(self, &Float(*other))
    }
}

impl std::fmt::Debug for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::ops::AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl std::ops::SubAssign for Float {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl std::ops::Neg for Float {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::MulAssign for Float {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl std::ops::Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl std::ops::DivAssign for Float {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl std::ops::Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res /= rhs;
        res
    }
}

impl std::iter::Sum<Float> for Float {
    fn sum<I: Iterator<Item = Float>>(iter: I) -> Self {
        Self(iter.map(|f| f.0).sum())
    }
}

#[cfg(test)]
mod tests {}
