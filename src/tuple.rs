use std::fmt::Formatter;
use crate::prelude::*;
use std::ops::IndexMut;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple {
    pub elements: [Float; 4],
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[derive(PartialEq)]
        enum TupleType { Point, Vector, Neither }

        let mut tuple_type = TupleType::Neither;
        if self.is_point() {
            tuple_type = TupleType::Point;
        } else if self.is_vector() {
            tuple_type = TupleType::Vector;
        }

        write!(f, "{}(", match tuple_type {
            TupleType::Point => "Point",
            TupleType::Vector => "Vector",
            TupleType::Neither => "Tuple",
        })?;

        write!(f, "{}, {}, {}", self.elements[0], self.elements[1], self.elements[2])?;

        if tuple_type == TupleType::Neither {
            write!(f, ", {}", self.elements[3])?;
        }

        write!(f, ")")
    }
}

impl Tuple {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Self {
        Self {
            elements: [x, y, z, w],
        }
    }

    pub fn new_from_underlying(
        x: UnderlyingFloat,
        y: UnderlyingFloat,
        z: UnderlyingFloat,
        w: UnderlyingFloat,
    ) -> Self {
        Self::new(Float(x), Float(y), Float(z), Float(w))
    }

    pub fn zero() -> Self {
        Self::new_from_underlying(0.0, 0.0, 0.0, 0.0)
    }
    pub fn point(x: Float, y: Float, z: Float) -> Self {
        Self::new(x, y, z, Float(1.0))
    }

    /// Constructs a Tuple with w = 1
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// assert_eq!(
    ///   Tuple::point_from_underlying(4.0, -4.0, 3.0),
    ///   Tuple::new_from_underlying(4.0, -4.0, 3.0, 1.0)
    /// );
    pub fn point_from_underlying(
        x: UnderlyingFloat,
        y: UnderlyingFloat,
        z: UnderlyingFloat,
    ) -> Self {
        Self::point(Float(x), Float(y), Float(z))
    }

    pub fn vector(x: Float, y: Float, z: Float) -> Self {
        Self::new(x, y, z, Float(0.0))
    }

    /// Constructs a Tuple with w = 0
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// assert_eq!(
    ///   Tuple::vector_from_underlying(4.0, -4.0, 3.0),
    ///   Tuple::new_from_underlying(4.0, -4.0, 3.0, 0.0)
    /// );
    pub fn vector_from_underlying(
        x: UnderlyingFloat,
        y: UnderlyingFloat,
        z: UnderlyingFloat,
    ) -> Self {
        Self::vector(Float(x), Float(y), Float(z))
    }

    /// Tuple with w == 1
    ///
    /// ```
    /// use rtlib::float::Float;
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::new_from_underlying(4.3, -4.2, 3.1, 1.0);
    /// assert_eq!(a.get_x(), 4.3);
    /// assert_eq!(a.get_y(), -4.2);
    /// assert_eq!(a.get_z(), 3.1);
    /// assert_eq!(a.get_w(), 1.0);
    /// assert!(a.is_point());
    /// assert!(!a.is_vector());
    /// ```
    pub fn is_point(&self) -> bool {
        self.elements[3] == 1.0
    }

    /// Tuple with w == 0
    ///
    /// ```
    /// use rtlib::float::Float;
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::new_from_underlying(4.3, -4.2, 3.1, 0.0);
    /// assert_eq!(a.get_x(), 4.3);
    /// assert_eq!(a.get_y(), -4.2);
    /// assert_eq!(a.get_z(), 3.1);
    /// assert_eq!(a.get_w(), 0.0);
    /// assert!(!a.is_point());
    /// assert!(a.is_vector());
    /// ```
    pub fn is_vector(&self) -> bool {
        self.elements[3] == 0.0
    }

    pub fn get_x(&self) -> Float {
        self.elements[0]
    }

    pub fn get_y(&self) -> Float {
        self.elements[1]
    }

    pub fn get_z(&self) -> Float {
        self.elements[2]
    }

    pub fn get_w(&self) -> Float {
        self.elements[3]
    }

    pub fn get_underlying_x(&self) -> f64 {
        self.get_x().0
    }

    pub fn get_underlying_y(&self) -> f64 {
        self.get_y().0
    }

    pub fn get_underlying_z(&self) -> f64 {
        self.get_z().0
    }

    pub fn get_underlying_w(&self) -> f64 {
        self.get_w().0
    }

    pub fn get_mut_x(&mut self) -> &mut Float {
        self.elements.index_mut(0)
    }

    pub fn get_mut_y(&mut self) -> &mut Float {
        self.elements.index_mut(1)
    }

    pub fn get_mut_z(&mut self) -> &mut Float {
        self.elements.index_mut(2)
    }

    pub fn get_mut_w(&mut self) -> &mut Float {
        self.elements.index_mut(3)
    }

    pub fn get_mut_underlying_x(&mut self) -> &mut f64 {
        &mut (self.get_mut_x()).0
    }

    pub fn get_mut_underlying_y(&mut self) -> &mut f64 {
        &mut (self.get_mut_y()).0
    }

    pub fn get_mut_underlying_z(&mut self) -> &mut f64 {
        &mut (self.get_mut_z()).0
    }

    pub fn get_mut_underlying_w(&mut self) -> &mut f64 {
        &mut (self.get_mut_w()).0
    }

    /// Calculates magnitude of Tuple
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// assert_eq!(Tuple::vector_from_underlying(1.0, 0.0, 0.0).magnitude(), 1.0);
    /// ```
    pub fn magnitude(&self) -> Float {
        self.elements.iter().map(|f| *f * *f).sum::<Float>().sqrt()
    }

    /// Yields the normalization of a Tuple
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// assert_eq!(Tuple::vector_from_underlying(1.0, 2.0, 3.0).normalize(), Tuple::vector_from_underlying(0.26726, 0.53452, 0.80178));
    /// assert_eq!(Tuple::vector_from_underlying(1.0, 0.0, 0.0).normalize().magnitude(), 1.0);
    /// ```
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }
}

impl std::ops::AddAssign for Tuple {
    /// AddAssigns two Tuples
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::new_from_underlying(3.0, -2.0, 5.0, 1.0);
    /// a += Tuple::new_from_underlying(-2.0, 3.0, 1.0, 0.0);
    /// assert_eq!(a, Tuple::new_from_underlying(1.0, 1.0, 6.0, 1.0));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.elements.iter_mut().enumerate().for_each(|(idx, val)| {
            *val += rhs.elements[idx];
        });
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    /// Adds two Tuples
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let a1 = Tuple::new_from_underlying(3.0, -2.0, 5.0, 1.0);
    /// let a2 = Tuple::new_from_underlying(-2.0, 3.0, 1.0, 0.0);
    /// assert_eq!(a1 + a2, Tuple::new_from_underlying(1.0, 1.0, 6.0, 1.0));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl std::ops::SubAssign for Tuple {
    /// SubAssigns two Tuples
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::point_from_underlying(3.0, 2.0, 1.0);
    /// a -= Tuple::point_from_underlying(5.0, 6.0, 7.0);
    /// assert_eq!(a, Tuple::vector_from_underlying(-2.0, -4.0, -6.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::point_from_underlying(3.0, 2.0, 1.0);
    /// a -= Tuple::vector_from_underlying(5.0, 6.0, 7.0);
    /// assert_eq!(a, Tuple::point_from_underlying(-2.0, -4.0, -6.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::vector_from_underlying(3.0, 2.0, 1.0);
    /// a -= Tuple::vector_from_underlying(5.0, 6.0, 7.0);
    /// assert_eq!(a, Tuple::vector_from_underlying(-2.0, -4.0, -6.0));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.elements.iter_mut().enumerate().for_each(|(idx, val)| {
            *val -= rhs.elements[idx];
        });
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    /// Subs two Tuples
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let p1 = Tuple::point_from_underlying(3.0, 2.0, 1.0);
    /// let p2 = Tuple::point_from_underlying(5.0, 6.0, 7.0);
    /// assert_eq!(p1 - p2, Tuple::vector_from_underlying(-2.0, -4.0, -6.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let p = Tuple::point_from_underlying(3.0, 2.0, 1.0);
    /// let v = Tuple::vector_from_underlying(5.0, 6.0, 7.0);
    /// assert_eq!(p - v, Tuple::point_from_underlying(-2.0, -4.0, -6.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let v1 = Tuple::vector_from_underlying(3.0, 2.0, 1.0);
    /// let v2 = Tuple::vector_from_underlying(5.0, 6.0, 7.0);
    /// assert_eq!(v1 - v2, Tuple::vector_from_underlying(-2.0, -4.0, -6.0));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    /// Negates Tuple
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(-a, Tuple::new_from_underlying(-1.0, 2.0, -3.0, 4.0));
    fn neg(self) -> Self::Output {
        Self::new(-self.get_x(), -self.get_y(), -self.get_z(), -self.get_w())
    }
}

impl std::ops::MulAssign<Float> for Tuple {
    /// MulAssign with Scalar
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// let mut a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// a *= Float(3.5);
    /// assert_eq!(a, Tuple::new_from_underlying(3.5, -7.0, 10.5, -14.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// let mut a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// a *= Float(0.5);
    /// assert_eq!(a, Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn mul_assign(&mut self, rhs: Float) {
        self.elements.iter_mut().for_each(|elem| {
            *elem *= rhs;
        });
    }
}

impl std::ops::MulAssign<UnderlyingFloat> for Tuple {
    /// MulAssign with Scalar as underlying type
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// a *= 3.5;
    /// assert_eq!(a, Tuple::new_from_underlying(3.5, -7.0, 10.5, -14.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// a *= 0.5;
    /// assert_eq!(a, Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn mul_assign(&mut self, rhs: UnderlyingFloat) {
        self.elements.iter_mut().for_each(|elem| {
            *elem *= Float(rhs);
        });
    }
}

impl std::ops::Mul<Float> for Tuple {
    type Output = Self;
    /// Mul with Scalar
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(a * Float(3.5), Tuple::new_from_underlying(3.5, -7.0, 10.5, -14.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(a * Float(0.5), Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn mul(self, rhs: Float) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl std::ops::Mul<UnderlyingFloat> for Tuple {
    type Output = Self;
    /// Mul with Scalar
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(a * 3.5, Tuple::new_from_underlying(3.5, -7.0, 10.5, -14.0));
    /// ```
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(a * 0.5, Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn mul(self, rhs: UnderlyingFloat) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl std::ops::DivAssign<Float> for Tuple {
    /// DivAssign by Scalar
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// let mut a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// a /= Float(2.0);
    /// assert_eq!(a, Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn div_assign(&mut self, rhs: Float) {
        self.elements.iter_mut().for_each(|elem| {
            *elem /= rhs;
        });
    }
}

impl std::ops::DivAssign<UnderlyingFloat> for Tuple {
    /// DivAssign by Scalar as underlying type
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let mut a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// a /= 2.0;
    /// assert_eq!(a, Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn div_assign(&mut self, rhs: UnderlyingFloat) {
        self.elements.iter_mut().for_each(|elem| {
            *elem /= Float(rhs);
        });
    }
}

impl std::ops::Div<Float> for Tuple {
    type Output = Self;
    /// Div by Scalar
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(a / Float(2.0), Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn div(self, rhs: Float) -> Self::Output {
        let mut res = self;
        res /= rhs;
        res
    }
}

impl std::ops::Div<UnderlyingFloat> for Tuple {
    type Output = Self;
    /// Div by Scalar
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::new_from_underlying(1.0, -2.0, 3.0, -4.0);
    /// assert_eq!(a / 2.0, Tuple::new_from_underlying(0.5, -1.0, 1.5, -2.0));
    /// ```
    fn div(self, rhs: UnderlyingFloat) -> Self::Output {
        let mut res = self;
        res /= rhs;
        res
    }
}

impl std::ops::Mul for Tuple {
    type Output = Float;

    /// Dot product of two tuples
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// use rtlib::float::Float;
    /// assert_eq!(Tuple::vector_from_underlying(1.0, 2.0, 3.0) * Tuple::vector_from_underlying(2.0, 3.0, 4.0), 20.0)
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        std::iter::zip(self.elements, rhs.elements)
            .map(|(l, r)| l * r)
            .sum()
    }
}

impl std::ops::Rem for Tuple {
    type Output = Tuple;

    /// Cross product of two tuples
    ///
    /// ```
    /// use rtlib::tuple::Tuple;
    /// let a = Tuple::vector_from_underlying(1.0, 2.0, 3.0);
    /// let b = Tuple::vector_from_underlying(2.0, 3.0, 4.0);
    /// assert_eq!(a % b, Tuple::vector_from_underlying(-1.0, 2.0, -1.0));
    /// assert_eq!(b % a, Tuple::vector_from_underlying(1.0, -2.0, 1.0));
    /// assert_eq!(a % b, -(b % a));
    /// ```
    fn rem(self, rhs: Self) -> Self::Output {
        Self::vector(
            self.get_y() * rhs.get_z() - self.get_z() * rhs.get_y(),
            self.get_z() * rhs.get_x() - self.get_x() * rhs.get_z(),
            self.get_x() * rhs.get_y() - self.get_y() * rhs.get_x(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_vec_from_zero() {
        let zero = Tuple::zero();
        let v = Tuple::vector_from_underlying(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector_from_underlying(-1.0, 2.0, -3.0));
    }

    #[test]
    fn magnitudes() {
        assert_eq!(
            Tuple::vector_from_underlying(1.0, 0.0, 0.0).magnitude(),
            1.0
        );
        assert_eq!(
            Tuple::vector_from_underlying(0.0, 1.0, 0.0).magnitude(),
            1.0
        );
        assert_eq!(
            Tuple::vector_from_underlying(0.0, 0.0, 1.0).magnitude(),
            1.0
        );
        assert_eq!(
            Tuple::vector_from_underlying(1.0, 2.0, 3.0).magnitude(),
            14.0f64.sqrt()
        );
        assert_eq!(
            Tuple::vector_from_underlying(-1.0, -2.0, -3.0).magnitude(),
            14.0f64.sqrt()
        );
    }

    #[test]
    fn normalizations() {
        assert_eq!(
            Tuple::vector_from_underlying(4.0, 0.0, 0.0).normalize(),
            Tuple::vector_from_underlying(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::vector_from_underlying(1.0, 0.0, 0.0).normalize(),
            Tuple::vector_from_underlying(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::vector_from_underlying(1.0, 2.0, 3.0).normalize(),
            Tuple::vector_from_underlying(0.26726, 0.53452, 0.80178)
        );
        assert_eq!(
            Tuple::vector_from_underlying(1.0, 0.0, 0.0)
                .normalize()
                .magnitude(),
            Float(1.0)
        );
    }

    #[test]
    fn display() {
        {
            let vec = Tuple::vector_from_underlying(1.0, 2.0, 3.0);
            assert_eq!(format!("{}", vec), "Vector(1, 2, 3)");
        }
        {
            let point = Tuple::point_from_underlying(1.0, 2.0, 3.0);
            assert_eq!(format!("{}", point), "Point(1, 2, 3)");
        }
        {
            let tuple = Tuple::new_from_underlying(1.0, 2.0, 3.0, 4.0);
            assert_eq!(format!("{}", tuple), "Tuple(1, 2, 3, 4)");
        }
    }

    #[test]
    fn mutation() {
        let mut vec = Tuple::vector_from_underlying(1.0, 2.0, 3.0);
        *vec.get_mut_underlying_x() = -1.0;
        *vec.get_mut_underlying_y() = -2.0;
        *vec.get_mut_underlying_z() = -3.0;
        *vec.get_mut_underlying_w() = 1.0;
        assert_eq!(vec, Tuple::point_from_underlying(-1.0, -2.0, -3.0));
    }
}
