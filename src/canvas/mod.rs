pub mod coord_2d;

use crate::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    data: Vec<Color>,
    dim: Coord2D,
}

impl Canvas {
    /// Creates canvas with every pixel black
    ///
    /// ```
    /// use rtlib::canvas::Canvas;
    /// use rtlib::canvas::coord_2d::Coord2D;
    /// use rtlib::color::Color;
    /// let black = Color::new_from_underlying(0.0, 0.0, 0.0);
    /// let canvas = Canvas::new(Coord2D::new(23, 42));
    /// assert!(canvas.iter().all(|(coord, color)| *color == black));
    /// ```
    pub fn new(dim: Coord2D) -> Self {
        Self {
            data: vec![Color::new_from_underlying(0.0, 0.0, 0.0); dim.x * dim.y],
            dim,
        }
    }

    /// Gets the dimension of canvas
    ///
    /// ```
    /// use rtlib::canvas::Canvas;
    /// use rtlib::canvas::coord_2d::Coord2D;
    /// let canvas = Canvas::new(Coord2D::new(23, 42));
    /// assert_eq!(canvas.get_dim(), Coord2D::new(23, 42));
    /// ```
    pub fn get_dim(&self) -> Coord2D {
        self.dim
    }

    /// Gets the size of canvas
    ///
    /// ```
    /// use rtlib::canvas::Canvas;
    /// use rtlib::canvas::coord_2d::Coord2D;
    /// let canvas = Canvas::new(Coord2D::new(23, 42));
    /// assert_eq!(canvas.size(), 966);
    /// ```
    pub fn size(&self) -> usize {
        self.dim.x * self.dim.y
    }

    /// Iterates over coordinates and elements
    ///
    /// ```
    /// use rtlib::canvas::Canvas;
    /// use rtlib::canvas::coord_2d::Coord2D;
    /// use rtlib::color::Color;
    /// let mut canvas = Canvas::new(Coord2D::new(2, 2));
    /// canvas[Coord2D::new(0, 0)] = Color::new_from_underlying(0.1, 0.1, 0.1);
    /// canvas[Coord2D::new(1, 0)] = Color::new_from_underlying(0.2, 0.2, 0.2);
    /// canvas[Coord2D::new(0, 1)] = Color::new_from_underlying(0.3, 0.3, 0.3);
    /// canvas[Coord2D::new(1, 1)] = Color::new_from_underlying(0.4, 0.4, 0.4);
    /// let vec: Vec<_> = canvas.iter().collect();
    /// assert_eq!(vec, [(Coord2D::new(0, 0), &Color::new_from_underlying(0.1, 0.1, 0.1)), (Coord2D::new(1, 0), &Color::new_from_underlying(0.2, 0.2, 0.2)), (Coord2D::new(0, 1), &Color::new_from_underlying(0.3, 0.3, 0.3)), (Coord2D::new(1, 1), &Color::new_from_underlying(0.4, 0.4, 0.4))]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (Coord2D, &Color)> {
        let dim = self.dim;
        self.data
            .iter()
            .enumerate()
            .map(move |(index, data)| (Coord2D::new_from_index(index, &dim), data))
    }

    /// Iterates over coordinates and mutable elements
    ///
    /// ```
    /// use rtlib::canvas::Canvas;
    /// use rtlib::canvas::coord_2d::Coord2D;
    /// use rtlib::color::Color;
    /// let mut canvas = Canvas::new(Coord2D::new(2, 2));
    /// canvas[Coord2D::new(0, 0)] = Color::new_from_underlying(0.1, 0.1, 0.1);
    /// canvas[Coord2D::new(1, 0)] = Color::new_from_underlying(0.2, 0.2, 0.2);
    /// canvas[Coord2D::new(0, 1)] = Color::new_from_underlying(0.3, 0.3, 0.3);
    /// canvas[Coord2D::new(1, 1)] = Color::new_from_underlying(0.4, 0.4, 0.4);
    /// canvas.iter_mut().for_each(|(coord, color)| *color *= 2.0);
    /// let vec: Vec<_> = canvas.iter().collect();
    /// assert_eq!(vec, [(Coord2D::new(0, 0), &Color::new_from_underlying(0.2, 0.2, 0.2)), (Coord2D::new(1, 0), &Color::new_from_underlying(0.4, 0.4, 0.4)), (Coord2D::new(0, 1), &Color::new_from_underlying(0.6, 0.6, 0.6)), (Coord2D::new(1, 1), &Color::new_from_underlying(0.8, 0.8, 0.8))]);
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Coord2D, &mut Color)> {
        let dim = self.dim;
        self.data
            .iter_mut()
            .enumerate()
            .map(move |(index, data)| (Coord2D::new_from_index(index, &dim), data))
    }

    fn u8_representation(float: Float) -> u8 {
        ((float.0 * 255.0).round() as i32).clamp(0, 255) as u8
    }

    fn write_header(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        write!(f, "P3\n{} {}\n255\n", self.dim.x, self.dim.y)
    }

    fn write_data(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut line_width = 0;
        let mut do_write = |float: Float, force_reset: bool| -> Result<(), std::io::Error> {
            let s = format!("{}", Self::u8_representation(float));
            if force_reset || line_width + 1 + s.len() > 70 {
                writeln!(f)?;
                line_width = 0;
            }
            if line_width > 0 {
                write!(f, " ")?;
                line_width += 1;
            }
            write!(f, "{}", s)?;
            line_width += s.len();
            Ok(())
        };
        for (coord, color) in self.iter() {
            do_write(color.get_r(), coord.x == 0 && coord.y > 0)?;
            do_write(color.get_g(), false)?;
            do_write(color.get_b(), false)?;
        }

        writeln!(f)
    }

    fn write_binary_header(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        write!(f, "P6\n{} {}\n255\n", self.dim.x, self.dim.y)
    }

    fn write_binary_data(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut do_write = |float: Float| -> Result<(), std::io::Error> {
            f.write_all(&[Self::u8_representation(float)])
        };
        for (_, color) in self.iter() {
            do_write(color.get_r())?;
            do_write(color.get_g())?;
            do_write(color.get_b())?;
        }

        writeln!(f)
    }

    pub fn write_ppm(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.write_header(f)?;
        self.write_data(f)
    }

    pub fn write_binary_ppm(&self, f: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.write_binary_header(f)?;
        self.write_binary_data(f)
    }
}

impl Index<usize> for Canvas {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<Coord2D> for Canvas {
    type Output = Color;
    fn index(&self, index: Coord2D) -> &Self::Output {
        let index = index.to_index(&self.dim);
        &self.data[index]
    }
}

impl IndexMut<Coord2D> for Canvas {
    fn index_mut(&mut self, index: Coord2D) -> &mut Self::Output {
        let index = index.to_index(&self.dim);
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn draw_on_canvas() {
        let mut canvas = Canvas::new(Coord2D::new(10, 10));
        let red = Color::new_from_underlying(1.0, 0.0, 0.0);
        let black = Color::new_from_underlying(0.0, 0.0, 0.0);
        let changed_coord = Coord2D::new(3, 2);
        canvas[changed_coord] = red;
        canvas.iter().for_each(|(coord, color)| {
            assert_eq!(
                *color,
                match coord {
                    c if c == changed_coord => red,
                    _ => black,
                }
            );
        });
    }

    #[test]
    fn write_header() {
        let canvas = Canvas::new(Coord2D::new(5, 3));
        let mut data: Vec<u8> = Vec::new();
        let res = canvas.write_header(&mut data);
        assert!(res.is_ok());
        let conv = std::str::from_utf8(&data);
        assert!(conv.is_ok());
        let str = conv.unwrap();
        assert_eq!(str, "P3\n5 3\n255\n");
    }

    #[test]
    fn write_data() {
        let mut canvas = Canvas::new(Coord2D::new(5, 3));
        canvas[Coord2D::new(0, 0)] = Color::new_from_underlying(1.5, 0.0, 0.0);
        canvas[Coord2D::new(2, 1)] = Color::new_from_underlying(0.0, 0.5, 0.0);
        canvas[Coord2D::new(4, 2)] = Color::new_from_underlying(-0.5, 0.0, 1.0);
        let mut data: Vec<u8> = Vec::new();
        let res = canvas.write_data(&mut data);
        assert!(res.is_ok());
        let conv = std::str::from_utf8(&data);
        assert!(conv.is_ok());
        let str = conv.unwrap();
        assert_eq!(
            str,
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 \
            0 0 0 0 0 0 0 0 0 0 255\n"
        );
    }

    #[test]
    fn write_data_long_lines() {
        let mut canvas = Canvas::new(Coord2D::new(10, 2));
        canvas
            .iter_mut()
            .for_each(|(_, color)| *color = Color::new_from_underlying(1.0, 0.8, 0.6));
        let mut data: Vec<u8> = Vec::new();
        let res = canvas.write_data(&mut data);
        assert!(res.is_ok());
        let conv = std::str::from_utf8(&data);
        assert!(conv.is_ok());
        let str = conv.unwrap();
        assert_eq!(
            str,
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 \
        255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 \
        204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        );
    }

    #[test]
    fn write_ppm() {
        let canvas = Canvas::new(Coord2D::new(5, 3));
        let mut data: Vec<u8> = Vec::new();
        let res = canvas.write_ppm(&mut data);
        assert!(res.is_ok());
        let conv = std::str::from_utf8(&data);
        assert!(conv.is_ok());
        let str = conv.unwrap();
        assert_eq!(str.chars().rev().next().unwrap(), '\n');
    }
}
