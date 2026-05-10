//! Simple moving 2D mist for Bevy focused on performance over features.

mod mist;
mod noise;
mod plugin;

pub mod prelude {
    pub use crate::mist::MeshMist;
    pub use crate::plugin::FastMistPlugin;
}
