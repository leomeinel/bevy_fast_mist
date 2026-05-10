//! [`Material2d`] for mist rendering.

use std::collections::HashMap;

use bevy::{
    asset::{Asset, AssetId, AssetPath, Assets, Handle, embedded_path},
    color::{Alpha as _, LinearRgba},
    ecs::{
        entity::Entity,
        query::{Added, Changed, Or, With},
        system::{Commands, Local, Query, Res, ResMut},
    },
    image::Image,
    math::{FloatOrd, Vec2},
    mesh::Mesh2d,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d},
    time::Time,
    utils::default,
};

use crate::mist::prelude::*;

/// [`Material2d`] for mist rendering.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub(super) struct MeshMistMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub(super) noise_texture: Handle<Image>,
    #[uniform(2)]
    pub(super) color: LinearRgba,
    #[uniform(2)]
    pub(super) offset: Vec2,
    #[uniform(2)]
    pub(super) alpha_bias: f32,
    #[uniform(2)]
    pub(super) max_alpha: f32,
}
impl MeshMistMaterial {
    fn with_noise_texture(self, noise_texture: Handle<Image>) -> Self {
        Self {
            noise_texture,
            ..self
        }
    }
}
impl From<MeshMist> for MeshMistMaterial {
    fn from(mist: MeshMist) -> Self {
        Self {
            color: (mist.color.to_linear() * mist.intensity).with_alpha(1.),
            alpha_bias: mist.alpha_bias,
            max_alpha: mist.max_alpha / (1. + mist.alpha_bias),
            ..default()
        }
    }
}
impl Material2d for MeshMistMaterial {
    fn fragment_shader() -> ShaderRef {
        AssetPath::from_path_buf(embedded_path!("mesh_mist.wgsl"))
            .with_source("embedded")
            .into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Insert [`MeshMistMaterial`] for every relevant [`MeshMist`].
pub(super) fn insert_mesh_mist_material(
    mist_query: Query<
        (Entity, &MeshMist),
        (With<Mesh2d>, Or<(Added<MeshMist>, Changed<MeshMist>)>),
    >,
    mut commands: Commands,
    mut materials: ResMut<Assets<MeshMistMaterial>>,
    mist_noise_map: Res<MistNoiseMap>,
) {
    for (entity, mist) in mist_query {
        let Some(noise_texture) = mist_noise_map.0.get(&FloatOrd(mist.frequency)) else {
            continue;
        };
        let material = MeshMistMaterial::from(*mist).with_noise_texture(noise_texture.clone());
        let material = materials.add(material);
        commands.entity(entity).insert(MeshMaterial2d(material));
    }
}

/// Update [`MeshMistMaterial::offset`].
///
/// ## Formula
///
/// [`offset`](MeshMistMaterial::offset) = [`MeshMist::direction`] * [`Time::elapsed_secs_wrapped()`].
pub(super) fn update_mesh_mist_material_offset(
    mut materials: ResMut<Assets<MeshMistMaterial>>,
    mist_query: Query<(&MeshMist, &mut MeshMaterial2d<MeshMistMaterial>), With<Mesh2d>>,
    time: Res<Time>,
    mut id_map: Local<HashMap<AssetId<MeshMistMaterial>, Vec2>>,
) {
    // NOTE: This prevents duplicate updates since we are modifying the `MeshMaterial2d` by its id.
    id_map.clear();
    for (mist, material) in mist_query {
        id_map.insert(material.0.id(), mist.direction);
    }

    for (id, direction) in id_map.iter() {
        let Some(material) = materials.get_mut(*id) else {
            continue;
        };
        material.offset = direction * time.elapsed_secs_wrapped();
    }
}
