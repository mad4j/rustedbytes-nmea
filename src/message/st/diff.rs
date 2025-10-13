//! $PSTMDIFF
//! Differential Correction Data
//! Parameter   Format              Description
//! ListSize    Decimal, 2 digits   Amount of visible satellites in this message (n)
//! NCS         Decimal, 2 digits   Number of corrected satellites
//! SatxID      Decimal, 2 digits   Satellite x ID (PRN)
//! CorrxAvl    Decimal             Correction available for Satellite
//!
//! $PSTMDIFF,‹ListSize>,<NCS>,[<Sat1ID>,<Corr1Avl>,] ... [<SatNID>,<CorrNAv1>,]*<checksum><cr><lf>

use crate::message::ParsedSentence;
use heapless::Vec;

pub const SATELLITE_CORRECTION_AMOUNT: usize = 12;

#[derive(Debug, Clone)]
pub struct SatelliteCorrection {
    pub satellite_id: u16,
    pub correction_available: u32,
}

#[derive(Debug, Clone)]
pub struct DifferentialCorrectionData {
    pub list_size: u8,
    pub number_of_corrected_satellites: u8,
    pub satellites: Vec<SatelliteCorrection, SATELLITE_CORRECTION_AMOUNT>,
}

impl DifferentialCorrectionData {
    pub(crate) fn parse(sentence: &ParsedSentence) -> Option<Self> {
        let list_size = sentence.parse_field::<u8>(1)?;
        let number_of_corrected_satellites = sentence.parse_field::<u8>(2)?;
        let mut satellites = Vec::new();

        let len = (sentence.field_count - 3) / 2;

        for i in 0..len {
            let satellite_id = sentence.parse_field::<u16>(3 + i * 2)?;
            let correction_available = sentence.parse_field::<u32>(3 + i * 2 + 1)?;
            if satellites
                .push(SatelliteCorrection {
                    satellite_id,
                    correction_available,
                })
                .is_err()
            {
                // We've reached the arbitrary SATELLITE_CORRECTION_AMOUNT satellite limit
                break;
            }
        }

        Some(DifferentialCorrectionData {
            list_size,
            number_of_corrected_satellites,
            satellites,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::StMessageData;
    use crate::{MessageType, NmeaParser, TalkerId};

    #[test]
    fn test_parse_differential_correction_data() {
        let mut parser = NmeaParser::new();
        let (msg, _size) = parser
            .parse_bytes(b"$PSTMDIFF,03,02,01,100,02,200,03,300*1C\r\n")
            .unwrap();
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::PSTM);
        assert_eq!(msg.talker_id(), TalkerId::PSTM);
        let data = msg.as_st().unwrap();
        match data {
            StMessageData::DifferentialCorrectionData(data) => {
                assert_eq!(data.list_size, 3);
                assert_eq!(data.number_of_corrected_satellites, 2);
                assert_eq!(data.satellites.len(), 3);

                assert_eq!(data.satellites[0].satellite_id, 1);
                assert_eq!(data.satellites[0].correction_available, 100);

                assert_eq!(data.satellites[1].satellite_id, 2);
                assert_eq!(data.satellites[1].correction_available, 200);

                assert_eq!(data.satellites[2].satellite_id, 3);
                assert_eq!(data.satellites[2].correction_available, 300);
            }
            _ => panic!("Expected DifferentialCorrectionData"),
        }
    }
}
