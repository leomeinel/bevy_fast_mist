//! [`Plugin`] for rendering mist from [`MeshMistMaterial`].

use bevy::{
    app::{App, Plugin, Update},
    asset::embedded_asset,
    ecs::schedule::IntoScheduleConfigs,
    shader::load_shader_library,
    sprite_render::Material2dPlugin,
};

use crate::mist::prelude::*;

/// [`Plugin`] for rendering mist from [`MeshMistMaterial`].
pub(crate) struct MeshMistPlugin;
impl Plugin for MeshMistPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "types.wgsl");
        embedded_asset!(app, "mesh_mist.wgsl");

        app.add_plugins(Material2dPlugin::<MeshMistMaterial>::default());

        app.init_resource::<MistNoiseMap>();

        app.add_systems(
            Update,
            (
                super::noise::update_mist_noise_map,
                super::material::insert_mesh_mist_material,
                super::material::update_mesh_mist_material_offset,
            )
                .chain(),
        );
    }
}
