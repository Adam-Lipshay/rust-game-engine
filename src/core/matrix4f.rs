use zerocopy::AsBytes;

#[derive(Copy, Clone, AsBytes)]
#[repr(C)]
pub struct Matrix4f {
    m: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f { m: [[0.0; 4]; 4] }
    }

    pub fn init_identity(&mut self) -> &mut Matrix4f {
        self.m = [[0.0; 4]; 4];
        self.m[0][0] = 1.0;
        self.m[1][1] = 1.0;
        self.m[2][2] = 1.0;
        self.m[3][3] = 1.0;

        self
    }

    pub fn init_translation(&mut self, x: f32, y: f32, z: f32) -> &mut Matrix4f {
        self.init_identity();
        self.m[0][3] = x;
        self.m[1][3] = y;
        self.m[2][3] = z;

        self
    }

    pub fn init_rotation(&mut self, x: f32, y: f32, z: f32) -> &mut Matrix4f {
        let x = x.to_radians();
        let y = y.to_radians();
        let z = z.to_radians();

        let mut rx = Matrix4f::new();
        let mut ry = Matrix4f::new();
        let mut rz = Matrix4f::new();

        rx.m[0][0] = 1.0;
        rx.m[3][3] = 1.0;
        rx.m[1][1] = x.cos();
        rx.m[2][1] = x.sin();
        rx.m[1][2] = -x.sin();
        rx.m[2][2] = x.cos();

        ry.m[1][1] = 1.0;
        ry.m[3][3] = 1.0;
        ry.m[0][0] = y.cos();
        ry.m[2][0] = y.sin();
        ry.m[0][2] = -y.sin();
        ry.m[2][2] = y.cos();

        rz.m[2][2] = 1.0;
        rz.m[3][3] = 1.0;
        rz.m[0][0] = z.cos();
        rz.m[1][0] = z.sin();
        rz.m[0][1] = -z.sin();
        rz.m[1][1] = z.cos();

        self.m = rz.mul(ry.mul(rx)).get_m();

        self
    }

    pub fn init_scale(&mut self, x: f32, y: f32, z: f32) -> &mut Matrix4f {
        self.m = [[0.0; 4]; 4];
        self.m[0][0] = x;
        self.m[1][1] = y;
        self.m[2][2] = z;
        self.m[3][3] = 1.0;

        self
    }

    pub fn init_projection(&mut self, fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) -> &mut Matrix4f {
        self.m = [[0.0; 4]; 4];

        let aspect_ratio = width / height;
        let tan_half_fov = (fov / 2.0).to_radians().tan();
        let z_range = z_near - z_far;

        self.m[0][0] = 1.0 / (tan_half_fov * aspect_ratio);
        self.m[1][1] = 1.0 / tan_half_fov;
        self.m[2][2] = (-z_near - z_far) / z_range;
        self.m[2][3] = 2.0 * z_near * z_far / z_range;
        self.m[3][2] = 1.0;

        self
    }

    pub fn mul(&self, r: Matrix4f) -> Matrix4f {
        let mut res = Matrix4f::new();
        for i in 0..4 {
            for j in 0..4 {
                res.set(
                    i,
                    j,
                    self.m[i as usize][0] * r.get(0, j)
                        + self.m[i as usize][1] * r.get(1, j)
                        + self.m[i as usize][2] * r.get(2, j)
                        + self.m[i as usize][3] * r.get(3, j),
                );
            }
        }

        res
    }

    pub fn get_m(&self) -> [[f32; 4]; 4] {
        self.m
    }

    pub fn set_m(&mut self, m: [[f32; 4]; 4]) {
        self.m = m;
    }

    pub fn get(&self, x: i32, y: i32) -> f32 {
        self.m[x as usize][y as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, value: f32) {
        self.m[x as usize][y as usize] = value;
    }

    pub fn serialize(&self) -> [f32; 24] {
        let mut res = [0.0; 24];
        let mut counter = 0;
        for row in &self.m {
            res[counter..counter + 4].copy_from_slice(row);
            counter += 4;
        }
        res
    }
}
