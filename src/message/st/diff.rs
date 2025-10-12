//! $PSTMDIFF
//! Differential Correction Data
//! Parameter   Format              Description
//! ListSize    Decimal, 2 digits   Amount of visible satellites in this message (n)
//! NCS         Decimal, 2 digits   Number of corrected satellites
//! SatxID      Decimal, 2 digits   Satellite x ID (PRN)
//! CorrxAvl    Decimal             Correction available for Satellite
//
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

        for i in 3..sentence.field_count {
            let satellite_id = sentence.parse_field::<u16>(i)?;
            let correction_available = sentence.parse_field::<u32>(i + 1)?;
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
