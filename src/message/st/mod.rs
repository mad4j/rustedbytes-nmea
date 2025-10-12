use crate::message::st::diff::DifferentialCorrectionData;
use crate::message::st::tg::TimeAndSatelliteInformation;
use crate::message::ParsedSentence;
use crate::message::StMessageData::{ConfigAntiJamResult, ConfigGeofenceCircleConfigureResult, ConfigGeofenceEnableResult, ConfigLpaResult, ConfigOdometerResult};
use crate::MessageType;

mod diff;
mod tg;

#[cfg(feature = "st-teseo-liv3")]
#[derive(Debug, Clone)]
pub enum StMessageData {
    DifferentialCorrectionData(DifferentialCorrectionData),
    TimeAndSatelliteInformation(TimeAndSatelliteInformation),
    ConfigAntiJamResult(bool),
    ConfigGeofenceEnableResult(bool),
    ConfigGeofenceCircleConfigureResult(bool),
    ConfigLpaResult(bool),
    ConfigOdometerResult(bool)
}

impl ParsedSentence {
    pub fn as_st(&self, buffer: &[u8]) -> Option<StMessageData> {
        if self.message_type != MessageType::PSTM {
            return None;
        }

        match buffer {
            x if x.starts_with(b"PSTMDIFF") => DifferentialCorrectionData::parse(self)
                .map(|d| StMessageData::DifferentialCorrectionData(d)),
            x if x.starts_with(b"PSTMTG") => TimeAndSatelliteInformation::parse(self)
                .map(|d| StMessageData::TimeAndSatelliteInformation(d)),
            x if x.starts_with(b"PSTMCFGAJMOK") => Some(ConfigAntiJamResult(true)),
            x if x.starts_with(b"PSTMCFGAJMERROR") => Some(ConfigAntiJamResult(false)),
            x if x.starts_with(b"PSTMCFGGEOFENCEOK") => Some(ConfigGeofenceEnableResult(true)),
            x if x.starts_with(b"PSTMCFGGEOFENCEERROR") => Some(ConfigGeofenceEnableResult(false)),
            x if x.starts_with(b"PSTMCFGGEOCIROK") => Some(ConfigGeofenceCircleConfigureResult(true)),
            x if x.starts_with(b"PSTMCFGGEOCIRERROR") => Some(ConfigGeofenceCircleConfigureResult(false)),
            x if x.starts_with(b"PSTMCFGLPAOK*") => Some(ConfigLpaResult(true)),
            x if x.starts_with(b"PSTMCFGLPAERROR*") => Some(ConfigLpaResult(false)),
            x if x.starts_with(b"PSTMCFGGEOFENCEOK*") => Some(ConfigOdometerResult(true)),
            x if x.starts_with(b"PSTMCFGGEOFENCEERROR*") => Some(ConfigOdometerResult(false)),
            _ => None,
        }
    }
}
