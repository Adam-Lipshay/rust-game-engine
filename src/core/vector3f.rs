use zerocopy::AsBytes;

use super::quaternion::{Math as QuaternionMath, Quaternion};

#[derive(Copy, Clone, AsBytes, Debug)]
#[repr(C)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f { x, y, z }
    }

    pub fn rotate(&mut self, angle: f32, axis: Vector3f) {
        let half_angle_rads = (angle / 2.0).to_radians();

        let sin_half_angle = half_angle_rads.sin();
        let cos_half_angle = half_angle_rads.cos();

        let rx = axis.get_x() * sin_half_angle;
        let ry = axis.get_y() * sin_half_angle;
        let rz = axis.get_z() * sin_half_angle;
        let rw = cos_half_angle;

        let rotation = Quaternion::new(rx, ry, rz, rw);
        let conjucate = rotation.conjucate();

        let w = rotation.mul(*self).mul(conjucate);

        self.x = w.get_x();
        self.y = w.get_y();
        self.z = w.get_z();
    }

    pub fn to_string(&self) -> String {
        format!("X: {} Y: {} Z: {}", self.x, self.y, self.z)
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }

    pub fn set_z(&mut self, value: f32) {
        self.z = value;
    }

    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn dot(&self, vector: Vector3f) -> f32 {
        self.x * vector.get_x() + self.y * vector.get_y() + self.z * vector.get_z()
    }

    pub fn normalize(&mut self) -> &mut Vector3f {
        let length = self.length();

        if length != 0.0 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }

        self
    }

    // pub fn rotate(&self, angle: f32) -> Vector3f {
    //     let rad = angle as f64 * std::f64::consts::PI / 180 as f64;
    //     let cos = rad.cos();
    //     let sin = rad.sin();

    //     Vector3f::new((self.x as f64 * cos - self.y as f64 * sin) as f32, (self.x as f64 * sin + self.y as f64 * cos) as f32)
    // }

    pub fn cross(&self, vector: Vector3f) -> Vector3f {
        let x = self.y * vector.get_z() - self.z * vector.get_y();
        let y = self.z * vector.get_x() - self.x * vector.get_z();
        let z = self.x * vector.get_y() - self.y * vector.get_x();

        Vector3f::new(x, y, z)
    }

    pub fn serialize(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

pub trait Math<T> {
    fn add(&self, value: T) -> Vector3f;
    fn sub(&self, value: T) -> Vector3f;
    fn mul(&self, value: T) -> Vector3f;
    fn div(&self, value: T) -> Vector3f;
}

impl Math<Vector3f> for Vector3f {
    fn add(&self, vector: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x + vector.get_x(),
            self.y + vector.get_y(),
            self.z + vector.get_z(),
        )
    }

    fn sub(&self, vector: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x - vector.get_x(),
            self.y - vector.get_y(),
            self.z - vector.get_z(),
        )
    }

    fn mul(&self, vector: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x * vector.get_x(),
            self.y * vector.get_y(),
            self.z * vector.get_z(),
        )
    }

    fn div(&self, vector: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x / vector.get_x(),
            self.y / vector.get_y(),
            self.z / vector.get_z(),
        )
    }
}

impl Math<f32> for Vector3f {
    fn add(&self, r: f32) -> Vector3f {
        Vector3f::new(self.x + r, self.y + r, self.z + r)
    }

    fn sub(&self, r: f32) -> Vector3f {
        Vector3f::new(self.x - r, self.y - r, self.z - r)
    }

    fn mul(&self, r: f32) -> Vector3f {
        Vector3f::new(self.x * r, self.y * r, self.z * r)
    }

    fn div(&self, r: f32) -> Vector3f {
        Vector3f::new(self.x / r, self.y / r, self.z / r)
    }
}
