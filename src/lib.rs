pub mod canvas;
pub mod color;
pub mod float;
pub mod tuple;

pub mod prelude {
    pub type UnderlyingFloat = f64;
    pub const EPSILON: UnderlyingFloat = 0.00001;

    pub use crate::canvas::coord_2d::Coord2D;
    pub use crate::canvas::Canvas;
    pub use crate::color::Color;
    pub use crate::float::Float;
    pub use crate::tuple::Tuple;
}
