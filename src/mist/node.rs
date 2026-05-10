//! [`ViewNode`]s for rendering mist to the screen texture.

use bevy::{
    ecs::{query::QueryItem, world::World},
    log::error,
    render::{
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_phase::ViewBinnedRenderPhases,
        render_resource::*,
        renderer::RenderContext,
        view::{ExtractedView, ViewTarget},
    },
};

use crate::mist::prelude::*;

/// [`ViewNode`] that renders mist to the screen texture.
#[derive(Default)]
pub(super) struct MeshMistNode;
impl ViewNode for MeshMistNode {
    type ViewQuery = (&'static ViewTarget, &'static ExtractedView);

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, extracted_view): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.view_entity();
        let mist_phases = world.resource::<ViewBinnedRenderPhases<MeshMistPhase>>();
        let Some(mist_phase) = mist_phases.get(&extracted_view.retained_view_entity) else {
            return Ok(());
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("mesh_mist_render_pass"),
            // NOTE: I'm not entirely sure if using unsampled here is correct.
            color_attachments: &[Some(view_target.get_unsampled_color_attachment())],
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
