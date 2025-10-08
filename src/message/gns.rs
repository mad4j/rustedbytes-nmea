//! GNS (GNSS Fix Data) message implementation
//!
//! The GNS message provides combined GNSS fix data from multiple satellite systems.
//! It includes time, position, mode indicators for different GNSS systems, number of
//! satellites, HDOP, altitude, geoid separation, and differential data age.
//!
//! ## Message Format
//!
//! ```text
//! $GPGNS,hhmmss.ss,llll.ll,a,yyyyy.yy,a,c,xx,x.x,x.x,x.x,x.x,xxxx,a*hh
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPGNS, GNGNS, etc.) |
//! | 1 | UTC Time | String | Yes | hhmmss.ss format |
//! | 2 | Latitude | f64 | Yes | ddmm.mmmmm format |
//! | 3 | N/S Indicator | char | Yes | N = North, S = South |
//! | 4 | Longitude | f64 | Yes | dddmm.mmmmm format |
//! | 5 | E/W Indicator | char | Yes | E = East, W = West |
//! | 6 | Mode Indicator | String | Yes | Position fix mode for each GNSS |
//! | 7 | Satellites | u8 | Yes | Number of satellites in use |
//! | 8 | HDOP | f32 | No | Horizontal dilution of precision |
//! | 9 | Altitude | f32 | No | Altitude above mean sea level |
//! | 10 | Geoid Sep | f32 | No | Geoid separation |
//! | 11 | Age of Diff | f32 | No | Age of differential corrections |
//! | 12 | Diff Station ID | String | No | Differential station ID |
//! | 13 | Nav Status | char | No | Navigation status indicator |
//!
//! ## Mode Indicator Field
//!
//! The mode indicator (field 6) contains characters representing the positioning mode
//! for each supported GNSS system:
//! - N = No fix
//! - A = Autonomous GNSS fix
//! - D = Differential mode
//! - P = Precise (RTK fixed)
//! - R = RTK float
//! - F = RTK float
//! - E = Estimated (dead reckoning) mode
//! - M = Manual input mode
//! - S = Simulator mode
//!
//! ## Example
//!
//! ```text
//! $GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79
//! ```
//!
//! This represents:
//! - Time: 12:23:10.0 UTC
//! - Position: 37°23.46587'N, 122°02.26957'W
//! - Mode: AAAA (all systems in autonomous mode)
//! - 12 satellites in use
//! - HDOP: 0.9
//! - Altitude: 1005.543 meters above MSL
//! - Geoid separation: 6.5 meters

use crate::message::ParsedSentence;
use crate::types::{MessageType, TalkerId};

/// GNS - GNSS Fix Data parameters
#[derive(Debug, Clone)]
pub struct GnsData {
    pub talker_id: TalkerId,
    time_data: [u8; 16],
    time_len: u8,
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    mode_indicator_data: [u8; 8],
    mode_indicator_len: u8,
    pub num_satellites: u8,
    pub hdop: Option<f32>,
    pub altitude: Option<f32>,
    pub geoid_separation: Option<f32>,
    pub age_of_diff: Option<f32>,
    diff_station_id_data: [u8; 8],
    diff_station_id_len: u8,
    pub nav_status: Option<char>,
}

impl GnsData {
    /// Get time as string slice
    pub fn time(&self) -> &str {
        core::str::from_utf8(&self.time_data[..self.time_len as usize]).unwrap_or("")
    }

    /// Get mode indicator as string slice
    pub fn mode_indicator(&self) -> &str {
        core::str::from_utf8(&self.mode_indicator_data[..self.mode_indicator_len as usize])
            .unwrap_or("")
    }

    /// Get differential station ID as string slice (if present)
    pub fn diff_station_id(&self) -> Option<&str> {
        if self.diff_station_id_len > 0 {
            core::str::from_utf8(&self.diff_station_id_data[..self.diff_station_id_len as usize])
                .ok()
        } else {
            None
        }
    }
}

impl ParsedSentence {
    /// Extract GNS message parameters
    ///
    /// Parses the GNS (GNSS Fix Data) message and returns a structured
    /// `GnsData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(GnsData)` if the message is a valid GNS message with all mandatory fields
    /// - `None` if:
    ///   - The message is not a GNS message
    ///   - Any mandatory field is missing or invalid
    ///
    /// # Mandatory Fields
    ///
    /// - Time (field 1)
    /// - Latitude (field 2)
    /// - Latitude direction (field 3)
    /// - Longitude (field 4)
    /// - Longitude direction (field 5)
    /// - Mode indicator (field 6)
    /// - Number of satellites (field 7)
    ///
    /// # Optional Fields
    ///
    /// - HDOP, altitude, geoid separation, age of differential, station ID, nav status
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let parser = NmeaParser::new();
    /// let sentence = b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";
    ///
    /// let result = parser.parse_bytes(sentence);
    /// if let Ok((Some(msg), _consumed)) = result {
    ///     if let Some(gns) = msg.as_gns() {
    ///         assert_eq!(gns.time(), "122310.0");
    ///         assert_eq!(gns.latitude, 3723.46587);
    ///         assert_eq!(gns.num_satellites, 12);
    ///     }
    /// }
    /// ```
    pub fn as_gns(&self) -> Option<GnsData> {
        if self.message_type != MessageType::GNS {
            return None;
        }

        // Validate mandatory fields
        let time_str = self.get_field_str(1)?;
        let latitude: f64 = self.parse_field(2)?;
        let lat_direction = self.parse_field_char(3)?;
        let longitude: f64 = self.parse_field(4)?;
        let lon_direction = self.parse_field_char(5)?;
        let mode_indicator_str = self.get_field_str(6)?;
        let num_satellites: u8 = self.parse_field(7)?;

        // Copy time string to fixed array
        let mut time_data = [0u8; 16];
        let time_bytes = time_str.as_bytes();
        let time_len = time_bytes.len().min(16) as u8;
        time_data[..time_len as usize].copy_from_slice(&time_bytes[..time_len as usize]);

        // Copy mode indicator to fixed array
        let mut mode_indicator_data = [0u8; 8];
        let mode_bytes = mode_indicator_str.as_bytes();
        let mode_indicator_len = mode_bytes.len().min(8) as u8;
        mode_indicator_data[..mode_indicator_len as usize]
            .copy_from_slice(&mode_bytes[..mode_indicator_len as usize]);

        // Copy diff station ID if present
        let mut diff_station_id_data = [0u8; 8];
        let diff_station_id_len = if let Some(id_str) = self.get_field_str(12) {
            let id_bytes = id_str.as_bytes();
            let len = id_bytes.len().min(8) as u8;
            diff_station_id_data[..len as usize].copy_from_slice(&id_bytes[..len as usize]);
            len
        } else {
            0
        };

        Some(GnsData {
            talker_id: self.talker_id,
            time_data,
            time_len,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            mode_indicator_data,
            mode_indicator_len,
            num_satellites,
            hdop: self.parse_field(8),
            altitude: self.parse_field(9),
            geoid_separation: self.parse_field(10),
            age_of_diff: self.parse_field(11),
            diff_station_id_data,
            diff_station_id_len,
            nav_status: self.parse_field_char(13),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_gns_complete_message() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert_eq!(gns_data.time(), "122310.0");
        assert_eq!(gns_data.latitude, 3723.46587);
        assert_eq!(gns_data.lat_direction, 'N');
        assert_eq!(gns_data.longitude, 12202.26957);
        assert_eq!(gns_data.lon_direction, 'W');
        assert_eq!(gns_data.mode_indicator(), "AAAA");
        assert_eq!(gns_data.num_satellites, 12);
        assert_eq!(gns_data.hdop, Some(0.9));
        assert_eq!(gns_data.altitude, Some(1005.543));
        assert_eq!(gns_data.geoid_separation, Some(6.5));
        assert_eq!(gns_data.age_of_diff, None);
        assert_eq!(gns_data.diff_station_id(), None);
        assert_eq!(gns_data.nav_status, None);
    }

    #[test]
    fn test_gns_with_empty_optional_fields() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,,,,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert_eq!(gns_data.time(), "122310.0");
        assert_eq!(gns_data.latitude, 3723.46587);
        assert_eq!(gns_data.num_satellites, 12);
        assert_eq!(gns_data.hdop, None);
        assert_eq!(gns_data.altitude, None);
        assert_eq!(gns_data.geoid_separation, None);
    }

    #[test]
    fn test_gns_missing_time() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGNS,,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because time is mandatory
        assert!(result.is_none());
    }

    #[test]
    fn test_gns_missing_latitude() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGNS,122310.0,,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gns_missing_mode_indicator() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because mode indicator is mandatory
        assert!(result.is_none());
    }

    #[test]
    fn test_gns_missing_num_satellites() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because num_satellites is mandatory
        assert!(result.is_none());
    }

    #[test]
    fn test_gns_with_differential_data() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,DDDD,12,0.9,1005.543,6.5,2.5,0120*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert_eq!(gns_data.mode_indicator(), "DDDD");
        assert_eq!(gns_data.age_of_diff, Some(2.5));
        assert_eq!(gns_data.diff_station_id(), Some("0120"));
    }

    #[test]
    fn test_gns_with_nav_status() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,,V*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert_eq!(gns_data.nav_status, Some('V'));
    }

    #[test]
    fn test_gns_numeric_precision() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert!((gns_data.latitude - 3723.46587).abs() < 0.00001);
        assert!((gns_data.longitude - 12202.26957).abs() < 0.00001);

        if let Some(hdop) = gns_data.hdop {
            assert!((hdop - 0.9).abs() < 0.01);
        }

        if let Some(alt) = gns_data.altitude {
            assert!((alt - 1005.543).abs() < 0.001);
        }
    }

    #[test]
    fn test_gns_different_talker_id() {
        let parser = NmeaParser::new();
        // GNGNS is multi-GNSS (GPS + GLONASS + others)
        let sentence =
            b"$GNGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert_eq!(gns_data.talker_id, crate::types::TalkerId::GN);
    }

    #[test]
    fn test_gns_gps_talker_id() {
        let parser = NmeaParser::new();
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns();
        assert!(gns.is_some());

        let gns_data = gns.unwrap();
        assert_eq!(gns_data.talker_id, crate::types::TalkerId::GP);
    }

    #[test]
    fn test_gns_multiple_constellations() {
        let parser = NmeaParser::new();

        // Test GPS
        let gp_sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";
        let gp_result = parser.parse_sentence_complete(gp_sentence);
        assert!(gp_result.is_some());
        let gp_msg = gp_result.unwrap();
        let gp_gns = gp_msg.as_gns().unwrap();
        assert_eq!(gp_gns.talker_id, crate::types::TalkerId::GP);

        // Test GLONASS
        let gl_sentence =
            b"$GLGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";
        let gl_result = parser.parse_sentence_complete(gl_sentence);
        assert!(gl_result.is_some());
        let gl_msg = gl_result.unwrap();
        let gl_gns = gl_msg.as_gns().unwrap();
        assert_eq!(gl_gns.talker_id, crate::types::TalkerId::GL);

        // Test Galileo
        let ga_sentence =
            b"$GAGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";
        let ga_result = parser.parse_sentence_complete(ga_sentence);
        assert!(ga_result.is_some());
        let ga_msg = ga_result.unwrap();
        let ga_gns = ga_msg.as_gns().unwrap();
        assert_eq!(ga_gns.talker_id, crate::types::TalkerId::GA);
    }

    #[test]
    fn test_gns_mode_indicators() {
        let parser = NmeaParser::new();

        // Test autonomous mode
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,AAAA,12,0.9,1005.543,6.5,,*79\r\n";
        let result = parser.parse_sentence_complete(sentence);
        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns().unwrap();
        assert_eq!(gns.mode_indicator(), "AAAA");

        // Test differential mode
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,DDDD,12,0.9,1005.543,6.5,,*79\r\n";
        let result = parser.parse_sentence_complete(sentence);
        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns().unwrap();
        assert_eq!(gns.mode_indicator(), "DDDD");

        // Test no fix mode
        let sentence =
            b"$GPGNS,122310.0,3723.46587,N,12202.26957,W,NNNN,12,0.9,1005.543,6.5,,*79\r\n";
        let result = parser.parse_sentence_complete(sentence);
        assert!(result.is_some());
        let msg = result.unwrap();
        let gns = msg.as_gns().unwrap();
        assert_eq!(gns.mode_indicator(), "NNNN");
    }
}
