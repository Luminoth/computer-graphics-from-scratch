use crate::models::*;

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
}
