#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_fast_mist::mist::types::MeshMist
#import bevy_fast_mist::simplex_noise_2d::simplex_noise_2d

const EDGE_BAND = 0.25;
const INV_EDGE_BAND = 1. / EDGE_BAND;
const EDGE_FREQ_SCALE = 0.6;
const EDGE_NOISE_SCALE = 0.5;

@group(2) @binding(0)
var<uniform> mist: MeshMist;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let mist_noise = 0.5 + 0.5 * simplex_noise_2d(in.world_position.xy * mist.frequency + mist.offset);
    let mist_alpha = saturate(mist_noise + mist.alpha_bias) * mist.max_alpha;

    let edge_dist = min(min(in.uv.x, 1.0 - in.uv.x), min(in.uv.y, 1.0 - in.uv.y)) * INV_EDGE_BAND;
    let edge_noise = 0.5 + 0.5 * simplex_noise_2d(in.world_position.xy * mist.frequency * EDGE_FREQ_SCALE + mist.offset);
    let edge_alpha = 1. - smoothstep(1., 0., edge_dist - edge_noise * EDGE_NOISE_SCALE);

    return vec4<f32>(mist.color, mist_alpha * edge_alpha);
}
