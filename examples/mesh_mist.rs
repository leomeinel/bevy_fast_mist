//! Scene with a green [`Rectangle`] as background and a light cyan [`MeshMist`].

use bevy::{color::palettes::tailwind, prelude::*};
use bevy_fast_mist::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, FastMistPlugin::default()))
        .add_systems(Startup, setup)
        .run()
}

/// Setup scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ClearColor(tailwind::NEUTRAL_500.into()));
    commands.spawn(Camera2d);

    // Background object
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(600., 600.))),
        MeshMaterial2d(materials.add(Color::from(tailwind::GREEN_500))),
    ));

    commands.spawn((
        MeshMist {
            color: tailwind::CYAN_300.into(),
            intensity: 1.,
            ..default()
        },
        // NOTE: `Mesh2d` is required for the shape of `MeshMist`.
        Mesh2d(meshes.add(Rectangle::new(400., 400.))),
    ));
}
