//! RMC (Recommended Minimum Navigation Information) message implementation
//!
//! The RMC message is one of the most important NMEA sentences as it provides
//! minimum GPS/GNSS fix data including time, date, position, speed, and course.
//! It's commonly referred to as the "Recommended Minimum" sentence.
//!
//! ## Message Format
//!
//! ```text
//! $GPRMC,hhmmss.ss,A,llll.ll,a,yyyyy.yy,a,x.x,x.x,ddmmyy,x.x,a*hh
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPRMC, GNRMC, etc.) |
//! | 1 | UTC Time | String | Yes | hhmmss.ss format |
//! | 2 | Status | char | Yes | A = Valid, V = Invalid |
//! | 3 | Latitude | f64 | Yes | ddmm.mmmmm format |
//! | 4 | N/S Indicator | char | Yes | N = North, S = South |
//! | 5 | Longitude | f64 | Yes | dddmm.mmmmm format |
//! | 6 | E/W Indicator | char | Yes | E = East, W = West |
//! | 7 | Speed (knots) | f32 | Yes | Speed over ground in knots |
//! | 8 | Track Angle | f32 | Yes | Track angle in degrees |
//! | 9 | Date | String | Yes | ddmmyy format |
//! | 10 | Mag Variation | f32 | No | Magnetic variation in degrees |
//! | 11 | Mag Var Dir | char | No | E = East, W = West |
//!
//! ## Example
//!
//! ```text
//! $GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A
//! ```
//!
//! This represents:
//! - Time: 12:35:19 UTC
//! - Status: Active (valid fix)
//! - Position: 48°07.038'N, 11°31.000'E
//! - Speed: 22.4 knots
//! - Track angle: 84.4°
//! - Date: March 23, 1994
//! - Magnetic variation: 3.1° West

use crate::message::ParsedSentence;
use crate::types::{MessageType, TalkerId};

/// RMC - Recommended Minimum Navigation Information parameters
#[derive(Debug, Clone)]
pub struct RmcData {
    pub talker_id: TalkerId,
    time_data: [u8; 16],
    time_len: u8,
    pub status: char,
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    pub speed_knots: f32,
    pub track_angle: f32,
    date_data: [u8; 8],
    date_len: u8,
    pub magnetic_variation: Option<f32>,
    pub mag_var_direction: Option<char>,
}

impl RmcData {
    /// Get time as string slice
    pub fn time(&self) -> &str {
        core::str::from_utf8(&self.time_data[..self.time_len as usize]).unwrap_or("")
    }

    /// Get date as string slice
    pub fn date(&self) -> &str {
        core::str::from_utf8(&self.date_data[..self.date_len as usize]).unwrap_or("")
    }
}

impl ParsedSentence {
    /// Extract RMC message parameters
    ///
    /// Parses the RMC (Recommended Minimum Navigation Information) message and
    /// returns a structured `RmcData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(RmcData)` if the message is a valid RMC message with all mandatory fields
    /// - `None` if:
    ///   - The message is not an RMC message
    ///   - Any mandatory field is missing or invalid
    ///
    /// # Mandatory Fields
    ///
    /// - Time (field 1)
    /// - Status (field 2) - 'A' for active, 'V' for void
    /// - Latitude (field 3)
    /// - Latitude direction (field 4)
    /// - Longitude (field 5)
    /// - Longitude direction (field 6)
    /// - Speed in knots (field 7)
    /// - Track angle (field 8)
    /// - Date (field 9)
    ///
    /// # Optional Fields
    ///
    /// - Magnetic variation and direction are optional
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let mut parser = NmeaParser::new();
    /// let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";
    ///
    /// for &c in sentence.iter() {
    ///     if let Some(msg) = parser.parse_char(c) {
    ///         if let Some(rmc) = msg.as_rmc() {
    ///             assert_eq!(rmc.time(), "123519");
    ///             assert_eq!(rmc.status, 'A');
    ///             assert_eq!(rmc.speed_knots, 22.4);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn as_rmc(&self) -> Option<RmcData> {
        if self.message_type != MessageType::RMC {
            return None;
        }

        // Validate mandatory fields
        let time_str = self.get_field_str(1)?;
        let status = self.parse_field_char(2)?;
        let latitude = self.parse_field_f64(3)?;
        let lat_direction = self.parse_field_char(4)?;
        let longitude = self.parse_field_f64(5)?;
        let lon_direction = self.parse_field_char(6)?;
        let speed_knots = self.parse_field_f32(7)?;
        let track_angle = self.parse_field_f32(8)?;
        let date_str = self.get_field_str(9)?;

        // Copy time to fixed array
        let mut time_data = [0u8; 16];
        let time_bytes = time_str.as_bytes();
        let time_len = time_bytes.len().min(16) as u8;
        time_data[..time_len as usize].copy_from_slice(&time_bytes[..time_len as usize]);

        // Copy date to fixed array
        let mut date_data = [0u8; 8];
        let date_bytes = date_str.as_bytes();
        let date_len = date_bytes.len().min(8) as u8;
        date_data[..date_len as usize].copy_from_slice(&date_bytes[..date_len as usize]);

        Some(RmcData {
            talker_id: self.talker_id,
            time_data,
            time_len,
            status,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            speed_knots,
            track_angle,
            date_data,
            date_len,
            magnetic_variation: self.parse_field_f32(10),
            mag_var_direction: self.parse_field_char(11),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_rmc_complete_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.time(), "123519");
        assert_eq!(rmc_data.status, 'A');
        assert_eq!(rmc_data.latitude, 4807.038);
        assert_eq!(rmc_data.lat_direction, 'N');
        assert_eq!(rmc_data.longitude, 1131.000);
        assert_eq!(rmc_data.lon_direction, 'E');
        assert_eq!(rmc_data.speed_knots, 22.4);
        assert_eq!(rmc_data.track_angle, 84.4);
        assert_eq!(rmc_data.date(), "230394");
        assert_eq!(rmc_data.magnetic_variation, Some(3.1));
        assert_eq!(rmc_data.mag_var_direction, Some('W'));
    }

    #[test]
    fn test_rmc_void_status() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,V,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.status, 'V');
    }

    #[test]
    fn test_rmc_without_magnetic_variation() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,,*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.magnetic_variation, None);
        assert_eq!(rmc_data.mag_var_direction, None);
    }

    #[test]
    fn test_rmc_missing_time() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_rmc_missing_status() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_rmc_missing_date() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_rmc_missing_speed() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_rmc_missing_track_angle() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_rmc_zero_speed() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,0.0,0.0,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.speed_knots, 0.0);
        assert_eq!(rmc_data.track_angle, 0.0);
    }

    #[test]
    fn test_rmc_numeric_precision() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert!((rmc_data.latitude - 4807.038).abs() < 0.001);
        assert!((rmc_data.longitude - 1131.000).abs() < 0.001);
        assert!((rmc_data.speed_knots - 22.4).abs() < 0.1);
        assert!((rmc_data.track_angle - 84.4).abs() < 0.1);
    }

    #[test]
    fn test_rmc_different_talker_id() {
        let mut parser = NmeaParser::new();
        // GNRMC is multi-GNSS
        let sentence = b"$GNRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());
        
        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.talker_id, crate::types::TalkerId::GN);
    }

    #[test]
    fn test_rmc_multiple_constellations() {
        let mut parser = NmeaParser::new();
        
        // Test GPS
        let gp_sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";
        let mut gp_result = None;
        for &c in gp_sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                gp_result = Some(msg);
            }
        }
        assert!(gp_result.is_some());
        let gp_msg = gp_result.unwrap();
        let gp_rmc = gp_msg.as_rmc().unwrap();
        assert_eq!(gp_rmc.talker_id, crate::types::TalkerId::GP);
        
        // Test GLONASS
        let gl_sentence = b"$GLRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";
        let mut gl_result = None;
        for &c in gl_sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                gl_result = Some(msg);
            }
        }
        assert!(gl_result.is_some());
        let gl_msg = gl_result.unwrap();
        let gl_rmc = gl_msg.as_rmc().unwrap();
        assert_eq!(gl_rmc.talker_id, crate::types::TalkerId::GL);
        
        // Test Galileo
        let ga_sentence = b"$GARMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";
        let mut ga_result = None;
        for &c in ga_sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                ga_result = Some(msg);
            }
        }
        assert!(ga_result.is_some());
        let ga_msg = ga_result.unwrap();
        let ga_rmc = ga_msg.as_rmc().unwrap();
        assert_eq!(ga_rmc.talker_id, crate::types::TalkerId::GA);
    }
}

