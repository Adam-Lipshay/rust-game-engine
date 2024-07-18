#[derive(Copy, Clone)]
pub struct Vector2f {
    x: f32,
    y: f32,
}

impl Vector2f {
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f {
            x,
            y,
        }
    }

    pub fn to_string(&self) -> String {
        format!("X: {} Y: {}", self.x, self.y)
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }

    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn dot(&self, vector: Vector2f) -> f32 {
        self.x * vector.get_x() + self.y * vector.get_y()
    }

    pub fn normalize(&mut self) -> &mut Vector2f {
        let length = self.length();

        if length != 0.0 {
            self.x /= length;
            self.y /= length;
        }

        self
    }

    pub fn rotate(&self, angle: f32) -> Vector2f {
        let rad = angle as f64 * std::f64::consts::PI / 180 as f64;
        let cos = rad.cos();
        let sin = rad.sin();

        Vector2f::new((self.x as f64 * cos - self.y as f64 * sin) as f32, (self.x as f64 * sin + self.y as f64 * cos) as f32)
    }
}

pub trait Math<T> {
    fn add(&self, value: T) -> Vector2f;
    fn sub(&self, value: T) -> Vector2f;
    fn mul(&self, value: T) -> Vector2f;
    fn div(&self, value: T) -> Vector2f;
}

impl Math<Vector2f> for Vector2f {
    fn add(&self, vector: Vector2f) -> Vector2f {
        Vector2f::new(self.x + vector.get_x(), self.y + vector.get_y())
    }

    fn sub(&self, vector: Vector2f) -> Vector2f {
        Vector2f::new(self.x - vector.get_x(), self.y - vector.get_y())
    }

    fn mul(&self, vector: Vector2f) -> Vector2f {
        Vector2f::new(self.x * vector.get_x(), self.y * vector.get_y())
    }

    fn div(&self, vector: Vector2f) -> Vector2f {
        Vector2f::new(self.x / vector.get_x(), self.y / vector.get_y())
    }
}

impl Math<f32> for Vector2f {
    fn add(&self, r: f32) -> Vector2f {
        Vector2f::new(self.x + r, self.y + r)
    }

    fn sub(&self, r: f32) -> Vector2f {
        Vector2f::new(self.x - r, self.y - r)
    }

    fn mul(&self, r: f32) -> Vector2f {
        Vector2f::new(self.x * r, self.y * r)
    }

    fn div(&self, r: f32) -> Vector2f {
        Vector2f::new(self.x / r, self.y / r)
    }
}
