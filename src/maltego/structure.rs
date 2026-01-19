use serde::Serialize;

#[derive(Serialize)]
pub struct Entity {
    #[serde(rename = "@Type")]
    pub entity_type: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Weight")]
    pub weight: u64,
}

#[derive(Serialize)]
pub struct Entities {
    #[serde(rename = "Entity")]
    pub entities: Vec<Entity>,
}

#[derive(Serialize)]
pub struct UIMessage {
    #[serde(rename = "@MessageType")]
    pub message_type: String,
    #[serde(rename = "$text")]
    pub content: String,
}

#[derive(Serialize)]
pub struct UIMessages {
    #[serde(rename = "UIMessage")]
    pub messages: Vec<UIMessage>,
}

#[derive(Serialize)]
pub struct MaltegoTransformResponseMessage {
    #[serde(rename = "Entities")]
    pub entities: Entities,
    #[serde(rename = "UIMessages")]
    pub ui_messages: UIMessages,
}

#[derive(Serialize)]
pub struct MaltegoMessage {
    #[serde(rename = "MaltegoTransformResponseMessage")]
    pub response: MaltegoTransformResponseMessage,
}

impl MaltegoMessage {
    pub fn to_xml(&self) -> String {
        quick_xml::se::to_string(self).unwrap()
    }
}
