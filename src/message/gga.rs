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

use crate::message::NmeaMessage;
use crate::types::{GgaData, MessageType};

impl NmeaMessage {
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
    /// let mut parser = NmeaParser::new();
    /// let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
    ///
    /// for &c in sentence.iter() {
    ///     if let Some(msg) = parser.parse_char(c) {
    ///         if let Some(gga) = msg.as_gga() {
    ///             assert_eq!(gga.time, "123519");
    ///             assert_eq!(gga.latitude, 4807.038);
    ///             assert_eq!(gga.fix_quality, 1);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn as_gga(&self) -> Option<GgaData<'_>> {
        if self.message_type != MessageType::GGA {
            return None;
        }

        // Validate mandatory fields
        let time = self.get_field_str(1)?;
        let latitude = self.parse_field_f64(2)?;
        let lat_direction = self.parse_field_char(3)?;
        let longitude = self.parse_field_f64(4)?;
        let lon_direction = self.parse_field_char(5)?;
        let fix_quality = self.parse_field_u8(6)?;

        Some(GgaData {
            time,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            fix_quality,
            num_satellites: self.parse_field_u8(7),
            hdop: self.parse_field_f32(8),
            altitude: self.parse_field_f32(9),
            altitude_units: self.parse_field_char(10),
            geoid_separation: self.parse_field_f32(11),
            geoid_units: self.parse_field_char(12),
            age_of_diff: self.parse_field_f32(13),
            diff_station_id: self.get_field_str(14),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_gga_complete_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.time, "123519");
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
        assert_eq!(gga_data.diff_station_id, None);
    }

    #[test]
    fn test_gga_with_empty_optional_fields() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,,,,,M,,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.time, "123519");
        assert_eq!(gga_data.latitude, 4807.038);
        assert_eq!(gga_data.fix_quality, 1);
        assert_eq!(gga_data.num_satellites, None);
        assert_eq!(gga_data.hdop, None);
        assert_eq!(gga_data.altitude, None);
    }

    #[test]
    fn test_gga_missing_time() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_none());
    }

    #[test]
    fn test_gga_missing_latitude() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_none());
    }

    #[test]
    fn test_gga_missing_longitude() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_none());
    }

    #[test]
    fn test_gga_missing_fix_quality() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_none());
    }

    #[test]
    fn test_gga_invalid_latitude_format() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,INVALID,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_none());
    }

    #[test]
    fn test_gga_with_differential_data() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,2,08,0.9,545.4,M,46.9,M,3.2,0120*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.fix_quality, 2);
        assert_eq!(gga_data.age_of_diff, Some(3.2));
        assert_eq!(gga_data.diff_station_id, Some("0120"));
    }

    #[test]
    fn test_gga_numeric_precision() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

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
        let mut parser = NmeaParser::new();
        // GNGGA is multi-GNSS (GPS + GLONASS + others)
        let sentence = b"$GNGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());
    }
}
