use crate::prelude::*;
use std::fmt::Formatter;
use std::ops::IndexMut;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub elements: [Float; 3],
}

impl std::fmt::Display for Color {
    /// Formats a Color for displaying
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let a = format!("{}", Color::new_from_underlying(0.2, 0.3, 0.4));
    /// assert_eq!(a, "RGB(0.2, 0.3, 0.4)");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB({}, {}, {})",
            self.elements[0], self.elements[1], self.elements[2]
        )
    }
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self {
            elements: [r, g, b],
        }
    }

    /// Constructs a Color from underlying types
    ///
    /// ```
    /// use rtlib::color::Color;
    /// use rtlib::float::Float;
    /// let c = Color::new_from_underlying(-0.5, 0.4, 1.7);
    /// assert_eq!(c.get_r(), Float(-0.5));
    /// assert_eq!(c.get_g(), Float(0.4));
    /// assert_eq!(c.get_b(), Float(1.7));
    /// ```
    pub fn new_from_underlying(r: UnderlyingFloat, g: UnderlyingFloat, b: UnderlyingFloat) -> Self {
        Self::new(Float(r), Float(g), Float(b))
    }

    pub fn get_r(&self) -> Float {
        self.elements[0]
    }

    pub fn get_g(&self) -> Float {
        self.elements[1]
    }

    pub fn get_b(&self) -> Float {
        self.elements[2]
    }

    pub fn get_underlying_r(&self) -> f64 {
        self.get_r().0
    }

    pub fn get_underlying_g(&self) -> f64 {
        self.get_g().0
    }

    pub fn get_underlying_b(&self) -> f64 {
        self.get_b().0
    }

    pub fn get_mut_r(&mut self) -> &mut Float {
        self.elements.index_mut(0)
    }

    pub fn get_mut_g(&mut self) -> &mut Float {
        self.elements.index_mut(1)
    }

    pub fn get_mut_b(&mut self) -> &mut Float {
        self.elements.index_mut(2)
    }

    pub fn get_mut_underlying_r(&mut self) -> &mut f64 {
        &mut self.get_mut_r().0
    }

    pub fn get_mut_underlying_g(&mut self) -> &mut f64 {
        &mut self.get_mut_g().0
    }

    pub fn get_mut_underlying_b(&mut self) -> &mut f64 {
        &mut self.get_mut_b().0
    }
}

impl std::ops::AddAssign for Color {
    /// AddAssigns two Colors
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let mut c = Color::new_from_underlying(0.9, 0.6, 0.75);
    /// c += Color::new_from_underlying(0.7, 0.1, 0.25);
    /// assert_eq!(c, Color::new_from_underlying(1.6, 0.7, 1.0));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.elements
            .iter_mut()
            .enumerate()
            .for_each(|(idx, elem)| *elem += rhs.elements[idx]);
    }
}

impl std::ops::Add for Color {
    type Output = Self;
    /// Adds two Colors
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let c1 = Color::new_from_underlying(0.9, 0.6, 0.75);
    /// let c2 = Color::new_from_underlying(0.7, 0.1, 0.25);
    /// assert_eq!(c1 + c2, Color::new_from_underlying(1.6, 0.7, 1.0));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl std::ops::SubAssign for Color {
    /// SubAssigns two Colors
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let mut c = Color::new_from_underlying(0.9, 0.6, 0.75);
    /// c -= Color::new_from_underlying(0.7, 0.1, 0.25);
    /// assert_eq!(c, Color::new_from_underlying(0.2, 0.5, 0.5));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.elements
            .iter_mut()
            .enumerate()
            .for_each(|(idx, elem)| *elem -= rhs.elements[idx]);
    }
}

impl std::ops::Sub for Color {
    type Output = Self;
    /// Subs two Colors
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let c1 = Color::new_from_underlying(0.9, 0.6, 0.75);
    /// let c2 = Color::new_from_underlying(0.7, 0.1, 0.25);
    /// assert_eq!(c1 - c2, Color::new_from_underlying(0.2, 0.5, 0.5));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl std::ops::MulAssign for Color {
    /// MulAssigns two Colors
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let mut c = Color::new_from_underlying(1.0, 0.2, 0.4);
    /// c *= Color::new_from_underlying(0.9, 1.0, 0.1);
    /// assert_eq!(c, Color::new_from_underlying(0.9, 0.2, 0.04));
    /// ```
    fn mul_assign(&mut self, rhs: Self) {
        self.elements
            .iter_mut()
            .enumerate()
            .for_each(|(idx, elem)| *elem *= rhs.elements[idx]);
    }
}

impl std::ops::Mul for Color {
    type Output = Self;
    /// Muls two Colors
    ///
    /// ```
    /// use rtlib::color::Color;
    /// let c1 = Color::new_from_underlying(1.0, 0.2, 0.4);
    /// let c2 = Color::new_from_underlying(0.9, 1.0, 0.1);
    /// assert_eq!(c1 * c2, Color::new_from_underlying(0.9, 0.2, 0.04));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl std::ops::MulAssign<Float> for Color {
    /// MulAssigns Color with scalar
    ///
    /// ```
    /// use rtlib::color::Color;
    /// use rtlib::float::Float;
    /// let mut c = Color::new_from_underlying(0.2, 0.3, 0.4);
    /// c *= Float(2.0);
    /// assert_eq!(c, Color::new_from_underlying(0.4, 0.6, 0.8));
    /// ```
    fn mul_assign(&mut self, rhs: Float) {
        self.elements.iter_mut().for_each(|elem| *elem *= rhs);
    }
}

impl std::ops::Mul<Float> for Color {
    type Output = Self;
    /// Muls Color with scalar
    ///
    /// ```
    /// use rtlib::color::Color;
    /// use rtlib::float::Float;
    /// let c = Color::new_from_underlying(0.2, 0.3, 0.4);
    /// assert_eq!(c * Float(2.0), Color::new_from_underlying(0.4, 0.6, 0.8));
    /// ```
    fn mul(self, rhs: Float) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl std::ops::MulAssign<UnderlyingFloat> for Color {
    /// MulAssigns Color with scalar
    ///
    /// ```
    /// use rtlib::color::Color;
    /// use rtlib::float::Float;
    /// let mut c = Color::new_from_underlying(0.2, 0.3, 0.4);
    /// c *= 2.0;
    /// assert_eq!(c, Color::new_from_underlying(0.4, 0.6, 0.8));
    /// ```
    fn mul_assign(&mut self, rhs: UnderlyingFloat) {
        self.elements
            .iter_mut()
            .for_each(|elem| *elem *= Float(rhs));
    }
}

impl std::ops::Mul<UnderlyingFloat> for Color {
    type Output = Self;
    /// Muls Color with scalar
    ///
    /// ```
    /// use rtlib::color::Color;
    /// use rtlib::float::Float;
    /// let c = Color::new_from_underlying(0.2, 0.3, 0.4);
    /// assert_eq!(c * 2.0, Color::new_from_underlying(0.4, 0.6, 0.8));
    /// ```
    fn mul(self, rhs: UnderlyingFloat) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}
