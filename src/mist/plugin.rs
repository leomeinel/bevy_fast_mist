/*
 * Heavily inspired by:
 * - https://bevy.org/examples/shaders/custom-post-processing/
 */

//! [`Plugin`] for rendering mist to a scaled texture.

use bevy::{
    app::{App, Plugin},
    asset::embedded_asset,
    core_pipeline::core_2d::graph::Core2d,
    ecs::schedule::IntoScheduleConfigs as _,
    render::{
        ExtractSchedule, Render, RenderApp, RenderDebugFlags, RenderStartup, RenderSystems,
        extract_resource::ExtractResourcePlugin,
        render_graph::{RenderGraphExt, RenderLabel, ViewNodeRunner},
        render_phase::{
            AddRenderCommand as _, BinnedRenderPhasePlugin, DrawFunctions, ViewBinnedRenderPhases,
        },
        render_resource::SpecializedMeshPipelines,
    },
    shader::load_shader_library,
    sprite_render::{Mesh2dPipeline, init_mesh_2d_pipeline},
};

use crate::{mist::prelude::*, noise::prelude::*};

/// [`Plugin`] for rendering mist to the screen texture.
pub(crate) struct MeshMistPlugin;
impl Plugin for MeshMistPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "types.wgsl");
        embedded_asset!(app, "mesh_mist.wgsl");

        app.add_plugins((
            BinnedRenderPhasePlugin::<MeshMistPhase, Mesh2dPipeline>::new(
                RenderDebugFlags::default(),
            ),
            ExtractResourcePlugin::<MistNoiseMap>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<DrawFunctions<MeshMistPhase>>()
            .init_resource::<SpecializedMeshPipelines<MeshMistPipeline>>()
            .init_resource::<ViewBinnedRenderPhases<MeshMistPhase>>()
            .init_resource::<MeshMistFragmentBindGroups>()
            .init_resource::<MeshMistUniformBuffers>();

        render_app.add_render_command::<MeshMistPhase, DrawMeshMist>();

        render_app.add_systems(
            RenderStartup,
            super::pipeline::init_mesh_mist_pipeline.after(init_mesh_2d_pipeline),
        );

        render_app.add_systems(
            ExtractSchedule,
            (
                super::extract::extract_view_entities,
                super::extract::extract_mesh_mists,
            ),
        );

        render_app.add_systems(
            Render,
            (
                super::phase::queue_mesh_mists.in_set(RenderSystems::Queue),
                (
                    super::prepare::prepare_mesh_mist_buffers,
                    super::prepare::prepare_mesh_mist_fragment_bind_groups,
                )
                    .chain()
                    .in_set(RenderSystems::PrepareResources),
            ),
        );

        render_app.add_render_graph_node::<ViewNodeRunner<MeshMistNode>>(Core2d, MeshMistLabel);
    }
}

/// Label for render graph edges for [`MeshMistNode`].
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub(crate) struct MeshMistLabel;
