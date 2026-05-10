/*
 * Heavily inspired by:
 * - https://github.com/malbernaz/bevy_lit
 */

//! Preparation [`RenderSystems`](bevy::render::RenderSystems).

use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        resource::Resource,
        system::{Query, Res, ResMut},
    },
    platform::collections::HashMap,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            BindGroupEntries, Buffer, BufferInitDescriptor, BufferUsages, PipelineCache,
        },
        renderer::{RenderDevice, RenderQueue},
        texture::GpuImage,
        view::ExtractedView,
    },
};
use bytemuck::cast_slice;

use crate::{mist::prelude::*, noise::prelude::*};

/// [`Buffer`]s mapped to [`MeshMist`] [`Entity`]s.
#[derive(Resource, Default)]
pub(super) struct MeshMistUniformBuffers(pub(super) HashMap<Entity, Buffer>);

/// Prepare [`MeshMistUniformBuffers`].
pub(super) fn prepare_mesh_mist_buffers(
    mist_query: Query<(Entity, &ExtractedMeshMist)>,
    mut buffers: ResMut<MeshMistUniformBuffers>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    buffers.0.clear();
    for (entity, mist) in &mist_query {
        let buffer = buffers.0.entry(entity).or_insert_with(|| {
            render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("mesh_mist_uniform_buffer"),
                contents: cast_slice(&[*mist]),
                usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            })
        });
        render_queue.write_buffer(buffer, 0, cast_slice(&[*mist]));
    }
}

/// Prepare [`MeshMistFragmentBindGroups`].
pub(super) fn prepare_mesh_mist_fragment_bind_groups(
    views: Query<&ExtractedView>,
    mist_query: Query<(Entity, &ExtractedMeshMistFrequency), With<ExtractedMeshMist>>,
    mut bind_groups: ResMut<MeshMistFragmentBindGroups>,
    buffers: Res<MeshMistUniformBuffers>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<MeshMistPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    mist_noise_map: Res<MistNoiseMap>,
) {
    bind_groups.0.clear();
    for extracted_view in &views {
        for (entity, scale) in &mist_query {
            let Some(noise_image) = mist_noise_map.0.get(&scale.0) else {
                continue;
            };
            let (Some(buffer), Some(noise_image)) =
                (buffers.0.get(&entity), gpu_images.get(noise_image.id()))
            else {
                continue;
            };

            let fragment_bind_group = render_device.create_bind_group(
                "mesh_mist_fragment_bind_group",
                &pipeline_cache.get_bind_group_layout(&pipeline.fragment_layout),
                &BindGroupEntries::sequential((
                    &noise_image.texture_view,
                    &pipeline.noise_sampler,
                    buffer.as_entire_binding(),
                )),
            );
            bind_groups.0.insert(
                (extracted_view.retained_view_entity, entity),
                fragment_bind_group,
            );
        }
    }
}
