//! GSV (GPS Satellites in View) message implementation
//!
//! The GSV message provides detailed information about satellites in view,
//! including their PRN (Pseudo-Random Noise) number, elevation, azimuth,
//! and signal-to-noise ratio (SNR). This message can span multiple sentences
//! if more than 4 satellites are visible.
//!
//! ## Message Format
//!
//! ```text
//! $GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPGSV, GNGSV, etc.) |
//! | 1 | Num Messages | u8 | Yes | Total number of GSV messages |
//! | 2 | Message Num | u8 | Yes | Current message number (1-based) |
//! | 3 | Satellites | u8 | Yes | Total satellites in view |
//! | 4-7 | Sat 1 Info | - | No | PRN, elevation, azimuth, SNR |
//! | 8-11 | Sat 2 Info | - | No | PRN, elevation, azimuth, SNR |
//! | 12-15 | Sat 3 Info | - | No | PRN, elevation, azimuth, SNR |
//! | 16-19 | Sat 4 Info | - | No | PRN, elevation, azimuth, SNR |
//!
//! ## Satellite Information
//!
//! Each satellite includes 4 fields:
//! - **PRN**: Satellite PRN (Pseudo-Random Noise) number (u8)
//! - **Elevation**: Elevation in degrees, 0-90° (u16)
//! - **Azimuth**: Azimuth in degrees, 0-359° (u16)
//! - **SNR**: Signal-to-noise ratio in dB, 0-99 (u8)
//!
//! ## Multi-Sentence Messages
//!
//! If more than 4 satellites are visible, multiple GSV messages are sent.
//! Each message contains up to 4 satellites. The num_messages field indicates
//! how many total messages to expect, and message_num indicates which message
//! this is in the sequence.
//!
//! ## Example
//!
//! ```text
//! $GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75
//! ```
//!
//! This represents:
//! - 2 total GSV messages
//! - This is message 1 of 2
//! - 8 satellites in view
//! - Satellite 1: PRN=01, elevation=40°, azimuth=83°, SNR=46dB
//! - Satellite 2: PRN=02, elevation=17°, azimuth=308°, SNR=41dB
//! - Satellite 3: PRN=12, elevation=7°, azimuth=344°, SNR=39dB
//! - Satellite 4: PRN=14, elevation=22°, azimuth=228°, SNR=45dB

use crate::message::NmeaMessage;
use crate::types::{GsvData, MessageType, SatelliteInfo};

impl NmeaMessage {
    /// Extract GSV message parameters
    ///
    /// Parses the GSV (GPS Satellites in View) message and returns a structured
    /// `GsvData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(GsvData)` if the message is a valid GSV message with all mandatory fields
    /// - `None` if:
    ///   - The message is not a GSV message
    ///   - Any mandatory field is missing or invalid
    ///
    /// # Mandatory Fields
    ///
    /// - Number of messages (field 1)
    /// - Message number (field 2)
    /// - Total satellites in view (field 3)
    ///
    /// # Optional Fields
    ///
    /// - Up to 4 satellite information blocks (fields 4-19)
    /// - Each satellite block contains: PRN, elevation, azimuth, SNR
    /// - Individual fields within a satellite block are also optional
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let mut parser = NmeaParser::new();
    /// let sentence = b"$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";
    ///
    /// for &c in sentence.iter() {
    ///     if let Some(msg) = parser.parse_char(c) {
    ///         if let Some(gsv) = msg.as_gsv() {
    ///             assert_eq!(gsv.num_messages, 2);
    ///             assert_eq!(gsv.satellites_in_view, 8);
    ///             assert!(gsv.satellite_info[0].is_some());
    ///         }
    ///     }
    /// }
    /// ```
    pub fn as_gsv(&self) -> Option<GsvData> {
        if self.message_type != MessageType::GSV {
            return None;
        }

        // Validate mandatory fields
        let num_messages = self.parse_field_u8(1)?;
        let message_num = self.parse_field_u8(2)?;
        let satellites_in_view = self.parse_field_u8(3)?;

        let sat1 = if self.get_field_str(4).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(4),
                elevation: self.parse_field_u16(5),
                azimuth: self.parse_field_u16(6),
                snr: self.parse_field_u8(7),
            })
        } else {
            None
        };

        let sat2 = if self.get_field_str(8).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(8),
                elevation: self.parse_field_u16(9),
                azimuth: self.parse_field_u16(10),
                snr: self.parse_field_u8(11),
            })
        } else {
            None
        };

        let sat3 = if self.get_field_str(12).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(12),
                elevation: self.parse_field_u16(13),
                azimuth: self.parse_field_u16(14),
                snr: self.parse_field_u8(15),
            })
        } else {
            None
        };

        let sat4 = if self.get_field_str(16).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(16),
                elevation: self.parse_field_u16(17),
                azimuth: self.parse_field_u16(18),
                snr: self.parse_field_u8(19),
            })
        } else {
            None
        };

        Some(GsvData {
            num_messages,
            message_num,
            satellites_in_view,
            satellite_info: [sat1, sat2, sat3, sat4],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_gsv_complete_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.num_messages, 2);
        assert_eq!(gsv_data.message_num, 1);
        assert_eq!(gsv_data.satellites_in_view, 8);

        // Check first satellite
        assert!(gsv_data.satellite_info[0].is_some());
        let sat1 = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat1.prn, Some(1));
        assert_eq!(sat1.elevation, Some(40));
        assert_eq!(sat1.azimuth, Some(83));
        assert_eq!(sat1.snr, Some(46));

        // Check second satellite
        assert!(gsv_data.satellite_info[1].is_some());
        let sat2 = gsv_data.satellite_info[1].as_ref().unwrap();
        assert_eq!(sat2.prn, Some(2));
        assert_eq!(sat2.elevation, Some(17));
        assert_eq!(sat2.azimuth, Some(308));
        assert_eq!(sat2.snr, Some(41));
    }

    #[test]
    fn test_gsv_partial_satellites() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,1,1,02,01,40,083,46,02,17,308,*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.satellites_in_view, 2);

        // First satellite should be complete
        assert!(gsv_data.satellite_info[0].is_some());
        let sat1 = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat1.prn, Some(1));

        // Second satellite should have missing SNR
        assert!(gsv_data.satellite_info[1].is_some());
        let sat2 = gsv_data.satellite_info[1].as_ref().unwrap();
        assert_eq!(sat2.prn, Some(2));
        assert_eq!(sat2.snr, None);

        // Third and fourth should be None
        assert!(gsv_data.satellite_info[2].is_none());
        assert!(gsv_data.satellite_info[3].is_none());
    }

    #[test]
    fn test_gsv_single_satellite() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,1,1,01,01,40,083,46*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.satellites_in_view, 1);
        assert!(gsv_data.satellite_info[0].is_some());
        assert!(gsv_data.satellite_info[1].is_none());
    }

    #[test]
    fn test_gsv_missing_num_messages() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_none());
    }

    #[test]
    fn test_gsv_missing_message_num() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,2,,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_none());
    }

    #[test]
    fn test_gsv_missing_satellites_in_view() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,2,1,,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_none());
    }

    #[test]
    fn test_gsv_satellite_partial_info() {
        let mut parser = NmeaParser::new();
        // Satellite with PRN but missing other fields
        let sentence = b"$GPGSV,1,1,01,01,,,*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        assert!(gsv_data.satellite_info[0].is_some());
        let sat = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat.prn, Some(1));
        assert_eq!(sat.elevation, None);
        assert_eq!(sat.azimuth, None);
        assert_eq!(sat.snr, None);
    }

    #[test]
    fn test_gsv_zero_elevation_azimuth() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,1,1,01,01,0,0,46*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        let sat = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat.elevation, Some(0));
        assert_eq!(sat.azimuth, Some(0));
    }

    #[test]
    fn test_gsv_max_elevation() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,1,1,01,01,90,180,46*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        let sat = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat.elevation, Some(90));
    }

    #[test]
    fn test_gsv_max_azimuth() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,1,1,01,01,45,359,46*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        let sat = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat.azimuth, Some(359));
    }

    #[test]
    fn test_gsv_different_talker_id() {
        let mut parser = NmeaParser::new();
        // GNGSV is multi-GNSS
        let sentence = b"$GNGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());
    }

    #[test]
    fn test_gsv_multiple_message_sequence() {
        let mut parser = NmeaParser::new();

        // First message of sequence
        let sentence1 = b"$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";
        let mut gsv1 = None;
        for &c in sentence1.iter() {
            if let Some(msg) = parser.parse_char(c) {
                gsv1 = msg.as_gsv();
            }
        }
        assert!(gsv1.is_some());
        let gsv1_data = gsv1.unwrap();
        assert_eq!(gsv1_data.message_num, 1);
        assert_eq!(gsv1_data.num_messages, 2);

        // Second message of sequence
        let sentence2 = b"$GPGSV,2,2,08,20,35,073,44,21,25,210,42,25,15,120,40,32,10,045,38*75\r\n";
        let mut gsv2 = None;
        for &c in sentence2.iter() {
            if let Some(msg) = parser.parse_char(c) {
                gsv2 = msg.as_gsv();
            }
        }
        assert!(gsv2.is_some());
        let gsv2_data = gsv2.unwrap();
        assert_eq!(gsv2_data.message_num, 2);
        assert_eq!(gsv2_data.num_messages, 2);
    }
}
