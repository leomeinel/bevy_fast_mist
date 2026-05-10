//! Different mist types and modules for rendering.
//!
//! This renders mist via [`MeshMistMaterial`](crate::mist::prelude::MeshMistMaterial).

mod material;
mod noise;
mod plugin;

pub(super) mod prelude {
    pub(crate) use super::MeshMist;
    pub(super) use super::material::MeshMistMaterial;
    pub(super) use super::noise::MistNoiseMap;
    pub(crate) use super::plugin::MeshMistPlugin;
}

use bevy::{
    color::Color, ecs::component::Component, math::Vec2, reflect::Reflect,
    render::sync_world::SyncToRenderWorld,
};

/// Mesh mist for area mist in a 2D environment.
///
/// This is meant to be added to a [`Mesh2d`](bevy::mesh::Mesh2d) which will determine the mists shape.
///
/// ## Formula
///
/// color = [`color`](Self::color) * [`intensity`](Self::intensity) * `alpha`.
///
/// ## Note
///
/// - `alpha` is influenced by two separate noise textures using [`frequency`](Self::frequency) and [`direction`](Self::direction).
#[derive(Component, Reflect, Clone, Copy)]
#[require(SyncToRenderWorld)]
pub struct MeshMist {
    /// The [`Color`] of the mist.
    pub color: Color,
    /// The intensity of the mist.
    pub intensity: f32,
    /// The frequency of the mist.
    ///
    /// This determines the size of the mist-filled regions.
    pub frequency: f32,
    /// The direction of the mist.
    ///
    /// This determines the movement direction of the mist.
    pub direction: Vec2,
    /// The alpha bias of the mist.
    ///
    /// This determines the minimum rendered value of the mist noise by being added directly to it.
    ///
    /// ## Note
    ///
    /// This is meant to be in the range of `0.0..-1.0`.
    pub alpha_bias: f32,
    /// This determines the absolute max alpha of the mist.
    pub max_alpha: f32,
}
impl Default for MeshMist {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.,
            frequency: 3.,
            direction: Vec2::new(4., 1.),
            alpha_bias: -0.3,
            max_alpha: 0.6,
        }
    }
}
