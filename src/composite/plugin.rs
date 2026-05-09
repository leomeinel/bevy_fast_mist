/*
 * Heavily inspired by:
 * - https://bevy.org/examples/shaders/custom-post-processing/
 */

//! [`Plugin`] for rendering lights to the screen texture.

use bevy::{
    app::{App, Plugin},
    asset::embedded_asset,
    core_pipeline::core_2d::graph::Core2d,
    render::{
        RenderApp, RenderStartup,
        render_graph::{RenderGraphExt, RenderLabel, ViewNodeRunner},
    },
};

use crate::composite::prelude::*;

/// [`Plugin`] for rendering lights to the screen texture.
pub(crate) struct MistCompositePlugin;
impl Plugin for MistCompositePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "mist_composite.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(RenderStartup, super::pipeline::init_mist_composite_pipeline);

        render_app
            .add_render_graph_node::<ViewNodeRunner<MistCompositeNode>>(Core2d, MistCompositeLabel);
    }
}

/// Label for render graph edges for [`Light2dCompositeNode`].
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub(crate) struct MistCompositeLabel;
