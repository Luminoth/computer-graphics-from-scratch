use crate::{Camera, Canvas, Instance, Plane};

#[derive(Debug, Default)]
pub struct Scene {
    instances: Vec<Instance>,
}

impl Scene {
    #[inline]
    pub fn get_instances(&self) -> &[Instance] {
        &self.instances
    }

    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    pub fn clip(&self, planes: impl AsRef<[Plane]>) -> Self {
        let mut clipped_instances = Vec::with_capacity(self.instances.len());
        for i in &self.instances {
            if let Some(clipped_instance) = i.clip(&planes) {
                clipped_instances.push(clipped_instance);
            }
        }

        Self {
            instances: clipped_instances,
        }
    }

    // TODO: should the camera be a member of the scene?
    pub fn render(&self, canvas: &Canvas, camera: &Camera) -> anyhow::Result<()> {
        let m_camera = camera.get_matrix();

        for instance in self.get_instances() {
            // world space to camera space
            let m = m_camera * instance.get_transform();

            instance.get_model().render(canvas, &m)?;
        }

        Ok(())
    }
}
