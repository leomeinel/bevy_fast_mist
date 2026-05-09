//! [`ViewNode`]s for rendering mist to a texture from [`MeshMistTextures`].

use bevy::{
    ecs::{query::QueryItem, world::World},
    log::error,
    render::{
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_phase::ViewBinnedRenderPhases,
        render_resource::*,
        renderer::RenderContext,
        view::ExtractedView,
    },
};

use crate::mist::prelude::*;

/// [`ViewNode`] that renders mist to a texture from [`MeshMistTextures`].
#[derive(Default)]
pub(super) struct MeshMistNode;
impl ViewNode for MeshMistNode {
    type ViewQuery = &'static ExtractedView;

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        extracted_view: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.view_entity();
        let mist_phases = world.resource::<ViewBinnedRenderPhases<MeshMistPhase>>();
        let mist_textures = world.resource::<MeshMistTextures>();
        let (Some(mist_phase), Some(mist_texture)) = (
            mist_phases.get(&extracted_view.retained_view_entity),
            mist_textures.0.get(&extracted_view.retained_view_entity),
        ) else {
            return Ok(());
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("mesh_mist_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &mist_texture.default_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if let Err(err) = mist_phase.render(&mut render_pass, world, view_entity) {
            error!("Error for mist_phase in MeshMistNode {err:?}");
        }

        Ok(())
    }
}
