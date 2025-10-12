use crate::message::st::diff::DifferentialCorrectionData;
use crate::message::st::tg::TimeAndSatelliteInformation;
use crate::message::ParsedSentence;
use crate::MessageType;

mod diff;
mod tg;

#[cfg(feature = "st-teseo-liv3")]
#[derive(Debug, Clone)]
pub enum StMessageData {
    DifferentialCorrectionData(DifferentialCorrectionData),
    TimeAndSatelliteInformation(TimeAndSatelliteInformation),
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
            _ => None,
        }
    }
}
