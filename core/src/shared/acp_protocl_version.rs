use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u16)]
#[derive(Serialize_repr, Deserialize_repr)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum AcpProtocolVersion {
    V0 = 0,
    V1 = 1,
}

#[cfg(test)]
mod tests {
    use super::AcpProtocolVersion;

    #[test]
    fn test_protocol_version_serialize() {
        assert_eq!(serde_json::to_string(&AcpProtocolVersion::V1).unwrap(), "1");
    }

    #[test]
    fn test_protocol_version_deserialize() {
        assert_eq!(
            serde_json::from_str::<AcpProtocolVersion>("0").unwrap(),
            AcpProtocolVersion::V0
        );
    }
}
