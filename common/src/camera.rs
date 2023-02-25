use glam::{EulerRot, Quat, Vec3};

#[derive(Debug, Default, Clone)]
pub struct Camera {
    position: Vec3,
    rotation: Quat,
}

impl Camera {
    // rotation here is in degrees
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::from_euler(
                EulerRot::YXZ,
                rotation.x.to_radians(),
                rotation.y.to_radians(),
                rotation.z.to_radians(),
            ),
        }
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::default(),
        }
    }

    #[inline]
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn get_rotation(&self) -> Quat {
        self.rotation
    }
}
