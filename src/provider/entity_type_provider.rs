use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;
use waiter_di::*;

use crate::model::entity_type::EntityType;
use crate::plugins::EntityTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/entities"]
struct ImageProcessingEntityTypeAsset;

#[async_trait]
pub trait ImageProcessingEntityTypeProvider: EntityTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct ImageProcessingEntityTypeProviderImpl {}

interfaces!(ImageProcessingEntityTypeProviderImpl: dyn EntityTypeProvider);

#[component]
impl ImageProcessingEntityTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ImageProcessingEntityTypeProvider for ImageProcessingEntityTypeProviderImpl {}

impl EntityTypeProvider for ImageProcessingEntityTypeProviderImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        let mut entity_types = Vec::new();
        for file in ImageProcessingEntityTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading entity_type from resource {}", filename);
            let asset = ImageProcessingEntityTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let entity_type: EntityType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(entity_type) => entity_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            entity_types.push(entity_type);
        }
        entity_types
    }
}
