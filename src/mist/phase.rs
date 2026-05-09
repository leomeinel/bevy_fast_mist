/*
 * Heavily inspired by:
 * - https://bevy.org/examples/shaders/custom-render-phase/
 * - https://github.com/bevyengine/bevy/blob/main/crates/bevy_core_pipeline/src/core_2d/mod.rs
 */

//! [`PhaseItem`]s and related for mist rendering from [`MeshMist`].

use std::ops::Range;

use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Query, Res, ResMut, SystemChangeTick},
    },
    log::error,
    mesh::Mesh2d,
    render::{
        mesh::RenderMesh,
        render_asset::RenderAssets,
        render_phase::{
            BinnedPhaseItem, BinnedRenderPhaseType, CachedRenderPipelinePhaseItem, DrawFunctionId,
            DrawFunctions, InputUniformIndex, PhaseItem, PhaseItemBatchSetKey, PhaseItemExtraIndex,
            SetItemPipeline, ViewBinnedRenderPhases,
        },
        render_resource::{CachedRenderPipelineId, PipelineCache, SpecializedMeshPipelines},
        sync_world::MainEntity,
        view::{ExtractedView, Msaa, RenderVisibleEntities},
    },
    sprite_render::{
        DrawMesh2d, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
};

use crate::mist::prelude::*;

/// [`PhaseItem`] drawn in the render phase for mist rendering from [`MeshMist`].
pub(crate) struct MeshMistPhase {
    #[allow(dead_code)]
    pub(crate) batch_set_key: MeshMistBatchSetKey,
    pub(crate) bin_key: MeshMistBinKey,
    pub(crate) representative_entity: (Entity, MainEntity),
    pub(crate) batch_range: Range<u32>,
    pub(crate) extra_index: PhaseItemExtraIndex,
}
impl PhaseItem for MeshMistPhase {
    #[inline]
    fn entity(&self) -> Entity {
        self.representative_entity.0
    }
    #[inline]
    fn main_entity(&self) -> MainEntity {
        self.representative_entity.1
    }
    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.bin_key.draw_function
    }
    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }
    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }
    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }
    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}
impl BinnedPhaseItem for MeshMistPhase {
    type BinKey = MeshMistBinKey;

    type BatchSetKey = MeshMistBatchSetKey;

    fn new(
        batch_set_key: Self::BatchSetKey,
        bin_key: Self::BinKey,
        representative_entity: (Entity, MainEntity),
        batch_range: Range<u32>,
        extra_index: PhaseItemExtraIndex,
    ) -> Self {
        Self {
            batch_set_key,
            bin_key,
            representative_entity,
            batch_range,
            extra_index,
        }
    }
}
impl CachedRenderPipelinePhaseItem for MeshMistPhase {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.bin_key.pipeline
    }
}

/// Batch set key for [`MeshMistPhase`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct MeshMistBatchSetKey {
    indexed: bool,
}
impl PhaseItemBatchSetKey for MeshMistBatchSetKey {
    fn indexed(&self) -> bool {
        self.indexed
    }
}

/// Bin key for [`MeshMistPhase`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshMistBinKey {
    pipeline: CachedRenderPipelineId,
    draw_function: DrawFunctionId,
}

/// Draw function for mist rendering from [`MeshMist`].
pub(super) type DrawMeshMist = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    SetMeshMistFragmentBindGroup<2>,
    DrawMesh2d,
);

/// Queue drawable entities for [`ViewBinnedRenderPhases<MeshMistPhase>`].
pub(super) fn queue_mesh_mists(
    mut views: Query<(&ExtractedView, &RenderVisibleEntities, &Msaa)>,
    has_marker: Query<(), With<ExtractedMeshMist>>,
    mut render_phases: ResMut<ViewBinnedRenderPhases<MeshMistPhase>>,
    mut pipelines: ResMut<SpecializedMeshPipelines<MeshMistPipeline>>,
    draw_functions: Res<DrawFunctions<MeshMistPhase>>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<MeshMistPipeline>,
    render_meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderMesh2dInstances>,
    system_change_tick: SystemChangeTick,
) {
    let draw_function = draw_functions.read().id::<DrawMeshMist>();

    for (view, visible_entities, msaa) in &mut views {
        let Some(phase) = render_phases.get_mut(&view.retained_view_entity) else {
            continue;
        };
        let view_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
            | Mesh2dPipelineKey::from_hdr(view.hdr);

        for (render_entity, visible_entity) in visible_entities.iter::<Mesh2d>() {
            if has_marker.get(*render_entity).is_err() {
                continue;
            }
            let Some(mesh_instance) = render_mesh_instances.get(visible_entity) else {
                continue;
            };
            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };

            let mesh_key =
                view_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline = pipelines.specialize(&pipeline_cache, &pipeline, mesh_key, &mesh.layout);
            let pipeline = match pipeline {
                Ok(id) => id,
                Err(err) => {
                    error!("{}", err);
                    continue;
                }
            };
            let batch_set_key = MeshMistBatchSetKey {
                indexed: mesh.indexed(),
            };
            let bin_key = MeshMistBinKey {
                pipeline,
                draw_function,
            };
            phase.add(
                batch_set_key,
                bin_key,
                (*render_entity, *visible_entity),
                InputUniformIndex::default(),
                // NOTE: We can't use `BinnedRenderPhaseType::BatchableMesh` because we are passing per object uniforms.
                BinnedRenderPhaseType::UnbatchableMesh,
                system_change_tick.last_run(),
            );
        }
    }
}
