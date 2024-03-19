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
    pub fn new_url(url: impl AsRef<str>, expired: u32) -> Self {
        Image::Url {
            image_url: ImageUrl {
                url: url.as_ref().to_owned(),
                expired,
            },
        }
    }

    pub fn new_base64(base64: impl AsRef<str>) -> Self {
        Image::Base64 {
            image_base64: ImageBase64 {
                base64: base64.as_ref().to_owned(),
            },
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
    Bool {
        #[serde(rename = "booleanValue")]
        boolean_value: bool,
    },
    Int {
        #[serde(rename = "integerValue")]
        integer_value: i64,
    },
    Float {
        #[serde(rename = "floatValue")]
        float_value: f32,
    },
    String {
        #[serde(rename = "stringValue")]
        string_value: String,
    },
    Image {
        #[serde(rename = "imageValue")]
        image_value: Image,
    },
}

impl PortValue {
    pub fn new_bool(boolean_value: bool) -> Self {
        PortValue::Bool { boolean_value }
    }

    pub fn new_integer(integer_value: i64) -> Self {
        PortValue::Int { integer_value }
    }

    pub fn new_float(float_value: f32) -> Self {
        PortValue::Float { float_value }
    }

    pub fn new_string(string_value: impl AsRef<str>) -> Self {
        PortValue::String {
            string_value: string_value.as_ref().to_owned(),
        }
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
