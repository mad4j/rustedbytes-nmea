use crate::message::st::diff::DifferentialCorrectionData;
use crate::message::st::get_unique_code::GetUniqueCode;
use crate::message::st::hw_version::HardwareVersion;
use crate::message::st::low_power_on_off::LowPowerOnOff;
use crate::message::st::standby_enable::StandbyEnableStatus;
use crate::message::st::sw_version::SoftwareVersion;
use crate::message::st::tg::TimeAndSatelliteInformation;
use crate::message::ParsedSentence;
use crate::MessageType;

mod diff;
mod get_unique_code;
mod hw_version;
mod low_power_on_off;
mod standby_enable;
mod sw_version;
mod tg;

#[cfg(feature = "st-teseo-liv3")]
#[derive(Debug, Clone)]
pub enum StMessageData {
    DifferentialCorrectionData(DifferentialCorrectionData),
    TimeAndSatelliteInformation(TimeAndSatelliteInformation),
    ConfigAntiJamResult(Result<(), ()>),
    ConfigGeofenceEnableResult(Result<(), ()>),
    ConfigGeofenceCircleConfigureResult(Result<(), ()>),
    ConfigLowPowerOnOffResult(Result<LowPowerOnOff, ()>),
    ConfigLpaResult(Result<(), ()>),
    ConfigStandbyEnableResult(Result<(), ()>),
    ConfigStandbyForceResult(Result<(), ()>),
    GetUniqueCode(Result<GetUniqueCode, ()>),
    SoftwareVersion(SoftwareVersion),
    HardwareVersion(HardwareVersion),
    StandbyEnableStatus(StandbyEnableStatus),
}

impl ParsedSentence {
    pub fn as_st(&self, buffer: &[u8]) -> Option<StMessageData> {
        if self.message_type != MessageType::PSTM {
            return None;
        }

        match buffer {
            x if x.starts_with(b"PSTMDIFF") => DifferentialCorrectionData::parse(self)
                .map(StMessageData::DifferentialCorrectionData),
            x if x.starts_with(b"PSTMTG") => TimeAndSatelliteInformation::parse(self)
                .map(StMessageData::TimeAndSatelliteInformation),
            x if x.starts_with(b"PSTMCFGAJMOK*") => {
                Some(StMessageData::ConfigAntiJamResult(Ok(())))
            }
            x if x.starts_with(b"PSTMCFGAJMERROR*") => {
                Some(StMessageData::ConfigAntiJamResult(Err(())))
            }
            x if x.starts_with(b"PSTMCFGGEOFENCEOK*") => {
                Some(StMessageData::ConfigGeofenceEnableResult(Ok(())))
            }
            x if x.starts_with(b"PSTMCFGGEOFENCEERROR*") => {
                Some(StMessageData::ConfigGeofenceEnableResult(Err(())))
            }
            x if x.starts_with(b"PSTMCFGGEOCIROK*") => {
                Some(StMessageData::ConfigGeofenceCircleConfigureResult(Ok(())))
            }
            x if x.starts_with(b"PSTMCFGGEOCIRERROR*") => {
                Some(StMessageData::ConfigGeofenceCircleConfigureResult(Err(())))
            }
            x if x.starts_with(b"PSTMLOWPOWERERROR*") => {
                Some(StMessageData::ConfigLowPowerOnOffResult(Err(())))
            }
            x if x.starts_with(b"PSTMLOWPOWERON,") => Some(
                StMessageData::ConfigLowPowerOnOffResult(Ok(LowPowerOnOff::parse(self).unwrap())),
            ),
            x if x.starts_with(b"PSTMCFGLPAOK*") => Some(StMessageData::ConfigLpaResult(Ok(()))),
            x if x.starts_with(b"PSTMCFGLPAERROR*") => {
                Some(StMessageData::ConfigLpaResult(Err(())))
            }
            x if x.starts_with(b"PSTMVER,STA80") => {
                HardwareVersion::parse(self).map(StMessageData::HardwareVersion)
            }
            x if x.starts_with(b"PSTMVER,") => {
                SoftwareVersion::parse(self).map(StMessageData::SoftwareVersion)
            }
            x if x.starts_with(b"PSTMSTANDBYENABLE,") => {
                StandbyEnableStatus::parse(self).map(StMessageData::StandbyEnableStatus)
            }
            x if x.starts_with(b"PSTMSTANDBYENABLEOK*") => {
                Some(StMessageData::ConfigStandbyEnableResult(Ok(())))
            }
            x if x.starts_with(b"PSTMSTANDBYENABLEERROR*") => {
                Some(StMessageData::ConfigStandbyEnableResult(Err(())))
            }
            x if x.starts_with(b"PSTMFORCESTANDBYOK*") => {
                Some(StMessageData::ConfigStandbyForceResult(Ok(())))
            }
            x if x.starts_with(b"PSTMFORCESTANDBYERROR*") => {
                Some(StMessageData::ConfigStandbyForceResult(Err(())))
            }
            x if x.starts_with(b"PSTMGETUCODEOK,") => {
                GetUniqueCode::parse(self).map(|b| StMessageData::GetUniqueCode(Ok(b)))
            }
            x if x.starts_with(b"PSTMGETUCODEERROR*") => {
                Some(StMessageData::GetUniqueCode(Err(())))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::message::StMessageData;
    use crate::NmeaMessage;

    #[test]
    fn configure_anti_jam_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (b"$PSTMCFGAJMOK*1D\r\n" as &[u8], true),
            (b"$PSTMCFGAJMERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigAntiJamResult(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_geofence_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (b"$PSTMCFGGEOFENCEOK*1D\r\n" as &[u8], true),
            (b"$PSTMCFGGEOFENCEERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigGeofenceEnableResult(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_geofence_circle_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (b"$PSTMCFGGEOCIROK*1D\r\n" as &[u8], true),
            (b"$PSTMCFGGEOCIRERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigGeofenceCircleConfigureResult(
                    val,
                )) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_low_power_on_off_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (
                b"$PSTMLOWPOWERON,100,12,1,1,1,1,10000,10,1,1,10,20*0B\r\n" as &[u8],
                true,
            ),
            (b"$PSTMLOWPOWERERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigLowPowerOnOffResult(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_lpa_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (b"$PSTMCFGLPAOK*1D\r\n" as &[u8], true),
            (b"$PSTMCFGLPAERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigLpaResult(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_standby_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (b"$PSTMSTANDBYENABLEOK*1D\r\n" as &[u8], true),
            (b"$PSTMSTANDBYENABLEERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigStandbyEnableResult(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_standby_force_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (b"$PSTMFORCESTANDBYOK*1D\r\n" as &[u8], true),
            (b"$PSTMFORCESTANDBYERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::ConfigStandbyForceResult(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }

    #[test]
    fn configure_get_unique_code_result_message() {
        let parser = crate::NmeaParser::new();
        [
            (
                b"$PSTMGETUCODEOK,0123456789ABCDEF0123456789ABCDEF*1D\r\n" as &[u8],
                true,
            ),
            (b"$PSTMGETUCODEERROR*1D\r\n", false),
        ]
        .iter()
        .for_each(|(cmd, result)| {
            let (msg, _i) = parser.parse_bytes(cmd).unwrap();
            let msg = msg.unwrap();
            let msg = match msg {
                NmeaMessage::StPropriety(StMessageData::GetUniqueCode(val)) => val,
                _ => panic!("Unexpected message type"),
            };
            assert_eq!(*result, msg.is_ok());
        });
    }
}
