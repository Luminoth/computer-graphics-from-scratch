use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Default, Clone)]
pub struct Camera {
    translation: Vec3,
    rotation: Quat,
}

impl Camera {
    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::default(),
        }
    }

    #[inline]
    pub fn get_translation(&self) -> Vec3 {
        self.translation
    }

    #[inline]
    pub fn get_rotation(&self) -> Quat {
        self.rotation
    }

    #[inline]
    pub fn get_matrix(&self) -> Mat4 {
        // TODO: this is not right?
        // should be: var cameraMatrix = MultiplyMM4(Transposed(camera.orientation), MakeTranslationMatrix(Multiply(-1, camera.position)));
        // not sure how I'm supposed to transpose the rotation? or if it's even necessary here?
        Mat4::from_rotation_translation(self.rotation, -1.0 * self.translation)
    }
}
