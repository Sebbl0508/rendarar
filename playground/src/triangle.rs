use bytemuck::{Pod, Zeroable};
use renderer::wgpu::{Buffer, RenderPipeline, ShaderSource, Vertex, WgpuContext};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TriangleVertex {
    position: [f32; 3],
    uv: [f32; 2],
    color: [f32; 4],
}

pub struct Triangle {
    pipeline: RenderPipeline,
    vtx_buf: Buffer,
}

impl Triangle {
    #[rustfmt::skip]
    const VERTICES: &'static [TriangleVertex] = &[
        TriangleVertex::new([ 0.0,  0.5, 0.0], [0.0; 2], [1.0, 0.0, 0.0, 1.0]),
        TriangleVertex::new([-0.5, -0.5, 0.0], [0.0; 2], [0.0, 1.0, 0.0, 1.0]),
        TriangleVertex::new([ 0.5, -0.5, 0.0], [0.0; 2], [0.0, 0.0, 1.0, 1.0]),
    ];

    pub fn new(ctx: &WgpuContext) -> Self {
        let pipeline = RenderPipeline::new(
            ctx,
            ShaderSource::SourceCode(include_str!("../../resources/shaders/simple_triangle.wgsl")),
            &[TriangleVertex::desc()],
            Some("simple triangle pipeline"),
        );

        let vtx_buf = Buffer::new_init(
            ctx.device(),
            bytemuck::cast_slice(Self::VERTICES),
            wgpu::BufferUsages::VERTEX,
            Some("triangle vertex buffer"),
        );

        Self { pipeline, vtx_buf }
    }

    pub fn render<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_pipeline(self.pipeline.raw());
        rpass.set_vertex_buffer(0, self.vtx_buf.raw().slice(..));

        rpass.draw(0..3, 0..1);
    }
}

impl TriangleVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array! {
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x4,
    };

    pub const fn new(position: [f32; 3], uv: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            position,
            uv,
            color,
        }
    }
}

impl Vertex for TriangleVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TriangleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
