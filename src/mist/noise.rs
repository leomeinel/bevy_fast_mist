//! Noise for mist rendering.

use bevy::{
    asset::{Assets, Handle, RenderAssetUsages},
    ecs::{
        resource::Resource,
        system::{Query, ResMut},
    },
    image::{Image, ImageSampler},
    math::{FloatOrd, Vec2},
    platform::collections::{HashMap, HashSet},
    render::{
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils::default,
};
use noise_functions::{Noise, Simplex};

use crate::mist::prelude::*;

/// Map of [`MeshMist::frequency`] to the relevant noise [`Handle<Image>`].
#[derive(Resource, Clone, Default, ExtractResource)]
pub(crate) struct MistNoiseMap(pub(crate) HashMap<FloatOrd, Handle<Image>>);

// FIXME: I'd prefer being able to only execute this when MeshMist is Added or Changed, but to
//        clear old frequencies, we need to execute this every frame.
/// Update [`MistNoiseMap`].
///
/// This generates [`Image`]s from [`noise_image`] and maps them to the used [`MeshMist::frequency`].
pub(super) fn update_mist_noise_map(
    mist_query: Query<&MeshMist>,
    mut images: ResMut<Assets<Image>>,
    mut noise_map: ResMut<MistNoiseMap>,
) {
    let frequencies: HashSet<FloatOrd> = mist_query.iter().map(|m| FloatOrd(m.frequency)).collect();
    noise_map.0.retain(|f, _| frequencies.contains(&*f));

    for frequency in frequencies {
        noise_map
            .0
            .entry(frequency)
            .or_insert_with(|| images.add(noise_image(frequency.0)));
    }
}

/// Size of the tileable noise [`Image`].
const IMAGE_SIZE: u32 = 512;
/// Frequency scale that is multiplied by [`MeshMist::frequency`] to get the edge frequency.
const EDGE_FREQUENCY_SCALE: f32 = 0.6;

//// Tileable noise [`Image`] using [`Simplex`] noise.
fn noise_image(frequency: f32) -> Image {
    let size = IMAGE_SIZE as f32;
    let mist_noise = Simplex.tileable(frequency, frequency);
    let edge_frequency = frequency * EDGE_FREQUENCY_SCALE;
    let edge_noise = Simplex.tileable(edge_frequency, edge_frequency);

    let mut data = Vec::new();
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let point = Vec2::new(x as f32, y as f32) / size;

            let mist_noise = mist_noise.sample2(point * frequency);
            let mist_value = ((0.5 + 0.5 * mist_noise) * 255.) as u8;
            let edge_noise = edge_noise.sample2(point * edge_frequency);
            let edge_value = ((0.5 + 0.5 * edge_noise) * 255.) as u8;

            data.extend_from_slice(&[mist_value, edge_value, 0, 255]);
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: IMAGE_SIZE,
            height: IMAGE_SIZE,
            ..default()
        },
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::linear();

    image
}
