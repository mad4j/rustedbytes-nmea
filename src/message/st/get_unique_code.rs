//! $PSTMGETUCODEOK
//! Message sent in response to command $PSTMGETUCODE
//! Parameter               Format          Description
//! unique_code             Char, 32 bytes  The Unique ID written in the secondary boots
//!
//! $PSTMGETUCODEOK, <unique_code›*<checksum><cr><lf>

use core::str::FromStr;
use heapless::String;

#[derive(Debug, Clone)]
pub struct GetUniqueCode {
    pub unique_code: String<32>,
}

impl GetUniqueCode {
    pub(crate) fn parse(sentence: &crate::message::ParsedSentence) -> Option<Self> {
        let unique_code = sentence.get_field_str(1)?;
        let unique_code = String::from_str(unique_code).ok()?;
        Some(Self { unique_code })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::StMessageData;
    use core::str::FromStr;

    #[test]
    fn test_parse_get_unique_code() {
        let parser = crate::NmeaParser::new();
        let (msg, _i) = parser
            .parse_bytes(b"$PSTMGETUCODEOK,0123456789ABCDEF0123456789ABCDEF*1C\r\n")
            .unwrap();

        let get_unique_code = match msg {
            Some(crate::NmeaMessage::StPropriety(StMessageData::GetUniqueCode(msg))) => {
                msg.unwrap()
            }
            _ => panic!("Unexpected message type"),
        };

        assert_eq!(
            get_unique_code.unique_code.as_str(),
            String::<32>::from_str("0123456789ABCDEF0123456789ABCDEF")
                .unwrap()
                .as_str()
        );
    }

    #[test]
    fn test_parse_get_unique_code_too_long() {
        let parser = crate::NmeaParser::new();
        let (get_unique_code, _i) = parser
            .parse_bytes(b"$PSTMGETUCODEOK,0123456789ABCDEF0123456789ABCDEFF*1D\r\n")
            .unwrap();

        let get_unique_code = match get_unique_code {
            Some(crate::NmeaMessage::StPropriety(StMessageData::GetUniqueCode(msg))) => {
                msg.unwrap()
            }
            _ => panic!("Unexpected message type"),
        };

        assert_eq!(
            get_unique_code.unique_code.as_str(),
            String::<32>::from_str("0123456789ABCDEF0123456789ABCDEF")
                .unwrap()
                .as_str()
        );
    }

    #[test]
    fn test_parse_get_unique_code_empty() {
        let parser = crate::NmeaParser::new();
        let get_unique_code = parser.parse_bytes(b"$PSTMGETUCODEOK,*1D\r\n");
        assert!(get_unique_code.is_err());
    }
}
