use wgpu::ShaderModule;

/// Wrapper for a wgpu shader module
///
/// > Only supports WGSL for now
pub struct Shader {
    raw: wgpu::ShaderModule,
}

impl Shader {
    pub const VERTEX_ENTRY: &'static str = "vs_main";
    pub const FRAGMENT_ENTRY: &'static str = "fs_main";

    pub fn new(device: &wgpu::Device, source: impl AsRef<str>, label: Option<&str>) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(source.as_ref().into()),
        });

        Self { raw: shader }
    }

    pub fn raw(&self) -> &wgpu::ShaderModule {
        &self.raw
    }

    pub fn vertex_entry(&self) -> &str {
        Self::VERTEX_ENTRY
    }

    pub fn fragment_entry(&self) -> &str {
        Self::FRAGMENT_ENTRY
    }
}

impl From<wgpu::ShaderModule> for Shader {
    fn from(value: ShaderModule) -> Self {
        Self { raw: value }
    }
}
