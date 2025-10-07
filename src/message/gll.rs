//! GLL (Geographic Position - Latitude/Longitude) message implementation
//!
//! The GLL message provides geographic position information including latitude,
//! longitude, UTC time, and data validity status. It's a simpler alternative to
//! GGA for applications that only need position and time.
//!
//! ## Message Format
//!
//! ```text
//! $GPGLL,llll.ll,a,yyyyy.yy,a,hhmmss.ss,A*hh
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPGLL, GNGLL, etc.) |
//! | 1 | Latitude | f64 | Yes | ddmm.mmmmm format |
//! | 2 | N/S Indicator | char | Yes | N = North, S = South |
//! | 3 | Longitude | f64 | Yes | dddmm.mmmmm format |
//! | 4 | E/W Indicator | char | Yes | E = East, W = West |
//! | 5 | UTC Time | String | Yes | hhmmss.ss format |
//! | 6 | Status | char | Yes | A = Valid, V = Invalid |
//!
//! ## Status Values
//!
//! - **A** (Active): Data is valid
//! - **V** (Void): Data is invalid or receiver has no fix
//!
//! ## Example
//!
//! ```text
//! $GPGLL,4916.45,N,12311.12,W,225444,A*1D
//! ```
//!
//! This represents:
//! - Position: 49°16.45'N, 123°11.12'W
//! - Time: 22:54:44 UTC
//! - Status: Active (valid data)

use crate::message::ParsedSentence;
use crate::types::{MessageType, TalkerId};

/// GLL - Geographic Position parameters
#[derive(Debug, Clone)]
pub struct GllData {
    pub talker_id: TalkerId,
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    time_data: [u8; 16],
    time_len: u8,
    pub status: char,
}

impl GllData {
    /// Get time as string slice
    pub fn time(&self) -> &str {
        core::str::from_utf8(&self.time_data[..self.time_len as usize]).unwrap_or("")
    }
}

impl ParsedSentence {
    /// Extract GLL message parameters
    ///
    /// Parses the GLL (Geographic Position) message and returns a structured
    /// `GllData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(GllData)` if the message is a valid GLL message with all mandatory fields
    /// - `None` if:
    ///   - The message is not a GLL message
    ///   - Any mandatory field is missing or invalid
    ///
    /// # Mandatory Fields
    ///
    /// All fields in the GLL message are mandatory:
    /// - Latitude (field 1)
    /// - Latitude direction (field 2)
    /// - Longitude (field 3)
    /// - Longitude direction (field 4)
    /// - Time (field 5)
    /// - Status (field 6)
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let parser = NmeaParser::new();
    /// let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A*1D\r\n";
    ///
    /// let (result, _consumed) = parser.parse_bytes(sentence);
    /// if let Ok(Some(msg)) = result {
    ///     if let Some(gll) = msg.as_gll() {
    ///         assert_eq!(gll.latitude, 4916.45);
    ///         assert_eq!(gll.status, 'A');
    ///     }
    /// }
    /// ```
    pub fn as_gll(&self) -> Option<GllData> {
        if self.message_type != MessageType::GLL {
            return None;
        }

        // Validate mandatory fields
        let latitude: f64 = self.parse_field(1)?;
        let lat_direction = self.parse_field_char(2)?;
        let longitude: f64 = self.parse_field(3)?;
        let lon_direction = self.parse_field_char(4)?;
        let time_str = self.get_field_str(5)?;
        let status = self.parse_field_char(6)?;

        // Copy time to fixed array
        let mut time_data = [0u8; 16];
        let time_bytes = time_str.as_bytes();
        let time_len = time_bytes.len().min(16) as u8;
        time_data[..time_len as usize].copy_from_slice(&time_bytes[..time_len as usize]);

        Some(GllData {
            talker_id: self.talker_id,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            time_data,
            time_len,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_gll_complete_message() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert_eq!(gll_data.latitude, 4916.45);
        assert_eq!(gll_data.lat_direction, 'N');
        assert_eq!(gll_data.longitude, 12311.12);
        assert_eq!(gll_data.lon_direction, 'W');
        assert_eq!(gll_data.time(), "225444");
        assert_eq!(gll_data.status, 'A');
    }

    #[test]
    fn test_gll_void_status() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,V*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert_eq!(gll_data.status, 'V');
    }

    #[test]
    fn test_gll_south_east() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,3723.2475,S,14507.3647,E,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert_eq!(gll_data.lat_direction, 'S');
        assert_eq!(gll_data.lon_direction, 'E');
    }

    #[test]
    fn test_gll_missing_latitude() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,,N,12311.12,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gll_missing_longitude() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gll_missing_time() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gll_missing_status() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gll_invalid_latitude() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,INVALID,N,12311.12,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        // Should return None because a mandatory field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_gll_numeric_precision() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert!((gll_data.latitude - 4916.45).abs() < 0.01);
        assert!((gll_data.longitude - 12311.12).abs() < 0.01);
    }

    #[test]
    fn test_gll_high_precision_coordinates() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.453789,N,12311.125678,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert!((gll_data.latitude - 4916.453789).abs() < 0.000001);
        assert!((gll_data.longitude - 12311.125678).abs() < 0.000001);
    }

    #[test]
    fn test_gll_different_talker_id() {
        let parser = NmeaParser::new();
        // GNGLL is multi-GNSS
        let sentence = b"$GNGLL,4916.45,N,12311.12,W,225444,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());
        
        let gll_data = gll.unwrap();
        assert_eq!(gll_data.talker_id, crate::types::TalkerId::GN);
    }

    #[test]
    fn test_gll_all_constellation_types() {
        let parser = NmeaParser::new();
        
        // Test GPS
        let gp_sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A*1D\r\n";
        let gp_result = parser.parse_sentence_complete(gp_sentence);
        assert!(gp_result.is_some());
        let gp_msg = gp_result.unwrap();
        let gp_gll = gp_msg.as_gll().unwrap();
        assert_eq!(gp_gll.talker_id, crate::types::TalkerId::GP);
        
        // Test BeiDou (GB)
        let gb_sentence = b"$GBGLL,4916.45,N,12311.12,W,225444,A*1D\r\n";
        let gb_result = parser.parse_sentence_complete(gb_sentence);
        assert!(gb_result.is_some());
        let gb_msg = gb_result.unwrap();
        let gb_gll = gb_msg.as_gll().unwrap();
        assert_eq!(gb_gll.talker_id, crate::types::TalkerId::GB);
    }

    #[test]
    fn test_gll_time_with_decimals() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444.50,A*1D\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert_eq!(gll_data.time(), "225444.50");
    }
}

