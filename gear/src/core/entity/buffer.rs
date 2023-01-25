use std::collections::LinkedList;

use super::Entity;

pub struct EntityBuffer {
    pub entities: LinkedList<Entity>,
    last_id: u32,
}

impl EntityBuffer {
    pub fn new() -> Self {
        Self {
            entities: LinkedList::new(),
            last_id: 0,
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push_back(entity);
    }
}
