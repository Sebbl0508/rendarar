use crate::camera::{Camera3D, CameraUniform, Projection};
use cgmath::{Deg, Point3, Rad};

pub struct CameraBundle3D {
    pub camera: Camera3D,
    pub projection: Projection,
    uniform: CameraUniform,
}

impl CameraBundle3D {
    pub const fn new(
        position: Point3<f32>,
        screen_size: winit::dpi::PhysicalSize<u32>,
        fovy: Deg<f32>,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let camera = Camera3D::new(position, Rad(0.0), Rad(0.0));
        let projection = Projection::new(screen_size.width, screen_size.height, fovy, znear, zfar);
        let uniform = CameraUniform::new();

        Self {
            camera,
            projection,
            uniform,
        }
    }

    pub fn update(&mut self) {
        self.uniform
            .update_view_proj(&self.camera, &self.projection);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.projection.resize(new_size.width, new_size.height);
        self.uniform
            .update_view_proj(&self.camera, &self.projection);
    }
}
