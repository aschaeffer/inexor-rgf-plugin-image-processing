use std::sync::Arc;

use crate::reactive::BehaviourCreationError;
use log::trace;
use serde_json::{json, Value};

use crate::behaviour::entity::GenerateNoiseProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const GENERATE_NOISE: &'static str = "generate_noise";

pub struct GenerateNoise {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl GenerateNoise {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<GenerateNoise, BehaviourCreationError> {
        let handle_id = e.properties.get(GenerateNoiseProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        let entity = e.clone();
        e.properties
            .get(GenerateNoiseProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v: &Value| {
                    if !v.is_boolean() || !v.as_bool().unwrap() {
                        return;
                    }

                    let image = image_noise::noise_image(1000, 10);
                    let mut buffer: Vec<u8> = Vec::new();
                    let result = image.write_to(&mut buffer, image::ImageOutputFormat::Png);
                    if !result.is_ok() {
                        return;
                    }
                    let data_as_base64 = base64::encode(&buffer);
                    let data_url = json!(format!("data:image/png;base64,{}", data_as_base64));
                    entity.set(GenerateNoiseProperties::DATA_URL.as_ref(), data_url);
                },
                handle_id,
            );
        Ok(GenerateNoise { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for GenerateNoise {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", GENERATE_NOISE, self.entity.id);
        let property = self.entity.properties.get(GenerateNoiseProperties::TRIGGER.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for GenerateNoise {
    fn drop(&mut self) {
        self.disconnect();
    }
}
