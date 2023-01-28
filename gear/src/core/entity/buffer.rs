use super::{Entity, EntityError, Light};

pub struct EntityBuffer {
    pub entities: Vec<Entity>,
    // pub lights: Vec<Light>,
    pub light: Option<Light>,
    last_id: u32,
}

impl EntityBuffer {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            // lights: Vec::new(),
            light: None,
            last_id: 0,
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn add_light(&mut self, light: Light) {
        // self.lights.push(light);
        self.light = Some(light);
    }

    pub fn get_entity(&self, id: u32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    // pub fn get_light(&self, id: u32) -> Option<&Light> {
    //     self.lights.iter().find(|l| l.entity.id == id)
    // }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    // pub fn get_light_mut(&mut self, id: u32) -> Option<&mut Light> {
    //     self.lights.iter_mut().find(|l| l.entity.id == id)
    // }

    pub fn remove_entity(&mut self, id: u32) -> Result<(), EntityError> {
        if let Some(index) = self.entities.iter().position(|e| e.id == id) {
            self.entities.remove(index);
            Ok(())
        } else {
            Err(EntityError::EntityNotFound)
        }
    }

    // pub fn remove_light(&mut self, id: u32) -> Result<(), EntityError> {
    //     if let Some(index) = self.lights.iter().position(|l| l.entity.id == id) {
    //         self.lights.remove(index);
    //         Ok(())
    //     } else {
    //         Err(EntityError::EntityNotFound)
    //     }
    // }
}
