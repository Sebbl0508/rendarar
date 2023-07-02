use crate::wgpu::shader::Shader;
use crate::wgpu::{Texture, WgpuContext};

// TODO: Struct for configuring pipeline (with most settings having a default implemented)

pub enum ShaderSource<'a> {
    SourceCode(&'a str),
    Module(wgpu::ShaderModule),
    Struct(Shader),
}

pub struct RenderPipeline {
    raw: wgpu::RenderPipeline,
    layout: wgpu::PipelineLayout,
    shader: Shader,
}

impl RenderPipeline {
    pub fn new<'a>(
        ctx: &WgpuContext,
        shader: ShaderSource,
        buffers: &'a [wgpu::VertexBufferLayout<'a>],
        bindgroup_layouts: &'a [&'a wgpu::BindGroupLayout],
        label: Option<&str>,
    ) -> Self {
        let shader_label = label.map(|lbl| format!("shader for pipeline {lbl}"));
        let shader = match shader {
            ShaderSource::SourceCode(src) => {
                Shader::new(ctx.device(), src, shader_label.as_ref().map(|v| v.as_str()))
            }
            ShaderSource::Module(module) => Shader::from(module),
            ShaderSource::Struct(shader) => shader,
        };

        let layout = ctx
            .device()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: bindgroup_layouts,
                push_constant_ranges: &[],
            });

        let pipeline = ctx
            .device()
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: shader.raw(),
                    entry_point: shader.vertex_entry(),
                    buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader.raw(),
                    entry_point: shader.fragment_entry(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: ctx.surface_config().format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        Self {
            raw: pipeline,
            layout,
            shader,
        }
    }

    pub fn raw(&self) -> &wgpu::RenderPipeline {
        &self.raw
    }

    pub fn layout(&self) -> &wgpu::PipelineLayout {
        &self.layout
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }
}
