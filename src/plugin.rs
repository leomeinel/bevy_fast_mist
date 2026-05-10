//! [`FastMistPlugin`] and related.
//!
//! ## Render stages
//!
//! 1. Render a mist map to a scalable texture.
//! 2. Compose from mist map to screen texture.

pub(crate) mod prelude {
    pub(crate) use super::FastMistSettings;
}

use bevy::{
    app::{App, Plugin, Update},
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    ecs::resource::Resource,
    render::{
        RenderApp,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::RenderGraphExt as _,
    },
};

use crate::{composite::prelude::*, mist::prelude::*, noise::prelude::*};

/// [`Plugin`] for fast 2D mist.
///
/// You can spawn [`MeshMist`]s to add mist to certain areas.
pub struct FastMistPlugin {
    /// Texture scale for any mist.
    ///
    /// Textures uses in rendering will be multiplied by this to get the mist texture resolution.
    pub texture_scale: f32,
}
impl Default for FastMistPlugin {
    fn default() -> Self {
        Self {
            texture_scale: 1. / 8.,
        }
    }
}
impl Plugin for FastMistPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FastMistSettings::from(self));
        app.init_resource::<MistNoiseMap>();

        app.add_plugins(ExtractResourcePlugin::<FastMistSettings>::default());
        app.add_plugins((MeshMistPlugin, MistCompositePlugin));

        app.add_systems(Update, super::noise::update_mist_noise_map);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_edges(
            Core2d,
            (
                Node2d::MainTransparentPass,
                MeshMistLabel,
                MistCompositeLabel,
                Node2d::EndMainPass,
            ),
        );
    }
}

/// Settings from [`FastMistPlugin`] as a [`Resource`].
///
/// This cannot be changed independently and should always be derived from [`FastMistPlugin`].
#[derive(Resource, Clone, Copy, ExtractResource)]
pub(crate) struct FastMistSettings {
    pub(crate) texture_scale: f32,
}
impl From<&FastMistPlugin> for FastMistSettings {
    fn from(plugin: &FastMistPlugin) -> Self {
        Self {
            texture_scale: plugin.texture_scale,
        }
    }
}
