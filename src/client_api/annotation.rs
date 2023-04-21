extern crate serde;

use self::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationPacket {
    pub kid: String,
    pub wrapped_data: Vec<u8>,
    pub iv: Vec<u8>,
    pub algorithm: String,
    pub key_length: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let packet = AnnotationPacket {
            kid: "test".to_string(),
            wrapped_data: vec![0x01, 0x02, 0x03],
            iv: vec![0x04, 0x05, 0x06],
            algorithm: "AES".to_string(),
            key_length: 256,
        };

        let serialized = serde_json::to_string(&packet).unwrap();
        let deserialized: AnnotationPacket = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.kid, packet.kid);
        assert_eq!(deserialized.wrapped_data, packet.wrapped_data);
        assert_eq!(deserialized.iv, packet.iv);
        assert_eq!(deserialized.algorithm, packet.algorithm);
        assert_eq!(deserialized.key_length, packet.key_length);
    }
}
