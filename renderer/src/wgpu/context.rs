use crate::wgpu::{Texture, WgpuError};
use std::error::Error;
use winit::window::Window;

pub struct WgpuContext {
    device: wgpu::Device,
    adapter: wgpu::Adapter,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_capabilities: wgpu::SurfaceCapabilities,
    surface_config: wgpu::SurfaceConfiguration,
    window_size: winit::dpi::PhysicalSize<u32>,

    // TODO: Move this into camera probably (every camera has optional depth buffer ?)
    depth_buffer: Texture,
}

impl WgpuContext {
    pub async fn new(window: &Window) -> Result<Self, WgpuError> {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });

        let surface = unsafe { instance.create_surface(window) }?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(WgpuError::NoFittingAdapterFound)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("primary render device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let depth_buffer =
            Texture::create_depth_texture(&device, &surface_config, Some("main depth buffer"));

        Ok(Self {
            device,
            adapter,
            queue,
            surface,
            surface_capabilities,
            surface_config,
            window_size,
            depth_buffer,
        })
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_config
    }

    pub fn surface_capabilities(&self) -> &wgpu::SurfaceCapabilities {
        &self.surface_capabilities
    }

    pub fn depth_buffer(&self) -> &Texture {
        &self.depth_buffer
    }

    pub fn surface_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);

            // recreate depth buffer
            self.depth_buffer = Texture::create_depth_texture(
                self.device(),
                self.surface_config(),
                Some("main depth buffer"),
            );

            self.window_size = new_size;
        }
    }
}
