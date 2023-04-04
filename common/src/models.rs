use glam::{Mat4, Quat, Vec3, Vec4};
use sdl2::pixels::Color;

use crate::{Canvas, Material, Triangle};

#[derive(Debug, Clone)]
pub struct Transform {
    translation: Vec3,
    rotation: Quat,
    scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::default(),
            rotation: Quat::default(),
            scale: 1.0,
        }
    }
}

impl From<Mat4> for Transform {
    fn from(v: Mat4) -> Self {
        let (scale, rotation, translation) = v.to_scale_rotation_translation();
        //assert!(scale.x. == scale.y && scale.y == scale.z, "{}", scale);
        Self::new(translation, rotation, scale.x)
    }
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: f32) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::default(),
            scale: 1.0,
        }
    }

    #[inline]
    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    #[inline]
    pub fn get_rotation(&self) -> Quat {
        self.rotation
    }

    #[inline]
    pub fn get_translation(&self) -> Vec3 {
        self.translation
    }

    #[inline]
    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.scale, self.scale, self.scale),
            self.rotation,
            self.translation,
        )
    }

    #[inline]
    #[allow(dead_code)]
    fn scale(&self, v: Vec3) -> Vec3 {
        v * self.scale
    }

    #[inline]
    #[allow(dead_code)]
    fn rotate(&self, v: Vec3) -> Vec3 {
        self.rotation * v
    }

    #[inline]
    #[allow(dead_code)]
    fn translate(&self, v: Vec3) -> Vec3 {
        v + self.translation
    }
}

impl std::ops::Mul<Vec4> for Transform {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.get_matrix() * rhs
    }
}

impl std::ops::Mul<Vec4> for &Transform {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.get_matrix() * rhs
    }
}

impl std::ops::Mul<Transform> for Mat4 {
    type Output = Transform;

    #[inline]
    fn mul(self, rhs: Transform) -> Transform {
        (self * rhs.get_matrix()).into()
    }
}

impl std::ops::Mul<&Transform> for Mat4 {
    type Output = Transform;

    #[inline]
    fn mul(self, rhs: &Transform) -> Transform {
        (self * rhs.get_matrix()).into()
    }
}

const CUBE_VERTICES: [Vec3; 8] = [
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(-1.0, 1.0, 1.0),
    Vec3::new(-1.0, -1.0, 1.0),
    Vec3::new(1.0, -1.0, 1.0),
    Vec3::new(1.0, 1.0, -1.0),
    Vec3::new(-1.0, 1.0, -1.0),
    Vec3::new(-1.0, -1.0, -1.0),
    Vec3::new(1.0, -1.0, -1.0),
];

const CUBE_TRIANGLES: [Triangle; 12] = [
    Triangle::new(0, 1, 2, Material::new(Color::RED, None, None)),
    Triangle::new(0, 2, 3, Material::new(Color::RED, None, None)),
    Triangle::new(4, 0, 3, Material::new(Color::GREEN, None, None)),
    Triangle::new(4, 3, 7, Material::new(Color::GREEN, None, None)),
    Triangle::new(5, 4, 7, Material::new(Color::BLUE, None, None)),
    Triangle::new(5, 7, 6, Material::new(Color::BLUE, None, None)),
    Triangle::new(1, 5, 6, Material::new(Color::YELLOW, None, None)),
    Triangle::new(1, 6, 2, Material::new(Color::YELLOW, None, None)),
    Triangle::new(4, 5, 1, Material::new(Color::MAGENTA, None, None)),
    Triangle::new(4, 1, 0, Material::new(Color::MAGENTA, None, None)),
    Triangle::new(2, 6, 7, Material::new(Color::CYAN, None, None)),
    Triangle::new(2, 7, 3, Material::new(Color::CYAN, None, None)),
];

#[derive(Debug, Default, Clone)]
pub struct Cube;

impl Cube {
    #[inline]
    fn get_vertices(&self) -> &[Vec3] {
        &CUBE_VERTICES
    }

    #[inline]
    fn get_triangles(&self) -> &[Triangle] {
        &CUBE_TRIANGLES
    }
}

#[derive(Debug, Clone)]
pub enum Model {
    Cube(Cube),
}

impl Model {
    #[inline]
    pub fn get_vertices(&self) -> &[Vec3] {
        match self {
            Self::Cube(cube) => cube.get_vertices(),
        }
    }

    #[inline]
    pub fn get_triangles(&self) -> &[Triangle] {
        match self {
            Self::Cube(cube) => cube.get_triangles(),
        }
    }

    // transform should be in camera space
    pub fn render(&self, canvas: &Canvas, transform: &Transform) -> anyhow::Result<()> {
        let mut projected = Vec::with_capacity(self.get_vertices().len());
        for v in self.get_vertices() {
            // model space to camera space
            let v = transform * v.extend(1.0);

            // camera space to viewport
            projected.push(canvas.project(&v.truncate()));
        }

        for t in self.get_triangles() {
            t.render(canvas, &projected)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    model: Model,
    transform: Transform,
}

impl Instance {
    pub fn new_cube(transform: Transform) -> Self {
        Self {
            model: Model::Cube(Cube::default()),
            transform,
        }
    }

    #[inline]
    pub fn get_model(&self) -> &Model {
        &self.model
    }

    #[inline]
    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn render(&self, canvas: &Canvas) -> anyhow::Result<()> {
        let model = self.get_model();

        let mut projected = Vec::with_capacity(model.get_vertices().len());
        for v in model.get_vertices() {
            // model space to world space
            let v = self.get_transform() * v.extend(1.0);

            // world space to viewport
            projected.push(canvas.project(&v.truncate()));
        }

        for t in model.get_triangles() {
            t.render(canvas, &projected)?;
        }

        Ok(())
    }
}
