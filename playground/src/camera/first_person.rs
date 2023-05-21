use cgmath::Point3;
use renderer::camera::{CameraBundle3D, CameraOptions3D, CameraUniform};
use renderer::wgpu::WgpuContext;
use wgpu::util::DeviceExt;

pub struct FirstPersonController {
    bundle: CameraBundle3D,

    buffer: wgpu::Buffer,
    bindgroup: wgpu::BindGroup,
    bindgroup_layout: wgpu::BindGroupLayout,
}

impl FirstPersonController {
    pub fn new(ctx: &WgpuContext, screen_size: winit::dpi::PhysicalSize<u32>) -> Self {
        let mut bundle = CameraBundle3D::new(CameraOptions3D::default(), screen_size);
        bundle.camera.position = Point3::new(0.0, 0.0, 1.0);
        bundle.update();

        let buffer = ctx
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("main camera buffer"),
                contents: bytemuck::cast_slice(&[bundle.uniform()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let (bindgroup, bindgroup_layout) = Self::create_bindgroup_and_layout(ctx, &buffer);

        Self {
            bundle,
            buffer,
            bindgroup,
            bindgroup_layout,
        }
    }

    pub fn create_bindgroup_and_layout(
        ctx: &WgpuContext,
        buffer: &wgpu::Buffer,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
        let layout = ctx
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera bindgroup layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let bindgroup = ctx.device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera bindgroup"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        (bindgroup, layout)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.bundle.resize(new_size);
    }

    pub fn update(&mut self, ctx: &WgpuContext) {
        self.bundle.update();
        ctx.queue().write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.bundle.uniform()]),
        );
    }

    pub fn uniform(&self) -> CameraUniform {
        self.bundle.uniform()
    }
}
