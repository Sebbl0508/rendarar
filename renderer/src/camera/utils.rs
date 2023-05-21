use crate::camera::{Camera3D, CameraUniform, Projection};
use cgmath::{Deg, Point3, Rad};

pub struct CameraOptions3D {
    position: Point3<f32>,
    fovy: Deg<f32>,
    znear: f32,
    zfar: f32,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

pub struct CameraBundle3D {
    pub camera: Camera3D,
    pub projection: Projection,
    uniform: CameraUniform,
}

impl CameraBundle3D {
    pub const fn new(options: CameraOptions3D, screen_size: winit::dpi::PhysicalSize<u32>) -> Self {
        let CameraOptions3D {
            position,
            fovy,
            znear,
            zfar,
            yaw,
            pitch,
        } = options;

        let camera = Camera3D::new(position, yaw, pitch);
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

    pub fn uniform(&self) -> CameraUniform {
        self.uniform
    }
}

impl Default for CameraOptions3D {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            znear: 0.0,
            zfar: 1000.0,
            fovy: Deg(60.0),
            yaw: Rad(0.0),
            pitch: Rad(0.0),
        }
    }
}
