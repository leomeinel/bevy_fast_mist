/*
 * Heavily inspired by:
 * - https://bevy.org/examples/shaders/custom-post-processing/
 */

//! Render pipelines for rendering mist to the screen texture.

use bevy::{
    asset::{AssetServer, Handle, load_embedded_asset},
    ecs::{
        resource::Resource,
        system::{Commands, Res},
    },
    image::BevyDefault,
    mesh::MeshVertexBufferLayoutRef,
    render::{
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::RenderDevice,
    },
    shader::Shader,
    sprite_render::{Mesh2dPipeline, Mesh2dPipelineKey},
    utils::default,
};

use crate::mist::prelude::*;

/// Pipeline that computes mist in the shader.
#[derive(Resource)]
pub(super) struct MeshMistPipeline {
    pub(super) mesh_pipeline: Mesh2dPipeline,
    pub(super) fragment_layout: BindGroupLayoutDescriptor,
    pub(super) noise_sampler: Sampler,
    pub(super) shader: Handle<Shader>,
}
impl SpecializedMeshPipeline for MeshMistPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor.label = Some("mesh_mist_pipeline".into());
        descriptor.layout.push(self.fragment_layout.clone());

        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment.shader = self.shader.clone();
        fragment.targets = vec![Some(ColorTargetState {
            format: TextureFormat::bevy_default(),
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        })];

        descriptor.multisample = MultisampleState::default();
        descriptor.depth_stencil = None;

        Ok(descriptor)
    }
}

/// Initialize [`MeshMistPipeline`].
pub(super) fn init_mesh_mist_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mesh_pipeline: Res<Mesh2dPipeline>,
    render_device: Res<RenderDevice>,
) {
    let fragment_layout = BindGroupLayoutDescriptor::new(
        "mesh_mist_fragment_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<ExtractedMeshMist>(false),
            ),
        ),
    );

    let noise_sampler = render_device.create_sampler(&SamplerDescriptor {
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: FilterMode::Linear,
        ..default()
    });

    commands.insert_resource(MeshMistPipeline {
        mesh_pipeline: mesh_pipeline.clone(),
        fragment_layout,
        noise_sampler,
        shader: load_embedded_asset!(asset_server.as_ref(), "mesh_mist.wgsl"),
    });
}
