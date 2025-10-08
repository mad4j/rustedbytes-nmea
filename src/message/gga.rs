//! GGA (Global Positioning System Fix Data) message implementation
//!
//! The GGA message provides essential GPS fix data including time, position,
//! fix quality, number of satellites, horizontal dilution of precision (HDOP),
//! altitude, and geoid separation.
//!
//! ## Message Format
//!
//! ```text
//! $GPGGA,hhmmss.ss,llll.ll,a,yyyyy.yy,a,x,xx,x.x,x.x,M,x.x,M,x.x,xxxx*hh
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPGGA, GNGGA, etc.) |
//! | 1 | UTC Time | String | Yes | hhmmss.ss format |
//! | 2 | Latitude | f64 | Yes | ddmm.mmmmm format |
//! | 3 | N/S Indicator | char | Yes | N = North, S = South |
//! | 4 | Longitude | f64 | Yes | dddmm.mmmmm format |
//! | 5 | E/W Indicator | char | Yes | E = East, W = West |
//! | 6 | Fix Quality | u8 | Yes | 0=Invalid, 1=GPS, 2=DGPS, etc. |
//! | 7 | Satellites | u8 | No | Number of satellites in use |
//! | 8 | HDOP | f32 | No | Horizontal dilution of precision |
//! | 9 | Altitude | f32 | No | Altitude above mean sea level |
//! | 10 | Altitude Units | char | No | M = meters |
//! | 11 | Geoid Sep | f32 | No | Geoid separation |
//! | 12 | Geoid Units | char | No | M = meters |
//! | 13 | Age of Diff | f32 | No | Age of differential corrections |
//! | 14 | Diff Station ID | String | No | Differential station ID |
//!
//! ## Example
//!
//! ```text
//! $GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47
//! ```
//!
//! This represents:
//! - Time: 12:35:19 UTC
//! - Position: 48°07.038'N, 11°31.000'E
//! - Fix quality: GPS fix
//! - 8 satellites in use
//! - HDOP: 0.9
//! - Altitude: 545.4 meters above MSL
//! - Geoid separation: 46.9 meters

use crate::message::ParsedSentence;
use crate::types::{MessageType, TalkerId};

/// GGA - Global Positioning System Fix Data parameters
#[derive(Debug, Clone)]
pub struct GgaData {
    pub talker_id: TalkerId,
    time_data: [u8; 16],
    time_len: u8,
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    pub fix_quality: u8,
    pub num_satellites: Option<u8>,
    pub hdop: Option<f32>,
    pub altitude: Option<f32>,
    pub altitude_units: Option<char>,
    pub geoid_separation: Option<f32>,
    pub geoid_units: Option<char>,
    pub age_of_diff: Option<f32>,
    diff_station_id_data: [u8; 8],
    diff_station_id_len: u8,
    /// Local reception timestamp in milliseconds (optional, set by user)
    pub local_timestamp_ms: Option<u64>,
}

impl GgaData {
    /// Get time as string slice
    pub fn time(&self) -> &str {
        core::str::from_utf8(&self.time_data[..self.time_len as usize]).unwrap_or("")
    }

    /// Get differential station ID as string slice (if present)
    pub fn diff_station_id(&self) -> Option<&str> {
        if self.diff_station_id_len > 0 {
            core::str::from_utf8(&self.diff_station_id_data[..self.diff_station_id_len as usize]).ok()
        } else {
            None
        }
    }
}

impl ParsedSentence {
    /// Extract GGA message parameters
    ///
    /// Parses the GGA (Global Positioning System Fix Data) message and returns
    /// a structured `GgaData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(GgaData)` if the message is a valid GGA message with all mandatory fields
    /// - `None` if:
    ///   - The message is not a GGA message
    ///   - Any mandatory field is missing or invalid
    ///
    /// # Mandatory Fields
    ///
    /// - Time (field 1)
    /// - Latitude (field 2)
    /// - Latitude direction (field 3)
    /// - Longitude (field 4)
    /// - Longitude direction (field 5)
    /// - Fix quality (field 6)
    ///
    /// # Optional Fields
    ///
    /// All other fields are optional and will be `None` if not present or invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let parser = NmeaParser::new();
    /// let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
    ///
    /// let result = parser.parse_bytes(sentence);
    /// if let Ok((Some(msg), _consumed)) = result {
    ///     if let Some(gga) = msg.as_gga() {
    ///         assert_eq!(gga.time(), "123519");
    ///         assert_eq!(gga.latitude, 4807.038);
    ///         assert_eq!(gga.fix_quality, 1);
    ///     }
    /// }
    /// ```
    pub fn as_gga(&self) -> Option<GgaData> {
        if self.message_type != MessageType::GGA {
            return None;
        }

        // Validate mandatory fields
        let time_str = self.get_field_str(1)?;
        let latitude: f64 = self.parse_field(2)?;
        let lat_direction = self.parse_field_char(3)?;
        let longitude: f64 = self.parse_field(4)?;
        let lon_direction = self.parse_field_char(5)?;
        let fix_quality: u8 = self.parse_field(6)?;

        // Copy time string to fixed array
        let mut time_data = [0u8; 16];
        let time_bytes = time_str.as_bytes();
        let time_len = time_bytes.len().min(16) as u8;
        time_data[..time_len as usize].copy_from_slice(&time_bytes[..time_len as usize]);

        // Copy diff station ID if present
        let mut diff_station_id_data = [0u8; 8];
        let diff_station_id_len = if let Some(id_str) = self.get_field_str(14) {
            let id_bytes = id_str.as_bytes();
            let len = id_bytes.len().min(8) as u8;
            diff_station_id_data[..len as usize].copy_from_slice(&id_bytes[..len as usize]);
            len
        } else {
            0
        };

        Some(GgaData {
            talker_id: self.talker_id,
            time_data,
            time_len,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            fix_quality,
            num_satellites: self.parse_field(7),
            hdop: self.parse_field(8),
            altitude: self.parse_field(9),
            altitude_units: self.parse_field_char(10),
            geoid_separation: self.parse_field(11),
            geoid_units: self.parse_field_char(12),
            age_of_diff: self.parse_field(13),
            diff_station_id_data,
            diff_station_id_len,
            local_timestamp_ms: self.local_timestamp_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_gga_complete_message() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.time(), "123519");
        assert_eq!(gga_data.latitude, 4807.038);
        assert_eq!(gga_data.lat_direction, 'N');
        assert_eq!(gga_data.longitude, 1131.000);
        assert_eq!(gga_data.lon_direction, 'E');
        assert_eq!(gga_data.fix_quality, 1);
        assert_eq!(gga_data.num_satellites, Some(8));
        assert_eq!(gga_data.hdop, Some(0.9));
        assert_eq!(gga_data.altitude, Some(545.4));
        assert_eq!(gga_data.altitude_units, Some('M'));
        assert_eq!(gga_data.geoid_separation, Some(46.9));
        assert_eq!(gga_data.geoid_units, Some('M'));
        assert_eq!(gga_data.age_of_diff, None);
        assert_eq!(gga_data.diff_station_id(), None);
    }

    #[test]
    fn test_gga_with_empty_optional_fields() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,,,,,M,,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.time(), "123519");
        assert_eq!(gga_data.latitude, 4807.038);
        assert_eq!(gga_data.fix_quality, 1);
        assert_eq!(gga_data.num_satellites, None);
        assert_eq!(gga_data.hdop, None);
        assert_eq!(gga_data.altitude, None);
    }

    #[test]
    fn test_gga_missing_time() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because time is mandatory
        assert!(result.is_none());
    }

    #[test]
    fn test_gga_missing_latitude() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gga_missing_longitude() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gga_missing_fix_quality() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gga_invalid_latitude_format() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,INVALID,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gga_with_differential_data() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGGA,123519,4807.038,N,01131.000,E,2,08,0.9,545.4,M,46.9,M,3.2,0120*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.fix_quality, 2);
        assert_eq!(gga_data.age_of_diff, Some(3.2));
        assert_eq!(gga_data.diff_station_id(), Some("0120"));
    }

    #[test]
    fn test_gga_numeric_precision() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert!((gga_data.latitude - 4807.038).abs() < 0.001);
        assert!((gga_data.longitude - 1131.000).abs() < 0.001);

        if let Some(hdop) = gga_data.hdop {
            assert!((hdop - 0.9).abs() < 0.01);
        }

        if let Some(alt) = gga_data.altitude {
            assert!((alt - 545.4).abs() < 0.1);
        }
    }

    #[test]
    fn test_gga_different_talker_id() {
        let parser = NmeaParser::new();
        // GNGGA is multi-GNSS (GPS + GLONASS + others)
        let sentence = b"$GNGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());
        
        let gga_data = gga.unwrap();
        assert_eq!(gga_data.talker_id, crate::types::TalkerId::GN);
    }

    #[test]
    fn test_gga_gps_talker_id() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());
        
        let gga_data = gga.unwrap();
        assert_eq!(gga_data.talker_id, crate::types::TalkerId::GP);
    }

    #[test]
    fn test_gga_glonass_talker_id() {
        let parser = NmeaParser::new();
        let sentence = b"$GLGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());
        
        let gga_data = gga.unwrap();
        assert_eq!(gga_data.talker_id, crate::types::TalkerId::GL);
    }

    #[test]
    fn test_gga_galileo_talker_id() {
        let parser = NmeaParser::new();
        let sentence = b"$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());
        
        let gga_data = gga.unwrap();
        assert_eq!(gga_data.talker_id, crate::types::TalkerId::GA);
    }

    #[test]
    fn test_gga_beidou_talker_id() {
        let parser = NmeaParser::new();
        let sentence = b"$GBGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());
        
        let gga_data = gga.unwrap();
        assert_eq!(gga_data.talker_id, crate::types::TalkerId::GB);
    }
}

