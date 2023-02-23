use glam::Vec3;

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
    let d = normal.dot(l);

    // diffuse
    let diffuse = if d > 0.0 {
        intensity * (d / (normal.length() * l.length()))
    } else {
        0.0
    };

    // specular
    let specular = if let Some(shininess) = shininess {
        let r = 2.0 * normal * d - l;

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

    pub fn get_contribution(&self, normal: Vec3, v: Vec3, shininess: Option<f32>) -> f32 {
        let l = self.get_direction();
        diffuse_specular(self.get_intensity(), normal, l, v, shininess)
    }
}

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

    #[inline]
    pub fn get_contribution(
        &self,
        point: Vec3,
        normal: Vec3,
        v: Vec3,
        shininess: Option<f32>,
    ) -> f32 {
        match self {
            Light::Ambient(light) => light.get_contribution(),
            Light::Point(light) => light.get_contribution(point, normal, v, shininess),
            Light::Directional(light) => light.get_contribution(normal, v, shininess),
        }
    }
}