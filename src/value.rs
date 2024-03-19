use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PortValueType {
    Bool,
    Int,
    Float,
    String,
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "imageType", rename_all = "camelCase")]
pub enum Image {
    Url {
        #[serde(rename = "imageURL")]
        image_url: ImageUrl,
    },
    Base64 {
        image_base64: ImageBase64,
    },
}

impl Image {
    pub fn new_url(url: String, expired: u32) -> Self {
        Image::Url {
            image_url: ImageUrl { url, expired },
        }
    }

    pub fn new_base64(base64: String) -> Self {
        Image::Base64 {
            image_base64: ImageBase64 { base64 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUrl {
    url: String,
    expired: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageBase64 {
    base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "portValueType", rename_all = "camelCase")]
pub enum PortValue {
    Bool { boolean_value: bool },
    Int { int_value: i64 },
    Float { float_value: f32 },
    String { string_value: String },
    Image { image_value: Image },
}

impl PortValue {
    pub fn new_bool(boolean_value: bool) -> Self {
        PortValue::Bool { boolean_value }
    }

    pub fn new_int(int_value: i64) -> Self {
        PortValue::Int { int_value }
    }

    pub fn new_float(float_value: f32) -> Self {
        PortValue::Float { float_value }
    }

    pub fn new_string(string_value: String) -> Self {
        PortValue::String { string_value }
    }

    pub fn new_image(image_value: Image) -> Self {
        PortValue::Image { image_value }
    }

    pub fn port_value_type(&self) -> PortValueType {
        match self {
            PortValue::Bool { .. } => PortValueType::Bool,
            PortValue::Int { .. } => PortValueType::Int,
            PortValue::Float { .. } => PortValueType::Float,
            PortValue::String { .. } => PortValueType::String,
            PortValue::Image { .. } => PortValueType::Image,
        }
    }
}
