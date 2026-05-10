/*
 * Heavily inspired by:
 * - https://github.com/jgayfer/bevy_light_2d
 */

//! Extracted [`Component`]s and systems for extraction to the render world.

use bevy::{
    camera::{Camera, Camera2d},
    color::{Alpha as _, LinearRgba},
    ecs::{
        component::Component,
        entity::Entity,
        lifecycle::RemovedComponents,
        query::With,
        system::{Commands, Local, Query, Res, ResMut},
    },
    math::{FloatOrd, Vec2},
    mesh::Mesh2d,
    platform::collections::HashSet,
    render::{
        Extract, batching::gpu_preprocessing::GpuPreprocessingMode,
        render_phase::ViewBinnedRenderPhases, render_resource::ShaderType,
        sync_world::RenderEntity, view::RetainedViewEntity,
    },
    time::Time,
    utils::default,
};
use bytemuck::{Pod, Zeroable};

use crate::mist::prelude::*;

/// [`ShaderType`] that gets extracted to the render world for [`MeshLight2d`].
#[repr(C)]
#[derive(Component, Default, Clone, Copy, ShaderType, Debug, Pod, Zeroable)]
pub(crate) struct ExtractedMeshMist {
    pub(super) color: LinearRgba,
    pub(super) offset: Vec2,
    pub(super) alpha_bias: f32,
    pub(super) max_alpha: f32,
}
impl ExtractedMeshMist {
    fn with_scaled_offset(self, scale_factor: f32) -> Self {
        Self {
            offset: self.offset * scale_factor,
            ..self
        }
    }
}
impl From<MeshMist> for ExtractedMeshMist {
    fn from(mist: MeshMist) -> Self {
        Self {
            color: (mist.color.to_linear() * mist.intensity).with_alpha(1.),
            offset: mist.direction,
            alpha_bias: mist.alpha_bias,
            max_alpha: mist.max_alpha / (1. + mist.alpha_bias),
            ..default()
        }
    }
}

#[derive(Component)]
pub(super) struct ExtractedMeshMistFrequency(pub(super) FloatOrd);
impl From<MeshMist> for ExtractedMeshMistFrequency {
    fn from(mist: MeshMist) -> Self {
        Self(FloatOrd(mist.frequency))
    }
}

/// Extract [`RetainedViewEntity`]s to relevant render phases.
pub(super) fn extract_view_entities(
    mut mist_phases: ResMut<ViewBinnedRenderPhases<MeshMistPhase>>,
    cameras: Extract<Query<(Entity, &Camera), With<Camera2d>>>,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    live_entities.clear();
    for (main_entity, camera) in &cameras {
        if !camera.is_active {
            continue;
        }
        // NOTE: This is the main camera, so we use the first subview index (0)
        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
        mist_phases.prepare_for_new_frame(retained_view_entity, GpuPreprocessingMode::None);
        live_entities.insert(retained_view_entity);
    }

    mist_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
}

/// Extract [`MeshLight2d`] as [`ExtractedMeshLight2d`] to render world.
pub(super) fn extract_mesh_mists(
    mut removed_mist: Extract<RemovedComponents<MeshMist>>,
    mist_query: Extract<Query<(&RenderEntity, &MeshMist), With<Mesh2d>>>,
    render_entity_query: Extract<Query<&RenderEntity>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Remove old extracted components
    for entity in removed_mist.read() {
        let Ok(render_entity) = render_entity_query.get(entity) else {
            continue;
        };
        commands
            .entity(**render_entity)
            .remove::<(ExtractedMeshMist, ExtractedMeshMistFrequency)>();
    }

    // Insert new extracted components
    for (render_entity, mist) in &mist_query {
        commands.entity(**render_entity).insert((
            ExtractedMeshMist::from(*mist).with_scaled_offset(time.elapsed_secs_wrapped()),
            ExtractedMeshMistFrequency::from(*mist),
        ));
    }
}
