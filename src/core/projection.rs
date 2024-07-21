use super::matrix4f::Matrix4f;

pub struct Projection {
    fov: f32,
    width: f32,
    height: f32,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    pub fn new(fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) -> Projection {
        Projection {
            fov,
            width,
            height,
            z_near,
            z_far
        }
    }

    pub fn get_projection(&self) -> Matrix4f {
        let mut projection_matrix = Matrix4f::new();
        projection_matrix.init_projection(self.fov, self.width, self.height, self.z_near, self.z_far);

        projection_matrix
    }

    pub fn set_projection(&mut self, fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) {
        self.fov = fov;
        self.width = width;
        self.height = height;
        self.z_near = z_near;
        self.z_far = z_far;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_z_near(&self) -> f32 {
        self.z_near
    }

    pub fn get_z_far(&self) -> f32 {
        self.z_far
    }
}
