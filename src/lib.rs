//! Simple 2D mist for Bevy focused on performance over features.

mod composite;
mod mist;
mod noise;
mod plugin;
mod utils;

pub mod prelude {
    pub use crate::mist::MeshMist;
    pub use crate::plugin::FastMistPlugin;
}
