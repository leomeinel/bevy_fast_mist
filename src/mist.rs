//! Different mist types and modules for rendering.
//!
//! This renders a mist map to a scalable texture.
//!
//! This is the first render stage of [`FastMistPlugin`](crate::prelude::FastMistPlugin).

mod extract;
mod node;
mod phase;
mod pipeline;
mod plugin;
mod prepare;

pub(super) mod prelude {
    pub(crate) use super::MeshMist;
    pub(super) use super::extract::ExtractedMeshMist;
    pub(super) use super::node::MeshMistNode;
    pub(super) use super::phase::DrawMeshMist;
    pub(crate) use super::phase::MeshMistPhase;
    pub(super) use super::pipeline::MeshMistPipeline;
    pub(crate) use super::plugin::{MeshMistLabel, MeshMistPlugin};
    pub(crate) use super::prepare::MeshMistTextures;
    pub(super) use super::prepare::MeshMistUniformBuffers;
    pub(super) use super::{MeshMistFragmentBindGroups, SetMeshMistFragmentBindGroup};
}

use bevy::{
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::ROQueryItem,
        resource::Resource,
        system::{SystemParamItem, lifetimeless::SRes},
    },
    platform::collections::HashMap,
    reflect::Reflect,
    render::{
        render_phase::{PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass},
        render_resource::BindGroup,
        sync_world::SyncToRenderWorld,
        view::{ExtractedView, RetainedViewEntity},
    },
};

// TODO: Add correct docs
/// Mesh mist for area mist in a 2D environment.
///
/// This is meant to be added to a [`Mesh2d`](bevy::mesh::Mesh2d) which will determine the mists shape.
///
/// The lit shape is an ellipse in the center that is influenced by the width and height of the [`Mesh2d`](bevy::mesh::Mesh2d). Therefore in most [`Mesh2d`](bevy::mesh::Mesh2d)s, the corners will not be lit.
///
/// ## Formula
///
/// color = src_color * (ambient_color + [`MeshMist::color`] * [`MeshMist::intensity`] * attenuation).
///
/// ## Note
///
/// attenuation decreases smoothly from the center outwards.
#[derive(Component, Reflect, Clone, Copy)]
#[require(SyncToRenderWorld)]
pub struct MeshMist {
    /// The [`Color`] of the mist.
    pub color: Color,
    /// The intensity of the mist.
    pub intensity: f32,
}
impl Default for MeshMist {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.,
        }
    }
}

/// [`BindGroup`]s mapped to [`MeshMist`] [`Entity`]s.
#[derive(Resource, Default)]
pub(super) struct MeshMistFragmentBindGroups(
    pub(super) HashMap<(RetainedViewEntity, Entity), BindGroup>,
);

/// Set [`BindGroup`]s from [`MeshMistFragmentBindGroups`] for [`DrawMeshMist`](crate::mist::prelude::DrawMeshMist).
pub(super) struct SetMeshMistFragmentBindGroup<const I: usize>;
impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetMeshMistFragmentBindGroup<I> {
    type Param = SRes<MeshMistFragmentBindGroups>;
    type ViewQuery = &'static ExtractedView;
    type ItemQuery = ();

    fn render<'w>(
        item: &P,
        view: ROQueryItem<'w, '_, Self::ViewQuery>,
        _entity: Option<()>,
        bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let bind_groups = bind_groups.into_inner();
        let Some(bind_group) = bind_groups
            .0
            .get(&(view.retained_view_entity, item.entity()))
        else {
            return RenderCommandResult::Skip;
        };

        pass.set_bind_group(I, &bind_group, &[]);
        RenderCommandResult::Success
    }
}
