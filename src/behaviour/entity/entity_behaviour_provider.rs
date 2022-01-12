use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::generate_noise::GenerateNoise;
use crate::behaviour::entity::generate_noise::GENERATE_NOISE;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct GenerateNoiseStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<GenerateNoise>>>);

#[waiter_di::provides]
fn create_generate_noise_storage() -> GenerateNoiseStorage {
    GenerateNoiseStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ImageProcessingEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_generate_noise(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_generate_noise(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ImageProcessingEntityBehaviourProviderImpl {
    generate_noise: GenerateNoiseStorage,
}

interfaces!(ImageProcessingEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl ImageProcessingEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            generate_noise: create_generate_noise_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ImageProcessingEntityBehaviourProvider for ImageProcessingEntityBehaviourProviderImpl {
    fn create_generate_noise(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let generate_noise = GenerateNoise::new(entity_instance);
        if generate_noise.is_ok() {
            let generate_noise = Arc::new(generate_noise.unwrap());
            self.generate_noise.0.write().unwrap().insert(id, generate_noise);
            debug!("Added behaviour {} to entity instance {}", GENERATE_NOISE, id);
        }
    }

    fn remove_generate_noise(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.generate_noise.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", GENERATE_NOISE, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.generate_noise.0.write().unwrap().contains_key(&id) {
            self.generate_noise.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", GENERATE_NOISE, id);
        }
    }
}

impl EntityBehaviourProvider for ImageProcessingEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            GENERATE_NOISE => self.create_generate_noise(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            GENERATE_NOISE => self.remove_generate_noise(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
