use bytemuck::{Pod, Zeroable};
use cgmath::Matrix4;
use wgpu::util::DeviceExt;

pub struct Globals {
    buffer: wgpu::Buffer,
    bindgroup: wgpu::BindGroup,
    bindgroup_layout: wgpu::BindGroupLayout,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct GlobalsUniform {
    /// Time since application initialization in seconds
    time: f32,

    /// Time since last frame in seconds
    dt: f32,

    /// Number of frames drawn
    frames: u32,

    // Padding, to make it 64-bit / 128-bit aligned
    __padding: u32,

    /// Matrix for the view (screen w * h)
    view_matrix: [[f32; 4]; 4],
}

impl Globals {
    pub fn new(device: &wgpu::Device, screen_size: winit::dpi::PhysicalSize<u32>) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("global uniform buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[GlobalsUniform {
                time: 0.0,
                dt: 16.6666,
                frames: 0,
                view_matrix: Self::calc_view_matrix(screen_size).into(),
                __padding: 0,
            }])
        });
    }

    pub fn update(&self, screen_size: winit::dpi::PhysicalSize<u32>, time: f32, dt: f32, frames: u32) {
        let uniform = GlobalsUniform {
            time, dt, frames, view_matrix: Self::calc_view_matrix(screen_size).into(), __padding,
        };

        todo!()
    }

    #[rustfmt::skip]
    pub fn calc_view_matrix(screen_size: winit::dpi::PhysicalSize<u32>) -> Matrix4<f32> {
        Matrix4::new(
            1.0 / (screen_size.width as f32 / 2.0), 0.0, 0.0, 0.0,
            0.0, -1.0 / (screen_size.height as f32 / 2.0), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, 0.0, 1.0,
        )
    }
}