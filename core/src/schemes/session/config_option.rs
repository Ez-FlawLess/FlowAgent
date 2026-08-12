use serde::Deserialize;

/// Unique identifier for a session configuration option.
#[derive(Deserialize)]
pub struct SessionConfigId(pub String);

/// A session configuration option selector and its current state.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfigOption {
    /// Unique identifier for the configuration option.
    pub id: SessionConfigId,
    /// Human-readable label for the option.
    pub name: String,
    /// Optional description for the Client to display to the user.
    pub description: Option<String>,
    /// Optional semantic category for this option (UX only).
    pub category: Option<SessionConfigOptionCategory>,
    #[serde(flatten)]
    pub variant: SessionConfigOptionVariant,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionConfigOptionCategory {
    /// Session mode selector.
    Mode,
    /// Model selector.
    Model,
    /// Model-related configuration parameter.
    ModelConfig,
    /// Thought/reasoning level selector.
    ThoughtLevel,
    /// Unknown / uncategorized selector or custom category starting with `_`.
    #[serde(untagged)]
    Other(String),
}

#[derive(Deserialize)]
pub struct SessionConfigValueId(pub String);

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SessionConfigOptionVariant {
    /// A single-value selector (dropdown) session configuration option payload.
    #[serde(rename_all = "camelCase")]
    Select {
        /// The currently selected value.
        current_value: SessionConfigValueId,
        /// The set of selectable options.
        options: Vec<SessionConfigSelectOptions>,
    },
    /// A boolean on/off toggle session configuration option payload.
    #[serde(rename_all = "camelCase")]
    Boolean {
        /// The current value of the boolean option.
        current_value: bool,
    },
}

#[derive(Deserialize)]
pub struct SessionConfigGroupId(pub String);

/// Possible values for a session configuration option.
#[derive(Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum SessionConfigSelectOptions {
    /// A flat list of options with no grouping.
    Ungrouped(SessionConfigSelectOption),
    /// A list of options grouped under headers.
    Grouped(SessionConfigSelectGroup),
}

/// A possible value for a session configuration option.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfigSelectOption {
    /// Unique identifier for this option value.
    pub value: SessionConfigValueId,
    /// Human-readable label for this option value.
    pub name: String,
    /// Optional description for this option value.
    pub description: Option<String>,
}

/// A group of possible values for a session configuration option.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfigSelectGroup {
    /// Unique identifier for this group.
    pub group: SessionConfigGroupId,
    /// Human-readable label for this group.
    pub name: String,
    /// The set of option values in this group.
    pub options: Vec<SessionConfigSelectOption>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_config_options() {
        let value = json!(
            [
                {
                    "id": "mode",
                    "name": "Session Mode",
                    "description": "Controls how the agent requests permission",
                    "category": "mode",
                    "type": "select",
                    "currentValue": "ask",
                    "options": [
                      {
                        "value": "ask",
                        "name": "Ask",
                        "description": "Request permission before making any changes"
                      },
                      {
                        "value": "code",
                        "name": "Code",
                        "description": "Write and modify code with full tool access"
                      }
                    ]
                },
                {
                    "id": "model",
                    "name": "Model",
                    "category": "model",
                    "type": "select",
                    "currentValue": "model-1",
                    "options": [
                      {
                        "value": "model-1",
                        "name": "Model 1",
                        "description": "The fastest model"
                      },
                      {
                        "value": "model-2",
                        "name": "Model 2",
                        "description": "The most powerful model"
                      }
                    ]
                }
            ]
        );

        let config_options: Vec<SessionConfigOption> = serde_json::from_value(value).unwrap();
    }
}
