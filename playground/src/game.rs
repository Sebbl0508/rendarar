use crate::camera::FirstPersonController;
use crate::triangle::Triangle;
use renderer::wgpu::WgpuContext;
use std::error::Error;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub struct Game {
    ctx: WgpuContext,
    window: Window,
    event_loop: Option<EventLoop<()>>,

    camera: FirstPersonController,
    triangle: Triangle,
}

impl Game {
    pub fn new(event_loop: EventLoop<()>, window: Window) -> Result<Self, Box<dyn Error>> {
        let ctx = beul::execute(WgpuContext::new(&window))?;
        log::info!("initialized wgpu");

        let camera = FirstPersonController::new(&ctx, ctx.surface_size());
        let triangle = Triangle::new(&ctx);

        Ok(Self {
            ctx,
            window,
            event_loop: Some(event_loop),

            triangle,
            camera,
        })
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.ctx.surface().get_current_texture()?;

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.ctx
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("main command encoder"),
                });

        {
            let mut main_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.807,
                            g: 1.0,
                            b: 0.101,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.ctx.depth_buffer().view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            self.triangle.render(&mut main_pass);
        }

        self.ctx.queue().submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ctx.resize(new_size);
        self.camera.resize(new_size);
    }

    pub fn run(mut self) {
        // This can't crash, since event loop has to be filled
        let event_loop = self.event_loop.take().unwrap_or_else(|| unreachable!());
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::Resized(new_size) => self.resize(new_size),
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size,
                } => self.resize(*new_inner_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(_) => match self.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    // Re-create surface on lost|outdated
                    self.resize(self.ctx.surface_size());
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("out of (gpu?) memory");
                    *control_flow = ControlFlow::Exit;
                }
                Err(wgpu::SurfaceError::Timeout) => log::warn!("surface timout, ignoring..."),
            },
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            _ => {}
        });
    }
}
