struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,

    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Camera {
    position: vec4<f32>,
    projection: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> cam: Camera;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    //out.clip_pos = vec4<f32>(in.position, 1.0);
    out.clip_pos = cam.projection * vec4<f32>(in.position, 1.0);
    out.uv = in.uv;
    out.color = in.color;

    return out;
}



@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}