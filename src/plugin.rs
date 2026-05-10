//! [`FastMistPlugin`] and related.

use bevy::app::{App, Plugin};

use crate::mist::prelude::*;

/// [`Plugin`] for fast 2D mist.
///
/// You can spawn [`MeshMist`]s to add mist to certain areas.
pub struct FastMistPlugin;
impl Plugin for FastMistPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshMistPlugin);
    }
}
