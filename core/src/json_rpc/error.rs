use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: RpcErrorCode,
    pub message: String,
}

/// Predefined error codes for common JSON-RPC and ACP-specific errors.
///
/// These codes follow the JSON-RPC 2.0 specification for standard errors
/// and use the reserved range (-32000 to -32099) for protocol-specific errors.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[repr(i32)]
pub enum RpcErrorCode {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    Parse,
    /// The JSON sent is not a valid Request object.
    InvalidRequest,
    /// The method does not exist or is not available.
    MethodNotFound,
    /// Invalid method parameter(s).
    InvalidParams,
    /// Internal JSON-RPC error. Reserved for implementation-defined server errors.
    Internal,
    /// Execution of the method was aborted either due to a cancellation request
    /// from the caller or because of resource constraints or shutdown.
    RequestCancelled,
    /// Authentication is required before this operation can be performed.
    AuthenticationRequired,
    /// A given resource, such as a file, was not found.
    ResourceNotFound,
    Other(i32),
}

impl From<i32> for RpcErrorCode {
    fn from(value: i32) -> Self {
        match value {
            -32700 => Self::Parse,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::Internal,
            -32800 => Self::RequestCancelled,
            -32000 => Self::AuthenticationRequired,
            -32002 => Self::ResourceNotFound,
            other => Self::Other(other),
        }
    }
}

impl<'de> Deserialize<'de> for RpcErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RpcErrorCodeVisitor;

        impl<'de> serde::de::Visitor<'de> for RpcErrorCodeVisitor {
            type Value = RpcErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an integer JSON-RPC error code")
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(RpcErrorCode::from(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom("RPC error code is outside i32 range"))?;

                Ok(RpcErrorCode::from(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom("RPC error code is outside i32 range"))?;

                Ok(RpcErrorCode::from(value))
            }
        }

        deserializer.deserialize_i32(RpcErrorCodeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_rpc_error() {
        let error = json!({
            "code": -32800,
            "message": "Request was cancelled"
        });

        let error: RpcError = serde_json::from_value(error).unwrap();

        assert_eq!(error.code, RpcErrorCode::RequestCancelled);
        assert_eq!(error.message, "Request was cancelled");
    }

    #[test]
    fn test_deserialize_rpc_error_code_other() {
        let error = json!({
            "code": 23,
            "message": "other error"
        });

        let error: RpcError = serde_json::from_value(error).unwrap();

        assert_eq!(error.code, RpcErrorCode::Other(23));
        assert_eq!(error.message, "other error");
    }
}
