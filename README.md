# Bevy Fast Mist

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/leomeinel/bevy_fast_mist)
[![Crates.io](https://img.shields.io/crates/v/bevy_fast_mist.svg)](https://crates.io/crates/bevy_fast_mist)
[![Downloads](https://img.shields.io/crates/d/bevy_fast_mist.svg)](https://crates.io/crates/bevy_fast_mist)
[![Docs](https://docs.rs/bevy_fast_mist/badge.svg)](https://docs.rs/bevy_fast_mist/latest/bevy_fast_mist/)

Simple moving 2D mist for Bevy focused on performance over features.

:warning: | This is still in development and not at all feature complete.

## Features

### Mist

| Purpose   | Component  | Config                                                                       |
| --------- | ---------- | ---------------------------------------------------------------------------- |
| Mesh mist | `MeshMist` | `color`, `intensity`, `frequency`, `direction`, `alpha_bias` and `max_alpha` |

### Mesh2d

The following `Components` allow the customization of their shape via attaching a `Mesh2d`:

- `MeshMist`

The `Mesh2d` is required for the `Components` to render.

## Usage

Take a look at [`/examples`](https://github.com/leomeinel/bevy_fast_mist/tree/main/examples) to find out how to use this crate.

### Showcase

I am using most features in my learning project [Slimy Mist](https://github.com/leomeinel/slimy_mist).

<img src="https://github.com/leomeinel/bevy_fast_mist/blob/main/static/slimy_mist.webp?raw=true" width="400" alt="slimy mist example">

### Examples

#### `mesh_mist.rs`

Scene with a green `Rectangle` as background and a light cyan `MeshMist`.

<img src="https://github.com/leomeinel/bevy_fast_mist/blob/main/static/mesh_mist.webp?raw=true" width="400" alt="mesh mist example">

## Resources

### Code

- [Bevy Example - Custom Render Phase](https://bevy.org/examples/shaders/custom-render-phase/)
- [Noise Functions](https://crates.io/crates/noise-functions)

### Articles

- [hackmd.io - Bevy's Rendering Crates](https://hackmd.io/@bevy/rendering_summary)
- [hackmd.io - The Abyss](https://hackmd.io/@bevy/the_abyss)
- [NoisePosti.ng - The Perlin Problem: Moving Past Square Noise](https://noiseposti.ng/posts/2022-01-16-The-Perlin-Problem-Moving-Past-Square-Noise.html)
- [WebGPU Shading Language](https://www.w3.org/TR/WGSL/)
