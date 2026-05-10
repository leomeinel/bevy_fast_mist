//! Different mist types and modules for rendering.
//!
//! This renders mist to the screen texture.
//!
//! This is the first and only render stage of [`FastMistPlugin`](crate::prelude::FastMistPlugin).

mod extract;
mod node;
mod phase;
mod pipeline;
mod plugin;
mod prepare;

pub(super) mod prelude {
    pub(crate) use super::MeshMist;
    pub(super) use super::extract::{ExtractedMeshMist, ExtractedMeshMistFrequency};
    pub(super) use super::node::MeshMistNode;
    pub(super) use super::phase::DrawMeshMist;
    pub(crate) use super::phase::MeshMistPhase;
    pub(super) use super::pipeline::MeshMistPipeline;
    pub(crate) use super::plugin::{MeshMistLabel, MeshMistPlugin};
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
    math::Vec2,
    platform::collections::HashMap,
    reflect::Reflect,
    render::{
        render_phase::{PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass},
        render_resource::BindGroup,
        sync_world::SyncToRenderWorld,
        view::{ExtractedView, RetainedViewEntity},
    },
};

/// Mesh mist for area mist in a 2D environment.
///
/// This is meant to be added to a [`Mesh2d`](bevy::mesh::Mesh2d) which will determine the mists shape.
///
/// ## Formula
///
/// color = vec4<f32>([`color`](Self::color) * [`intensity`](Self::intensity), `mist_alpha` * `edge_alpha`).
///
/// ## Note
///
/// - `mist_alpha` and `edge_alpha` are sampled from a noise texture using [`frequency`](Self::frequency) and [`direction`](Self::direction).
/// - `mist_alpha` is calculated via: `mist_noise` + [`alpha_bias`](Self::alpha_bias) * [`max_alpha`](Self::max_alpha)
#[derive(Component, Reflect, Clone, Copy)]
#[require(SyncToRenderWorld)]
pub struct MeshMist {
    /// The [`Color`] of the mist.
    pub color: Color,
    /// The intensity of the mist.
    pub intensity: f32,
    /// The frequency of the mist.
    ///
    /// This determines the size of the mist-filled regions.
    pub frequency: f32,
    /// The direction of the mist.
    ///
    /// This determines the movement direction of the mist.
    pub direction: Vec2,
    /// The alpha bias of the mist.
    ///
    /// This determines the minimum rendered value of the mist noise by being added directly to it.
    ///
    /// ## Note
    ///
    /// This is meant to be in the range of `0.0..-1.0`.
    pub alpha_bias: f32,
    /// This determines the absolute max alpha of the mist.
    pub max_alpha: f32,
}
impl Default for MeshMist {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.,
            frequency: 3.,
            direction: Vec2::new(4., 1.),
            alpha_bias: -0.3,
            max_alpha: 0.6,
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
