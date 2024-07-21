use super::{matrix4f::Matrix4f, projection, vector3f::Vector3f};

pub struct Transform<'a> {
    translation: Vector3f,
    rotation: Vector3f,
    scale: Vector3f,
    proj: &'a projection::Projection,
}

impl<'a> Transform<'a> {
    pub fn new(proj: &'a projection::Projection) -> Transform<'a> {
        Transform {
            translation: Vector3f::new(0.0, 0.0, 0.0),
            rotation: Vector3f::new(0.0, 0.0, 0.0),
            scale: Vector3f::new(1.0, 1.0, 1.0),
            proj,
        }
    }

    pub fn get_translation(&self) -> Vector3f {
        self.translation
    }

    pub fn get_rotation(&self) -> Vector3f {
        self.rotation
    }

    pub fn get_scale(&self) -> Vector3f {
        self.scale
    }

    pub fn get_transformation(&self) -> Matrix4f {
        let mut translation_matrix = Matrix4f::new();
        let mut rotation_matrix = Matrix4f::new();
        let mut scale_matrix = Matrix4f::new();
        translation_matrix.init_translation(
            self.translation.get_x(),
            self.translation.get_y(),
            self.translation.get_z(),
        );
        rotation_matrix.init_rotation(
            self.rotation.get_x(),
            self.rotation.get_y(),
            self.rotation.get_z(),
        );
        scale_matrix.init_scale(self.scale.get_x(), self.scale.get_y(), self.scale.get_z());

        translation_matrix.mul(rotation_matrix.mul(scale_matrix))
    }

    pub fn get_projection_transformation(&self) -> Matrix4f {
        let mut projection_matrix = Matrix4f::new();
        projection_matrix.init_projection(
            self.proj.get_fov(),
            self.proj.get_width(),
            self.proj.get_height(),
            self.proj.get_z_near(),
            self.proj.get_z_far(),
        );

        projection_matrix.mul(self.get_transformation())
    }
}

pub trait TransformationSetters<T> {
    fn set_translation(&mut self, translation: T);
    fn set_rotation(&mut self, rotation: T);
    fn set_scale(&mut self, scale: T);
}

impl<'a> TransformationSetters<Vector3f> for Transform<'a> {
    fn set_translation(&mut self, translation: Vector3f) {
        self.translation = translation;
    }

    fn set_rotation(&mut self, rotation: Vector3f) {
        self.rotation = rotation;
    }

    fn set_scale(&mut self, scale: Vector3f) {
        self.scale = scale;
    }
}

impl<'a> TransformationSetters<(f32, f32, f32)> for Transform<'a> {
    fn set_translation(&mut self, translation: (f32, f32, f32)) {
        self.translation = Vector3f::new(translation.0, translation.1, translation.2);
    }

    fn set_rotation(&mut self, rotation: (f32, f32, f32)) {
        self.rotation = Vector3f::new(rotation.0, rotation.1, rotation.2);
    }

    fn set_scale(&mut self, scale: (f32, f32, f32)) {
        self.scale = Vector3f::new(scale.0, scale.1, scale.2);
    }
}
