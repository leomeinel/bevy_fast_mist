#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_fast_mist::mist::types::MeshMist
#import bevy_fast_mist::simplex_noise_2d::simplex_noise_2d

const SPEED = vec2<f32>(0.16, 0.04);

const MIST_FREQ = 0.01;
// NOTE: This is meant to be subtracted from `mist_noise` to shift it to the negatives and should always be negative.
const MIST_ALPHA_BIAS = -0.2;
// NOTE: `(1.0 + MIST_ALPHA_BIAS)` can be disregarded. It is only meant for scaling purposes.
const MIST_MAX_ALPHA = 0.6 / (1.0 + MIST_ALPHA_BIAS);

const EDGE_FREQ = 0.006;
const EDGE_BAND = 0.25;
const INV_EDGE_BAND = 1. / EDGE_BAND;

@group(2) @binding(0)
var<uniform> mesh_mist: MeshMist;

// TODO: Sample configurable noise from cpu-generated texture instead.

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let flow = mesh_mist.elapsed_secs * SPEED;

    let mist_noise = 0.5 + 0.5 * simplex_noise_2d(in.world_position.xy * MIST_FREQ + flow);
    let mist_alpha = saturate(mist_noise + MIST_ALPHA_BIAS) * MIST_MAX_ALPHA;

    let edge_dist = min(min(in.uv.x, 1.0 - in.uv.x), min(in.uv.y, 1.0 - in.uv.y)) * INV_EDGE_BAND;
    let edge_noise = 0.5 + 0.5 * simplex_noise_2d(in.world_position.xy * EDGE_FREQ + flow);
    let edge_alpha = 1. - smoothstep(1., 0., edge_dist - edge_noise * 0.5);

    return vec4<f32>(mesh_mist.color, mist_alpha * edge_alpha);
}
