//! Mirrors `net.h4bbo.lisbon.game.room.models.RoomModelManager`.
use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::dao::mysql::room_model_dao::RoomModelDao;
use crate::game::room::models::room_model::RoomModel;

pub struct RoomModelManager {
    model_map: HashMap<String, RoomModel>,
}

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<RoomModelManager>>> = RwLock::new(None);
}

impl RoomModelManager {
    /// Get the instance.
    pub fn get_instance() -> Arc<RoomModelManager> {
        if let Some(instance) = INSTANCE.read().as_ref() {
            return Arc::clone(instance);
        }

        let instance = Arc::new(Self {
            model_map: RoomModelDao::get_models(),
        });

        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }

    /// Mirrors `getModel`.
    pub fn get_model(&self, model_id: &str) -> Option<RoomModel> {
        self.model_map.get(model_id).cloned()
    }
}
