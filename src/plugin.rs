//! [`FastMistPlugin`] and related.
//!
//! ## Render stages
//!
//! 1. Render a mist map to a scalable texture.
//! 2. Compose from mist map to screen texture.

use bevy::{
    app::{App, Plugin, Update},
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    render::{RenderApp, render_graph::RenderGraphExt as _},
};

use crate::{mist::prelude::*, noise::prelude::*};

/// [`Plugin`] for fast 2D mist.
///
/// You can spawn [`MeshMist`]s to add mist to certain areas.
pub struct FastMistPlugin;
impl Plugin for FastMistPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MistNoiseMap>();

        app.add_plugins(MeshMistPlugin);

        app.add_systems(Update, super::noise::update_mist_noise_map);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_edges(
            Core2d,
            (
                Node2d::MainTransparentPass,
                MeshMistLabel,
                Node2d::EndMainPass,
            ),
        );
    }
}
