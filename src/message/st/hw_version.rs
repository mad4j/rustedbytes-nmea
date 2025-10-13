//! The HW version has the following syntax:
//! $PSTMVER,STA80XX_<HW_SIGNATURE_STRING>*<checksum><cr><lf>
//!
//! HW_SIGNATURE_STRING         STA8088 HW
//! 0x2229D041                  BB Mask
//! 0x3229D041                  BC Mask
//! HW_SIGNATURE_STRING         STA8089 and STA8090 HW
//! 0x122BC043                  AA Mask
//! 0x222BC043                  AB Mask
//! 0x322BC043                  BA Mask
//! 0x422BC043                  BB Mask
//! 0x522BC043                  BC Mask
//! 0x622BC043                  BD Mask

use core::str::FromStr;

#[derive(Debug, Clone)]
pub enum HarwareType {
    Sta8088,
    Sta8089,
    Sta8090,
}

impl FromStr for HarwareType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STA8088" => Ok(HarwareType::Sta8088),
            "STA8089" => Ok(HarwareType::Sta8089),
            "STA8090" => Ok(HarwareType::Sta8090),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mask {
    Aa,
    Ab,
    Ba,
    Bb,
    Bc,
    Bd,
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = match s {
            "0x2229D041" => Self::Bb,
            "0x3229D041" => Self::Bc,
            "0x122BC043" => Self::Aa,
            "0x222BC043" => Self::Ab,
            "0x322BC043" => Self::Ba,
            "0x422BC043" => Self::Bb,
            "0x522BC043" => Self::Bc,
            "0x622BC043" => Self::Bd,
            _ => return Err(()),
        };
        Ok(v)
    }
}

#[derive(Debug, Clone)]
pub struct HardwareVersion {
    pub hw_type: HarwareType,
    pub mask: Mask,
}

impl HardwareVersion {
    pub(crate) fn parse(sentence: &crate::message::ParsedSentence) -> Option<Self> {
        let val = sentence.get_field_str(1)?;
        let mut s = val.split("_");
        let hw_type = s.next()?.parse().ok()?;
        let mask = s.next()?.parse().ok()?;
        Some(HardwareVersion { hw_type, mask })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::StMessageData;
    use crate::NmeaParser;

    #[test]
    fn test_hw_version() {
        let parser = NmeaParser::new();
        let (msg, _i) = parser
            .parse_bytes(b"$PSTMVER,STA8088_0x2229D041*0F\r\n")
            .unwrap();
        let msg = msg.unwrap();
        match msg {
            crate::NmeaMessage::StPropriety(StMessageData::HardwareVersion(h)) => {
                assert!(matches!(h.hw_type, super::HarwareType::Sta8088));
                assert!(matches!(h.mask, super::Mask::Bb));
            }
            _ => panic!("Unexpected message type"),
        }

        let (msg, _i) = parser
            .parse_bytes(b"$PSTMVER,STA8089_0x122BC043*0F\r\n")
            .unwrap();
        let msg = msg.unwrap();
        match msg {
            crate::NmeaMessage::StPropriety(StMessageData::HardwareVersion(h)) => {
                assert!(matches!(h.hw_type, super::HarwareType::Sta8089));
                assert!(matches!(h.mask, super::Mask::Aa));
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
