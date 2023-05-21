use crate::camera::Camera;
use cgmath::num_traits::real::Real;
use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

#[derive(Debug)]
pub struct Camera3D {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}

impl Camera3D {
    pub fn new<V, Y, P>(position: V, yaw: Y, pitch: P) -> Self
    where
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn look_at(&mut self, point: Point3<f32>) {
        let dir = (point - self.position).normalize();

        // TODO: Find out if the resulting values are actually 'Deg'
        self.pitch = Rad(dir.y.asin());
        self.yaw = Rad(dir.x.atan2(dir.z));
    }
}

impl Camera for Camera3D {
    fn position(&self) -> Point3<f32> {
        self.position
    }

    fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}
