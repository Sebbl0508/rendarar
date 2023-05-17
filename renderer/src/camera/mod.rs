use cgmath::{Matrix4, Point3};
use std::f32::consts::FRAC_2_PI;

mod camera3d;
mod projection;
mod uniform;

pub use camera3d::Camera3D;
pub use projection::Projection;

pub trait Camera {
    fn position(&self) -> Point3<f32>;
    fn calc_matrix(&self) -> Matrix4<f32>;
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0
);

pub const SAFE_FRAC_PI_2: f32 = FRAC_2_PI - 0.0001;
