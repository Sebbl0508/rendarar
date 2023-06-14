use crate::wgpu::uniform::global::Globals;
use crate::wgpu::WgpuContext;

mod global;

pub struct Uniforms {
    /// These are used for 'every' shader per frame
    global: Globals,
}

impl Uniforms {
    pub fn new(ctx: &WgpuContext) -> Self {
        Self {
            global: Globals::new(ctx.device(), ctx.surface_size()),
        }
    }

    pub fn update_globals(&self) {
        todo!()
    }
}
