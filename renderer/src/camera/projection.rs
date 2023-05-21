use crate::camera::OPENGL_TO_WGPU_MATRIX;
use cgmath::{perspective, Matrix4, Rad};

#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self
    where
        F: Into<Rad<f32>>,
    {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn set_fovy<F>(&mut self, fovy: F)
    where
        F: Into<Rad<f32>>,
    {
        self.fovy = fovy.into();
    }

    pub fn set_znear(&mut self, znear: f32) {
        self.znear = znear;
    }

    pub fn set_zfar(&mut self, zfar: f32) {
        self.zfar = zfar;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
