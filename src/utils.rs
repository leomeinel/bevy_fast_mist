//! Utilities to be used in the crate.

mod color;

pub(crate) mod prelude {
    pub(crate) use super::color::ColorExt;
}
