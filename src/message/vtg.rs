//! VTG (Track Made Good and Ground Speed) message implementation
//!
//! The VTG message provides velocity and track information including:
//! - True track angle (relative to true north)
//! - Magnetic track angle (relative to magnetic north)
//! - Ground speed in knots
//! - Ground speed in kilometers per hour
//!
//! ## Message Format
//!
//! ```text
//! $GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48
//! ```
//!
//! ## Fields
//!
//! | Index | Field | Type | Mandatory | Description |
//! |-------|-------|------|-----------|-------------|
//! | 0 | Sentence ID | String | Yes | Message type (GPVTG, GNVTG, etc.) |
//! | 1 | Track True | f32 | No | Track angle in degrees (true north) |
//! | 2 | True Indicator | char | No | T = True |
//! | 3 | Track Magnetic | f32 | No | Track angle in degrees (magnetic north) |
//! | 4 | Magnetic Indicator | char | No | M = Magnetic |
//! | 5 | Speed Knots | f32 | No | Ground speed in knots |
//! | 6 | Knots Indicator | char | No | N = Knots |
//! | 7 | Speed KPH | f32 | No | Ground speed in kilometers per hour |
//! | 8 | KPH Indicator | char | No | K = Kilometers per hour |
//!
//! ## Note on Optional Fields
//!
//! All fields in VTG are technically optional. The message may contain empty
//! fields if the receiver doesn't have a fix or if certain data is not available.
//!
//! ## Track Angles
//!
//! - **True Track**: Angle relative to true north (0-359.9°)
//! - **Magnetic Track**: Angle relative to magnetic north (0-359.9°)
//! - The difference between true and magnetic track is the magnetic variation
//!
//! ## Speed Conversion
//!
//! - 1 knot = 1.852 km/h
//! - Speed values are ground speed, not airspeed
//!
//! ## Example
//!
//! ```text
//! $GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48
//! ```
//!
//! This represents:
//! - True track: 54.7° (relative to true north)
//! - Magnetic track: 34.4° (relative to magnetic north)
//! - Speed: 5.5 knots = 10.2 km/h
//! - Magnetic variation: ~20° East (54.7 - 34.4)

use crate::message::ParsedSentence;
use crate::types::{MessageType, TalkerId};

/// VTG - Track Made Good and Ground Speed parameters
#[derive(Debug, Clone)]
pub struct VtgData {
    pub talker_id: TalkerId,
    pub track_true: Option<f32>,
    pub track_true_indicator: Option<char>,
    pub track_magnetic: Option<f32>,
    pub track_magnetic_indicator: Option<char>,
    pub speed_knots: Option<f32>,
    pub speed_knots_indicator: Option<char>,
    pub speed_kph: Option<f32>,
    pub speed_kph_indicator: Option<char>,
}

impl ParsedSentence {
    /// Extract VTG message parameters
    ///
    /// Parses the VTG (Track Made Good and Ground Speed) message and returns
    /// a structured `VtgData` object containing all parsed fields.
    ///
    /// # Returns
    ///
    /// - `Some(VtgData)` if the message is a valid VTG message
    /// - `None` if the message is not a VTG message
    ///
    /// # Mandatory Fields
    ///
    /// The VTG message has no strictly mandatory fields. All fields are optional
    /// and will be `None` if not present or invalid.
    ///
    /// # Optional Fields
    ///
    /// All fields (1-8) are optional:
    /// - Track true and indicator
    /// - Track magnetic and indicator
    /// - Speed in knots and indicator
    /// - Speed in km/h and indicator
    ///
    /// # Example
    ///
    /// ```
    /// use rustedbytes_nmea::{NmeaParser, MessageType};
    ///
    /// let parser = NmeaParser::new();
    /// let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";
    ///
    /// let (result, _consumed) = parser.parse_bytes(sentence);
    /// if let Ok(Some(msg)) = result {
    ///     if let Some(vtg) = msg.as_vtg() {
    ///         assert_eq!(vtg.track_true, Some(54.7));
    ///         assert_eq!(vtg.speed_knots, Some(5.5));
    ///     }
    /// }
    /// ```
    pub fn as_vtg(&self) -> Option<VtgData> {
        if self.message_type != MessageType::VTG {
            return None;
        }

        Some(VtgData {
            talker_id: self.talker_id,
            track_true: self.parse_field_f32(1),
            track_true_indicator: self.parse_field_char(2),
            track_magnetic: self.parse_field_f32(3),
            track_magnetic_indicator: self.parse_field_char(4),
            speed_knots: self.parse_field_f32(5),
            speed_knots_indicator: self.parse_field_char(6),
            speed_kph: self.parse_field_f32(7),
            speed_kph_indicator: self.parse_field_char(8),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::NmeaParser;

    #[test]
    fn test_vtg_complete_message() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.track_true, Some(54.7));
        assert_eq!(vtg_data.track_true_indicator, Some('T'));
        assert_eq!(vtg_data.track_magnetic, Some(34.4));
        assert_eq!(vtg_data.track_magnetic_indicator, Some('M'));
        assert_eq!(vtg_data.speed_knots, Some(5.5));
        assert_eq!(vtg_data.speed_knots_indicator, Some('N'));
        assert_eq!(vtg_data.speed_kph, Some(10.2));
        assert_eq!(vtg_data.speed_kph_indicator, Some('K'));
    }

    #[test]
    fn test_vtg_with_empty_fields() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,,T,,M,,N,,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.track_true, None);
        assert_eq!(vtg_data.track_magnetic, None);
        assert_eq!(vtg_data.speed_knots, None);
        assert_eq!(vtg_data.speed_kph, None);
    }

    #[test]
    fn test_vtg_zero_speed() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,0.0,T,0.0,M,0.0,N,0.0,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.speed_knots, Some(0.0));
        assert_eq!(vtg_data.speed_kph, Some(0.0));
    }

    #[test]
    fn test_vtg_high_speed() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,270.5,T,250.3,M,125.8,N,233.0,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert!((vtg_data.speed_knots.unwrap() - 125.8).abs() < 0.1);
        assert!((vtg_data.speed_kph.unwrap() - 233.0).abs() < 0.1);
    }

    #[test]
    fn test_vtg_only_true_track() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,054.7,T,,M,,N,,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.track_true, Some(54.7));
        assert_eq!(vtg_data.track_magnetic, None);
    }

    #[test]
    fn test_vtg_only_knots() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,,T,,M,5.5,N,,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.speed_knots, Some(5.5));
        assert_eq!(vtg_data.speed_kph, None);
    }

    #[test]
    fn test_vtg_only_kph() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,,T,,M,,N,10.2,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.speed_knots, None);
        assert_eq!(vtg_data.speed_kph, Some(10.2));
    }

    #[test]
    fn test_vtg_track_angle_ranges() {
        let parser = NmeaParser::new();

        // Test 0 degrees
        let sentence = b"$GPVTG,0.0,T,0.0,M,5.5,N,10.2,K*48\r\n";
        let result = parser.parse_sentence_complete(sentence);
        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg().unwrap();
        assert_eq!(vtg.track_true, Some(0.0));

        // Test 359.9 degrees
        let sentence = b"$GPVTG,359.9,T,359.9,M,5.5,N,10.2,K*48\r\n";
        let result = parser.parse_sentence_complete(sentence);
        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg().unwrap();
        assert!((vtg.track_true.unwrap() - 359.9).abs() < 0.1);
    }

    #[test]
    fn test_vtg_speed_conversion_accuracy() {
        let parser = NmeaParser::new();
        // 1 knot = 1.852 km/h
        // 5.5 knots = 10.186 km/h (rounded to 10.2)
        let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        let knots = vtg_data.speed_knots.unwrap();
        let kph = vtg_data.speed_kph.unwrap();

        // Verify the conversion is approximately correct
        let expected_kph = knots * 1.852;
        assert!((kph - expected_kph).abs() < 0.2);
    }

    #[test]
    fn test_vtg_numeric_precision() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert!((vtg_data.track_true.unwrap() - 54.7).abs() < 0.1);
        assert!((vtg_data.track_magnetic.unwrap() - 34.4).abs() < 0.1);
        assert!((vtg_data.speed_knots.unwrap() - 5.5).abs() < 0.1);
        assert!((vtg_data.speed_kph.unwrap() - 10.2).abs() < 0.1);
    }

    #[test]
    fn test_vtg_different_talker_id() {
        let parser = NmeaParser::new();
        // GNVTG is multi-GNSS
        let sentence = b"$GNVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());
        
        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.talker_id, crate::types::TalkerId::GN);
    }

    #[test]
    fn test_vtg_mixed_constellation_data() {
        let parser = NmeaParser::new();
        
        // Test GPS
        let gp_sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";
        let gp_result = parser.parse_sentence_complete(gp_sentence);
        assert!(gp_result.is_some());
        let gp_msg = gp_result.unwrap();
        let gp_vtg = gp_msg.as_vtg().unwrap();
        assert_eq!(gp_vtg.talker_id, crate::types::TalkerId::GP);
        assert_eq!(gp_vtg.track_true, Some(54.7));
        
        // Test GLONASS
        let gl_sentence = b"$GLVTG,154.7,T,134.4,M,015.5,N,028.7,K*48\r\n";
        let gl_result = parser.parse_sentence_complete(gl_sentence);
        assert!(gl_result.is_some());
        let gl_msg = gl_result.unwrap();
        let gl_vtg = gl_msg.as_vtg().unwrap();
        assert_eq!(gl_vtg.talker_id, crate::types::TalkerId::GL);
        assert_eq!(gl_vtg.track_true, Some(154.7));
    }

    #[test]
    fn test_vtg_easterly_heading() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,090.0,T,085.0,M,10.0,N,18.5,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.track_true, Some(90.0));
    }

    #[test]
    fn test_vtg_westerly_heading() {
        let parser = NmeaParser::new();
        let sentence = b"$GPVTG,270.0,T,265.0,M,10.0,N,18.5,K*48\r\n";

        let result = parser.parse_sentence_complete(sentence);

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.track_true, Some(270.0));
    }
}
