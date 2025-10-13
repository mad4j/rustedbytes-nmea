//! $PSTMSTANDBYENABLE
//! Message sent in response to command $PSTMSTANDBYENABLE without parameters.
//!
//! Parameter                   Format              Description
//! status                      Decimal, 1 digits   Set the standby enable status
//!                                                     0: Active Periodic mode
//!                                                     1: Periodic mode, standby allowed
//!
//! $PSTMSTANDBYENABLE,‹status>*<checksum>‹cr><lf>

#[derive(Debug, Clone)]
pub enum PeriodicStandbyMode {
    Active,
    Periodic,
}

impl TryFrom<u8> for PeriodicStandbyMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PeriodicStandbyMode::Active),
            1 => Ok(PeriodicStandbyMode::Periodic),
            _ => Err("Invalid PeriodicStandbyMode value"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StandbyEnableStatus {
    pub status: PeriodicStandbyMode,
}

impl StandbyEnableStatus {
    pub(crate) fn parse(sentence: &crate::message::ParsedSentence) -> Option<Self> {
        let status = sentence.parse_field::<u8>(1)?;
        let status = PeriodicStandbyMode::try_from(status).ok()?;
        Some(Self {
            status,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::message::StMessageData;

    #[test]
    fn periodic_standby_mode_from_u8() {
        assert!(matches!(
            PeriodicStandbyMode::try_from(0).unwrap(),
            PeriodicStandbyMode::Active
        ));
        assert!(matches!(
            PeriodicStandbyMode::try_from(1).unwrap(),
            PeriodicStandbyMode::Periodic
        ));
        assert!(PeriodicStandbyMode::try_from(2).is_err());
    }

    #[test]
    fn test_standby_enable_status_parse() {
        let parser = crate::NmeaParser::new();
        let (msg, _i) = parser.parse_bytes(b"$PSTMSTANDBYENABLE,0*0E\r\n").unwrap();

        let standby_enable_status = match msg {
            Some(crate::NmeaMessage::StPropriety(StMessageData::StandbyEnableStatus(msg))) => msg,
            _ => panic!("Unexpected message type"),
        };
        assert!(matches!(
            standby_enable_status.status,
            PeriodicStandbyMode::Active
        ));

        let (msg, _i) = parser.parse_bytes(b"$PSTMSTANDBYENABLE,1*0F\r\n").unwrap();

        let standby_enable_status = match msg {
            Some(crate::NmeaMessage::StPropriety(StMessageData::StandbyEnableStatus(msg))) => msg,
            _ => panic!("Unexpected message type"),
        };
        assert!(matches!(
            standby_enable_status.status,
            PeriodicStandbyMode::Periodic
        ));
    }
}
