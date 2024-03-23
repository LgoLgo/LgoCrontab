use crate::error::SignatureError;

pub const SIGNATURE: &[u8] = b"FLV";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub has_audio: bool,
    pub has_video: bool,
    pub data_offset: u32,
}

impl TryFrom<&[u8]> for Header {
    type Error = SignatureError;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        match &input[..3] {
            SIGNATURE => {}
            unknown => {
                return Err(unknown.into());
            }
        }
        Ok(Header {
            version: input[3],
            has_audio: (input[4] & 0x04) != 0,
            has_video: (input[4] & 0x01) != 0,
            data_offset: u32::from_be_bytes([input[5], input[6], input[7], input[8]]),
        })
    }
}

mod tests {
    use crate::structure::{Header};

    #[test]
    fn test_header() {
        let input: &[u8] = &[b'F', b'L', b'V', 0x01, 0x05, 0x00, 0x00, 0x00, 0x09];
        assert_eq!(
            Header::try_from(input).unwrap(),
            Header {
                version: 1,
                has_audio: true,
                has_video: true,
                data_offset: 9,
            }
        );
    }

    #[test]
    fn test_signature_err() {
        let input: &[u8] = &[b'W', b'A', b'V', 0x01, 0x05, 0x00, 0x00, 0x00, 0x09];
        assert!(Header::try_from(input).is_err());
    }
}
