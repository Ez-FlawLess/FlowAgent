use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum JsonRpcVersion {
    #[serde(rename = "1.0")]
    V1,
    #[serde(rename = "2.0")]
    V2,
}

#[cfg(test)]
mod tests {
    use super::JsonRpcVersion;

    #[test]
    fn test_json_rpc_version_serialize() {
        let v1 = JsonRpcVersion::V1;
        assert_eq!(serde_json::to_string(&v1).unwrap(), r#""1.0""#);

        let v2 = JsonRpcVersion::V2;
        assert_eq!(serde_json::to_string(&v2).unwrap(), r#""2.0""#);
    }

    #[test]
    fn test_json_rpc_version_deserialize() {
        let v1: JsonRpcVersion = serde_json::from_str(r#""1.0""#).unwrap();
        assert_eq!(v1, JsonRpcVersion::V1);

        let v2: JsonRpcVersion = serde_json::from_str(r#""2.0""#).unwrap();
        assert_eq!(v2, JsonRpcVersion::V2);

        assert!(serde_json::from_str::<JsonRpcVersion>(r#""3.0""#).is_err());
    }
}
