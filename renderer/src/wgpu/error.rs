#[derive(Debug, thiserror::Error)]
pub enum WgpuError {
    #[error("no suitable wgpu adapter found")]
    NoFittingAdapterFound,

    #[error(transparent)]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),

    #[error(transparent)]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
}
