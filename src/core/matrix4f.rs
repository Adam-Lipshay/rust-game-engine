use zerocopy::AsBytes;

#[derive(Copy, Clone, AsBytes)]
#[repr(C)]
pub struct Matrix4f {
    m: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn init_identity(&mut self) {
        self.m = [[0.0; 4]; 4];
        self.m[0][0] = 1.0;
        self.m[1][1] = 1.0;
        self.m[2][2] = 1.0;
        self.m[3][3] = 1.0;
    }

    pub fn mul(&self, r: Matrix4f) -> Matrix4f {
        let mut res = Matrix4f::new();
        for i in 0..4 {
            for j in 0..4 {
                res.set(i, j, self.m[i as usize][0] * r.get(0, j) +
                              self.m[i as usize][1] * r.get(1, j) +
                              self.m[i as usize][2] * r.get(2, j) +
                              self.m[i as usize][3] * r.get(4, j));
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
}
