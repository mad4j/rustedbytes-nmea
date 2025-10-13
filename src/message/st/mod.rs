use crate::message::st::diff::DifferentialCorrectionData;
use crate::message::st::hw_version::HardwareVersion;
use crate::message::st::low_power_on_off::LowPowerOnOff;
use crate::message::st::sw_version::SoftwareVersion;
use crate::message::st::tg::TimeAndSatelliteInformation;
use crate::message::ParsedSentence;
use crate::message::StMessageData::{
    ConfigAntiJamResult, ConfigGeofenceCircleConfigureResult, ConfigGeofenceEnableResult,
    ConfigLowPowerOnOffResult, ConfigLpaResult, ConfigOdometerResult,
};
use crate::MessageType;

mod diff;
mod hw_version;
mod low_power_on_off;
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
    ConfigOdometerResult(Result<(), ()>),
    SoftwareVersion(SoftwareVersion),
    HardwareVersion(HardwareVersion),
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
            x if x.starts_with(b"PSTMCFGAJMOK*") => Some(ConfigAntiJamResult(Ok(()))),
            x if x.starts_with(b"PSTMCFGAJMERROR*") => Some(ConfigAntiJamResult(Err(()))),
            x if x.starts_with(b"PSTMCFGGEOFENCEOK*") => Some(ConfigGeofenceEnableResult(Ok(()))),
            x if x.starts_with(b"PSTMCFGGEOFENCEERROR*") => {
                Some(ConfigGeofenceEnableResult(Err(())))
            }
            x if x.starts_with(b"PSTMCFGGEOCIROK*") => {
                Some(ConfigGeofenceCircleConfigureResult(Ok(())))
            }
            x if x.starts_with(b"PSTMCFGGEOCIRERROR*") => {
                Some(ConfigGeofenceCircleConfigureResult(Err(())))
            }
            x if x.starts_with(b"PSTMLOWPOWERERROR*") => Some(ConfigLowPowerOnOffResult(Err(()))),
            x if x.starts_with(b"PSTMLOWPOWERON,") => Some(ConfigLowPowerOnOffResult(Ok(
                LowPowerOnOff::parse(self).unwrap(),
            ))),
            x if x.starts_with(b"PSTMCFGLPAOK*") => Some(ConfigLpaResult(Ok(()))),
            x if x.starts_with(b"PSTMCFGLPAERROR*") => Some(ConfigLpaResult(Err(()))),
            x if x.starts_with(b"PSTMCFGGEOFENCEOK*") => Some(ConfigOdometerResult(Ok(()))),
            x if x.starts_with(b"PSTMCFGGEOFENCEERROR*") => Some(ConfigOdometerResult(Err(()))),
            x if x.starts_with(b"PSTMVER,STA80") => {
                HardwareVersion::parse(self).map(|b| StMessageData::HardwareVersion(b))
            }
            x if x.starts_with(b"PSTMVER,") => {
                SoftwareVersion::parse(self).map(|b| StMessageData::SoftwareVersion(b))
            }
            _ => None,
        }
    }
}
