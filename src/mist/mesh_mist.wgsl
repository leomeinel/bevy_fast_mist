#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_fast_mist::mist::types::MeshMist

const INV_TEXTURE_SIZE = 1. / 512.;

const INV_EDGE_BAND = 1. / 0.25;
const EDGE_NOISE_SCALE = 0.5;

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var noise_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var<uniform> mist: MeshMist;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.world_position.xy + mist.offset;
    let noise_uv = fract(pos * INV_TEXTURE_SIZE);
    let noise = textureSample(noise_texture, noise_sampler, noise_uv);

    let mist_alpha = saturate(noise.r + mist.alpha_bias) * mist.max_alpha;
    let edge_dist = min(min(in.uv.x, 1.0 - in.uv.x), min(in.uv.y, 1.0 - in.uv.y)) * INV_EDGE_BAND;
    let edge_alpha = 1. - smoothstep(1., 0., edge_dist - noise.g * EDGE_NOISE_SCALE);
    let alpha = mist_alpha * edge_alpha;

    return vec4<f32>(mist.color.rgb, alpha);
}
