/*
 * Heavily inspired by:
 * - https://bevy.org/examples/shaders/custom-post-processing/
 * - https://github.com/jgayfer/bevy_light_2d
 */

//! [`ViewNode`]s for rendering lights to the screen texture.

use bevy::{
    ecs::{query::QueryItem, world::World},
    render::{
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::*,
        renderer::RenderContext,
        view::{ExtractedView, ViewTarget},
    },
};

use crate::{composite::prelude::*, mist::prelude::*};

/// [`ViewNode`] that renders to the screen texture.
///
/// ## Formula
///
/// (texture_output + ambient_color) * screen_texture.
///
/// ## Note
///
/// texture_output is from [`Light2dNode`].
#[derive(Default)]
pub(super) struct MistCompositeNode;
impl ViewNode for MistCompositeNode {
    type ViewQuery = (&'static ViewTarget, &'static ExtractedView);

    fn run(
        &self,
        _: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, extracted_view): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<MistCompositePipeline>();
        let mesh_mist_textures = world.resource::<MeshMistTextures>();
        let (Some(render_pipeline), Some(mesh_mist_texture)) = (
            pipeline_cache.get_render_pipeline(pipeline.pipeline_id),
            mesh_mist_textures
                .0
                .get(&extracted_view.retained_view_entity),
        ) else {
            return Ok(());
        };

        let fragment_bind_group = render_context.render_device().create_bind_group(
            "mist_composite_fragment_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipeline.fragment_layout),
            &BindGroupEntries::sequential((
                &mesh_mist_texture.default_view,
                &pipeline.mesh_mist_sampler,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("mist_composite_render_pass"),
            // TODO: Unsampled is probably incorrect, if so implement SpecializedRenderpipeline to modify msaa.
            color_attachments: &[Some(view_target.get_unsampled_color_attachment())],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &fragment_bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
