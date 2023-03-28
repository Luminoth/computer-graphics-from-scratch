use glam::Vec3;
use sdl2::pixels::Color;

use crate::{Material, Triangle};

#[derive(Debug, Clone)]
pub struct Transform {
    scale: f32,
    rotation: Vec3,
    translation: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            scale: 1.0,
            rotation: Vec3::default(),
            translation: Vec3::default(),
        }
    }
}

impl Transform {
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            scale: 1.0,
            rotation: Vec3::default(),
            translation,
        }
    }

    #[inline]
    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    #[inline]
    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    #[inline]
    pub fn get_translation(&self) -> Vec3 {
        self.translation
    }

    fn scale(&self, v: Vec3) -> Vec3 {
        v * self.scale
    }

    fn rotate(&self, v: Vec3) -> Vec3 {
        // TODO:
        v
    }

    fn translate(&self, v: Vec3) -> Vec3 {
        v + self.translation
    }

    pub fn apply(&self, v: Vec3) -> Vec3 {
        let scaled = self.scale(v);
        let rotated = self.rotate(scaled);
        let translated = self.translate(rotated);

        translated
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
}
