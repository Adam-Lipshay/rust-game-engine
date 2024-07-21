use super::vector3f::Vector3f;

pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }

    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    pub fn normalize(&mut self) -> &mut Quaternion {
        let length = self.length();

        if length != 0.0 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
            self.w /= length;
        }

        self
    }

    pub fn conjucate(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
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

    pub fn get_w(&self) -> f32 {
        self.w
    }

    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_z(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_w(&mut self, value: f32) {
        self.x = value;
    }
}

pub trait Math<T> {
    fn mul(&self, r: T) -> Quaternion;
}

impl Math<Quaternion> for Quaternion {
    fn mul(&self, r: Quaternion) -> Quaternion {
        let w = self.w * r.get_w() - self.x * r.get_x() - self.y * r.get_y() - self.z * r.get_z();
        let x = self.x * r.get_w() + self.w * r.get_x() + self.y * r.get_z() - self.z * r.get_y();
        let y = self.y * r.get_w() + self.w * r.get_y() + self.z * r.get_x() - self.x * r.get_z();
        let z = self.z * r.get_w() + self.w * r.get_z() + self.x * r.get_y() - self.y * r.get_x();

        Quaternion::new(x, y, z, w)
    }
}

impl Math<Vector3f> for Quaternion {
    fn mul(&self, r: Vector3f) -> Quaternion {
        let w = -self.x * r.get_x() - self.y * r.get_y() - self.z * r.get_z();
        let x = self.w * r.get_x() + self.y * r.get_z() - self.z * r.get_y();
        let y = self.w * r.get_y() + self.z * r.get_x() - self.x * r.get_z();
        let z = self.w * r.get_z() + self.x * r.get_y() - self.y * r.get_x();

        Quaternion::new(x, y, z, w)
    }
}
