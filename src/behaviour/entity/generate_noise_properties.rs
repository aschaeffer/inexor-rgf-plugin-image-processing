use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum GenerateNoiseProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "data_url")]
    DATA_URL,
}

impl GenerateNoiseProperties {
    pub fn default_value(&self) -> Value {
        match self {
            GenerateNoiseProperties::TRIGGER => json!(false),
            GenerateNoiseProperties::DATA_URL => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(GenerateNoiseProperties::TRIGGER),
            NamedProperty::from(GenerateNoiseProperties::DATA_URL),
        ]
    }
}

impl From<GenerateNoiseProperties> for NamedProperty {
    fn from(p: GenerateNoiseProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<GenerateNoiseProperties> for String {
    fn from(p: GenerateNoiseProperties) -> Self {
        p.to_string()
    }
}
