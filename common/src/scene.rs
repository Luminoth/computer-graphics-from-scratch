use crate::{Camera, Canvas, Instance};

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
