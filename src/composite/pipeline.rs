/*
 * Heavily inspired by:
 * - https://bevy.org/examples/shaders/custom-post-processing/
 */

//! Render pipelines for rendering lights to the screen texture.

use bevy::{
    asset::{AssetServer, load_embedded_asset},
    core_pipeline::FullscreenShader,
    ecs::{
        resource::Resource,
        system::{Commands, Res},
    },
    image::BevyDefault as _,
    render::{
        render_resource::{
            binding_types::{sampler, texture_2d},
            *,
        },
        renderer::RenderDevice,
    },
    utils::default,
};

/// Pipeline that multiplies a low resolution texture with the screen texture in the shader.
#[derive(Resource)]
pub(super) struct MistCompositePipeline {
    pub(super) fragment_layout: BindGroupLayoutDescriptor,
    pub(super) mesh_mist_sampler: Sampler,
    pub(super) pipeline_id: CachedRenderPipelineId,
}

/// Initialize [`MistCompositePipeline`].
pub(super) fn init_mist_composite_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
    render_device: Res<RenderDevice>,
) {
    let fragment_layout = BindGroupLayoutDescriptor::new(
        "mist_composite_fragment_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
            ),
        ),
    );

    let shader = load_embedded_asset!(asset_server.as_ref(), "mist_composite.wgsl");
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("mist_composite_pipeline".into()),
        layout: vec![fragment_layout.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader,
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::bevy_default(),
                blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });

    // NOTE: We are using linear sampling here to avoid pixelated mist
    let mist_sampler = render_device.create_sampler(&SamplerDescriptor {
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: FilterMode::Linear,
        ..default()
    });

    commands.insert_resource(MistCompositePipeline {
        fragment_layout,
        mesh_mist_sampler: mist_sampler,
        pipeline_id,
    });
}
