use serde::{Deserialize, Serialize};

use crate::schemes::{
    Request, Response,
    session::{config_option::SessionConfigOption, id::SessionId},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConfigOptionReq {
    pub session_id: SessionId,
    pub config_id: String,
    #[serde(flatten)]
    pub payload: ConfigOptionPayload,
}

impl Request for SetConfigOptionReq {
    fn method() -> &'static str {
        "session/set_config_option"
    }
}

#[derive(Serialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum ConfigOptionPayload {
    Boolean(ConfigOptionValueBoolean),
    String { value: String },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOptionValueBoolean {
    #[serde(rename = "type")]
    type_field: String, // Always "boolean"
    value: bool,
}

impl ConfigOptionPayload {
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(ConfigOptionValueBoolean {
            type_field: "boolean".to_string(),
            value,
        })
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String {
            value: value.into(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConfigOptionRes {
    pub config_options: Vec<SessionConfigOption>,
}

impl Response for SetConfigOptionRes {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_set_config_option_serialize() {
        let payload = SetConfigOptionReq {
            session_id: SessionId::new("sess_abc123def456"),
            config_id: "mode".to_string(),
            payload: ConfigOptionPayload::string("code"),
        };

        let result = serde_json::to_value(payload).unwrap();
        let expected = json!({
          "sessionId": "sess_abc123def456",
          "configId": "mode",
          "value": "code"
        });

        assert_eq!(result, expected);

        let payload = SetConfigOptionReq {
            session_id: SessionId::new("sess_abc123def456"),
            config_id: "brave_mode".to_string(),
            payload: ConfigOptionPayload::boolean(true),
        };

        let result = serde_json::to_value(payload).unwrap();
        let expected = json!({
          "sessionId": "sess_abc123def456",
          "configId": "brave_mode",
          "type": "boolean",
          "value": true
        });

        assert_eq!(result, expected);
    }
}
