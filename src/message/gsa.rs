//! GSA (GPS DOP and Active Satellites) message implementation
//!
//! The GSA message provides information about the GPS receiver operating mode,
//! active satellites used for navigation, and dilution of precision (DOP) values.
//! DOP values indicate the quality of the satellite geometry.
//!
//! ## Message Format
//!
//! ```text
//! $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPGSA, GNGSA, etc.) |
//! | 1 | Mode | char | Yes | M = Manual, A = Automatic |
//! | 2 | Fix Type | u8 | Yes | 1=No fix, 2=2D, 3=3D |
//! | 3-14 | Satellite IDs | u8 | No | PRNs of satellites used (up to 12) |
//! | 15 | PDOP | f32 | No | Position dilution of precision |
//! | 16 | HDOP | f32 | No | Horizontal dilution of precision |
//! | 17 | VDOP | f32 | No | Vertical dilution of precision |
//!
//! ## DOP Values
//!
//! DOP (Dilution of Precision) values indicate satellite geometry quality:
//! - **PDOP**: Position (3D) dilution of precision
//! - **HDOP**: Horizontal (2D) dilution of precision
//! - **VDOP**: Vertical dilution of precision
//!
//! Lower values indicate better precision:
//! - < 1: Ideal
//! - 1-2: Excellent
//! - 2-5: Good
//! - 5-10: Moderate
//! - 10-20: Fair
//! - > 20: Poor
//!
//! ## Example
//!
//! ```text
//! $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39
//! ```
//!
//! This represents:
//! - Mode: Automatic
//! - Fix type: 3D fix
//! - Satellites: PRN 04, 05, 09, 12, 24
//! - PDOP: 2.5
//! - HDOP: 1.3
//! - VDOP: 2.1

use crate::message::ParsedSentence;
use crate::types::{MessageType, TalkerId};

/// GSA - GPS DOP and active satellites parameters
#[derive(Debug, Clone)]
pub struct GsaData {
    pub talker_id: TalkerId,
    pub mode: char,
    pub fix_type: u8,
    pub satellite_ids: [Option<u8>; 12],
    pub pdop: Option<f32>,
    pub hdop: Option<f32>,
    pub vdop: Option<f32>,
}

impl ParsedSentence {
    /// Extract GSA message parameters
    ///
    /// Parses the GSA (GPS DOP and Active Satellites) message and returns
    /// a structured `GsaData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(GsaData)` if the message is a valid GSA message with all mandatory fields
    /// - `None` if:
    ///   - The message is not a GSA message
    ///   - Any mandatory field is missing or invalid
    ///
    /// # Mandatory Fields
    ///
    /// - Mode (field 1) - 'M' for manual, 'A' for automatic
    /// - Fix type (field 2) - 1 for no fix, 2 for 2D, 3 for 3D
    ///
    /// # Optional Fields
    ///
    /// - Satellite IDs (fields 3-14) - up to 12 satellite PRNs
    /// - PDOP, HDOP, VDOP (fields 15-17)
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let parser = NmeaParser::new();
    /// let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";
    ///
    /// let result = parser.parse_bytes(sentence);
    /// if let Ok((Some(msg), _consumed)) = result {
    ///     if let Some(gsa) = msg.as_gsa() {
    ///         assert_eq!(gsa.mode, 'A');
    ///         assert_eq!(gsa.fix_type, 3);
    ///         assert_eq!(gsa.satellite_ids[0], Some(4));
    ///     }
    /// }
    /// ```
    pub fn as_gsa(&self) -> Option<GsaData> {
        if self.message_type != MessageType::GSA {
            return None;
        }

        // Validate mandatory fields
        let mode = self.parse_field_char(1)?;
        let fix_type: u8 = self.parse_field(2)?;

        Some(GsaData {
            talker_id: self.talker_id,
            mode,
            fix_type,
            satellite_ids: [
                self.parse_field(3),
                self.parse_field(4),
                self.parse_field(5),
                self.parse_field(6),
                self.parse_field(7),
                self.parse_field(8),
                self.parse_field(9),
                self.parse_field(10),
                self.parse_field(11),
                self.parse_field(12),
                self.parse_field(13),
                self.parse_field(14),
            ],
            pdop: self.parse_field(15),
            hdop: self.parse_field(16),
            vdop: self.parse_field(17),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_gsa_complete_message() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.mode, 'A');
        assert_eq!(gsa_data.fix_type, 3);
        assert_eq!(gsa_data.satellite_ids[0], Some(4));
        assert_eq!(gsa_data.satellite_ids[1], Some(5));
        assert_eq!(gsa_data.satellite_ids[2], None);
        assert_eq!(gsa_data.satellite_ids[3], Some(9));
        assert_eq!(gsa_data.satellite_ids[4], Some(12));
        assert_eq!(gsa_data.pdop, Some(2.5));
        assert_eq!(gsa_data.hdop, Some(1.3));
        assert_eq!(gsa_data.vdop, Some(2.1));
    }

    #[test]
    fn test_gsa_manual_mode() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,M,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.mode, 'M');
    }

    #[test]
    fn test_gsa_2d_fix() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,2,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.fix_type, 2);
    }

    #[test]
    fn test_gsa_no_fix() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,1,,,,,,,,,,,,,,,*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.fix_type, 1);
        assert_eq!(gsa_data.satellite_ids[0], None);
    }

    #[test]
    fn test_gsa_partial_satellites() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,01,,,,,,,,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.satellite_ids[0], Some(1));
        assert_eq!(gsa_data.satellite_ids[1], None);
        assert_eq!(gsa_data.satellite_ids[11], None);
    }

    #[test]
    fn test_gsa_all_satellites() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,01,02,03,04,05,06,07,08,09,10,11,12,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        for i in 0..12 {
            assert_eq!(gsa_data.satellite_ids[i], Some((i + 1) as u8));
        }
    }

    #[test]
    fn test_gsa_without_dop() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,,,*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.pdop, None);
        assert_eq!(gsa_data.hdop, None);
        assert_eq!(gsa_data.vdop, None);
    }

    #[test]
    fn test_gsa_missing_mode() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gsa_missing_fix_type() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gsa_dop_precision() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert!((gsa_data.pdop.unwrap() - 2.5).abs() < 0.01);
        assert!((gsa_data.hdop.unwrap() - 1.3).abs() < 0.01);
        assert!((gsa_data.vdop.unwrap() - 2.1).abs() < 0.01);
    }

    #[test]
    fn test_gsa_different_talker_id() {
        let parser = NmeaParser::new();
        // GNGSA is multi-GNSS
        let sentence = b"$GNGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());
        
        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.talker_id, crate::types::TalkerId::GN);
    }

    #[test]
    fn test_gsa_constellation_tracking() {
        let parser = NmeaParser::new();
        
        // Test BeiDou
        let bd_sentence = b"$BDGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";
        let bd_result = parser.parse_sentence_complete(bd_sentence);
        assert!(bd_result.is_some());
        let bd_msg = bd_result.unwrap();
        let bd_gsa = bd_msg.as_gsa().unwrap();
        assert_eq!(bd_gsa.talker_id, crate::types::TalkerId::BD);
        
        // Test QZSS
        let qz_sentence = b"$QZGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";
        let qz_result = parser.parse_sentence_complete(qz_sentence);
        assert!(qz_result.is_some());
        let qz_msg = qz_result.unwrap();
        let qz_gsa = qz_msg.as_gsa().unwrap();
        assert_eq!(qz_gsa.talker_id, crate::types::TalkerId::QZ);
    }
}

