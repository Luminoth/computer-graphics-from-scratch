use glam::Vec3;

use crate::math::*;
use crate::shapes::*;

pub(crate) const SURFACE_EPSILON: f32 = 0.001;

#[derive(Debug, Copy, Clone)]
pub struct AmbientLight {
    intensity: f32,
}

impl AmbientLight {
    pub const fn new(intensity: f32) -> Self {
        Self { intensity }
    }

    #[inline]
    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    #[inline]
    pub fn get_contribution(&self) -> f32 {
        self.get_intensity()
    }
}

fn diffuse_specular(intensity: f32, normal: Vec3, l: Vec3, v: Vec3, shininess: Option<f32>) -> f32 {
    // diffuse
    let d = normal.dot(l);
    let diffuse = if d > 0.0 {
        intensity * (d / (normal.length() * l.length()))
    } else {
        0.0
    };

    // specular
    let specular = if let Some(shininess) = shininess {
        let r = reflect_ray(l, normal);

        let d = r.dot(v);
        if d > 0.0 {
            intensity * (d / (r.length() * v.length())).powf(shininess)
        } else {
            0.0
        }
    } else {
        0.0
    };

    diffuse + specular
}

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    intensity: f32,
    position: Vec3,
}

impl PointLight {
    pub const fn new(intensity: f32, position: Vec3) -> Self {
        Self {
            intensity,
            position,
        }
    }

    #[inline]
    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    #[inline]
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn get_contribution(
        &self,
        point: Vec3,
        normal: Vec3,
        v: Vec3,
        shininess: Option<f32>,
    ) -> f32 {
        let l = self.get_position() - point;
        diffuse_specular(self.get_intensity(), normal, l, v, shininess)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DirectionalLight {
    intensity: f32,
    direction: Vec3,
}

impl DirectionalLight {
    pub const fn new(intensity: f32, direction: Vec3) -> Self {
        Self {
            intensity,
            direction,
        }
    }

    #[inline]
    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    #[inline]
    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    #[inline]
    pub fn get_contribution(&self, normal: Vec3, v: Vec3, shininess: Option<f32>) -> f32 {
        let l = self.get_direction();
        diffuse_specular(self.get_intensity(), normal, l, v, shininess)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    Directional(DirectionalLight),
}

impl Light {
    pub const fn new_ambient(intensity: f32) -> Self {
        Self::Ambient(AmbientLight::new(intensity))
    }

    pub const fn new_point(intensity: f32, position: Vec3) -> Self {
        Self::Point(PointLight::new(intensity, position))
    }

    pub const fn new_directional(intensity: f32, direction: Vec3) -> Self {
        Self::Directional(DirectionalLight::new(intensity, direction))
    }
}

/// Compute the lighting at the given point with the given normal and light direction
pub fn compute_lighting(
    point: Vec3,
    normal: Vec3,
    light_direction: Vec3,
    shininess: Option<f32>,
    lights: impl AsRef<[Light]>,
    shapes: impl AsRef<[Shape]>,
) -> f32 {
    assert!(normal.is_normalized());

    let shapes = shapes.as_ref();

    lights
        .as_ref()
        .iter()
        .map(|light| match light {
            Light::Ambient(light) => light.get_contribution(),
            Light::Point(light) => {
                let l = light.get_position() - point;
                let t_max = 1.0;
                if does_intersect(point, l, SURFACE_EPSILON, t_max, shapes) {
                    0.0
                } else {
                    light.get_contribution(point, normal, light_direction, shininess)
                }
            }
            Light::Directional(light) => {
                let l = light.get_direction();
                let t_max = INFINITY;
                if does_intersect(point, l, SURFACE_EPSILON, t_max, shapes) {
                    0.0
                } else {
                    light.get_contribution(normal, light_direction, shininess)
                }
            }
        })
        .sum()
}
