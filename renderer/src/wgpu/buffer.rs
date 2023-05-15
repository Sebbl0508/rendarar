use wgpu::util::DeviceExt;

pub struct Buffer {
    raw: wgpu::Buffer,
    size: wgpu::BufferAddress,
    usages: wgpu::BufferUsages,
}

impl Buffer {
    pub fn new(
        device: &wgpu::Device,
        size: wgpu::BufferAddress,
        usages: wgpu::BufferUsages,
        mapped: bool,
        label: Option<&str>,
    ) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage: usages,
            mapped_at_creation: mapped,
        });
        Self {
            raw: buffer,
            size,
            usages,
        }
    }

    pub fn new_init<'a>(
        device: &wgpu::Device,
        contents: &'a [u8],
        usages: wgpu::BufferUsages,
        label: Option<&str>,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents,
            usage: usages,
        });
        let size = buffer.size();
        Self {
            raw: buffer,
            size,
            usages,
        }
    }

    pub fn size(&self) -> wgpu::BufferAddress {
        self.size
    }

    pub fn usages(&self) -> wgpu::BufferUsages {
        self.usages
    }
}
