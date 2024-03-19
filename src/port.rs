use crate::value::{PortValue, PortValueType};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    id: String,
    pub(crate) name: String,
    port_value_type: PortValueType,
    default_value: Option<PortValue>,
}

impl Port {
    pub fn new(name: String, value_type: PortValueType) -> Self {
        Port {
            // determined by frontend
            id: String::new(),
            name,
            port_value_type: value_type,
            default_value: None,
        }
    }

    pub fn with_default_value(self, default_value: PortValue) -> Self {
        Self {
            default_value: Some(default_value),
            ..self
        }
    }
}

impl std::cmp::PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl std::cmp::Eq for Port {}

impl std::cmp::PartialOrd for Port {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl std::cmp::Ord for Port {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
