use std::fmt::Formatter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord2D {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Coord2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord2D({}, {})", self.x, self.y)
    }
}

impl Coord2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn new_from_index(index: usize, dim: &Coord2D) -> Self {
        Self::new(index % dim.x, index / dim.x)
    }

    pub fn to_index(&self, dim: &Coord2D) -> usize {
        self.x + dim.x * self.y
    }
}
