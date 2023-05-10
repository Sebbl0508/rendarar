use crate::wgpu::WgpuContext;
use std::error::Error;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub struct Game {
    ctx: WgpuContext,
    window: Window,
    event_loop: Option<EventLoop<()>>,
}

impl Game {
    pub fn new(event_loop: EventLoop<()>, window: Window) -> Result<Self, Box<dyn Error>> {
        let ctx = beul::execute(WgpuContext::new(&window))?;
        log::info!("initialized wgpu");

        Ok(Self {
            ctx,
            window,
            event_loop: Some(event_loop),
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ctx.resize(new_size);
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
            _ => {}
        });
    }
}
