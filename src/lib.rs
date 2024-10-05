pub mod float;
pub mod tuple;

pub mod prelude {
    pub type UnderlyingFloat = f64;
    pub const EPSILON: UnderlyingFloat = 0.00001;

    pub use crate::float::Float;
    pub use crate::tuple::Tuple;
}
