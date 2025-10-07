//! NMEA sentence parser implementation

use crate::message::{Field, ParsedSentence, MAX_FIELDS};
use crate::types::{MessageType, NmeaMessage, ParseError, TalkerId};

/// Main NMEA parser structure (now stateless)
pub struct NmeaParser {}

impl NmeaParser {
    /// Create a new NMEA parser instance
    pub fn new() -> Self {
        NmeaParser {}
    }

    /// Parse multiple bytes and return a parsed message if found, along with bytes consumed
    /// 
    /// Returns:
    /// - (Ok(Some(message)), bytes_consumed) - Successfully parsed a complete message
    /// - (Ok(None), bytes_consumed) - Partial message, need more data (bytes_consumed will be 0 if no $ found)
    /// - (Err(ParseError), bytes_consumed) - Found complete message but it's invalid
    /// 
    /// The parser handles spurious characters before the '$' start marker by consuming them.
    pub fn parse_bytes(&self, data: &[u8]) -> (Result<Option<NmeaMessage>, ParseError>, usize) {
        // Find the start of a message
        let start_pos = data.iter().position(|&b| b == b'$');
        
        if start_pos.is_none() {
            // No message start found, consume all spurious data
            return (Ok(None), data.len());
        }
        
        let start_pos = start_pos.unwrap();
        
        // Find the end of the message (either \n or \r)
        let end_pos = data[start_pos..]
            .iter()
            .position(|&b| b == b'\n' || b == b'\r');
        
        if end_pos.is_none() {
            // Partial message - consume spurious data before $, but not the partial message
            return (Ok(None), start_pos);
        }
        
        let end_pos = start_pos + end_pos.unwrap();
        let sentence = &data[start_pos..end_pos];
        
        // Parse the complete sentence
        match self.parse_sentence(sentence) {
            Some(msg) => {
                // Successfully parsed - consume up to and including the line ending
                // Need to skip any additional \r or \n characters
                let mut consumed = end_pos + 1;
                while consumed < data.len() && (data[consumed] == b'\r' || data[consumed] == b'\n') {
                    consumed += 1;
                }
                (Ok(Some(msg)), consumed)
            }
            None => {
                // Complete message but invalid (missing mandatory fields)
                // Consume the invalid message
                let mut consumed = end_pos + 1;
                while consumed < data.len() && (data[consumed] == b'\r' || data[consumed] == b'\n') {
                    consumed += 1;
                }
                (Err(ParseError::InvalidMessage), consumed)
            }
        }
    }

    /// Parse a complete NMEA sentence from a buffer
    fn parse_sentence(&self, buffer: &[u8]) -> Option<NmeaMessage> {
        if buffer.len() < 7 || buffer[0] != b'$' {
            return None;
        }

        // Find sentence end (before checksum marker '*')
        let sentence_end = buffer.iter().position(|&b| b == b'*').unwrap_or(buffer.len());
        
        if sentence_end < 7 {
            return None;
        }

        // Extract talker ID and message type
        let (talker_id, message_type) = self.identify_message(&buffer[1..6]);
        if message_type == MessageType::Unknown {
            return None;
        }

        // Parse fields
        let mut fields = [None; MAX_FIELDS];
        let mut field_count = 0;
        let mut field_start = 1; // Skip '$'

        for i in 1..sentence_end {
            if buffer[i] == b',' || i == sentence_end - 1 {
                let field_end = if buffer[i] == b',' { i } else { i + 1 };

                if field_count < MAX_FIELDS {
                    let field_bytes = &buffer[field_start..field_end];
                    if !field_bytes.is_empty() {
                        fields[field_count] = Some(Field::from_bytes(field_bytes));
                    }
                    field_count += 1;
                }
                field_start = i + 1;
            }
        }

        let parsed = ParsedSentence {
            message_type,
            talker_id,
            fields,
            field_count,
        };

        // Convert parsed sentence to typed message
        match message_type {
            MessageType::GGA => parsed.as_gga().map(NmeaMessage::GGA),
            MessageType::RMC => parsed.as_rmc().map(NmeaMessage::RMC),
            MessageType::GSA => parsed.as_gsa().map(NmeaMessage::GSA),
            MessageType::GSV => parsed.as_gsv().map(NmeaMessage::GSV),
            MessageType::GLL => parsed.as_gll().map(NmeaMessage::GLL),
            MessageType::VTG => parsed.as_vtg().map(NmeaMessage::VTG),
            MessageType::Unknown => None,
        }
    }

    /// Identify the talker ID and message type from the sentence header
    fn identify_message(&self, header_bytes: &[u8]) -> (TalkerId, MessageType) {
        if header_bytes.len() < 5 {
            return (TalkerId::Unknown, MessageType::Unknown);
        }

        let talker_id = match &header_bytes[0..2] {
            b"GP" => TalkerId::GP,
            b"GL" => TalkerId::GL,
            b"GA" => TalkerId::GA,
            b"GB" => TalkerId::GB,
            b"GN" => TalkerId::GN,
            b"BD" => TalkerId::BD,
            b"QZ" => TalkerId::QZ,
            _ => TalkerId::Unknown,
        };

        let message_type = match &header_bytes[2..5] {
            b"GGA" => MessageType::GGA,
            b"RMC" => MessageType::RMC,
            b"GSA" => MessageType::GSA,
            b"GSV" => MessageType::GSV,
            b"GLL" => MessageType::GLL,
            b"VTG" => MessageType::VTG,
            _ => MessageType::Unknown,
        };

        (talker_id, message_type)
    }
}

impl Default for NmeaParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl NmeaParser {
    /// Parse a complete sentence with line ending for testing purposes
    /// This is a helper function for migrating old tests
    pub(crate) fn parse_sentence_complete(&self, sentence: &[u8]) -> Option<NmeaMessage> {
        let (result, _consumed) = self.parse_bytes(sentence);
        match result {
            Ok(msg) => msg,
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests based on references/nmea_valid.txt
    #[test]
    fn test_valid_gga_from_reference() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
        
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::GGA);
        
        let gga = msg.as_gga().expect("Should parse as GGA");
        assert_eq!(gga.time(), "123519");
        assert_eq!(gga.latitude, 4807.038);
        assert_eq!(gga.lat_direction, 'N');
        assert_eq!(gga.longitude, 1131.000);
        assert_eq!(gga.lon_direction, 'E');
    }

    #[test]
    fn test_valid_rmc_from_reference() {
        let parser = NmeaParser::new();
        let sentence = b"$GPRMC,235947,A,5540.123,N,01231.456,E,000.0,360.0,130694,011.3,E*62\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
        
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::RMC);
        
        let rmc = msg.as_rmc().expect("Should parse as RMC");
        assert_eq!(rmc.time(), "235947");
        assert_eq!(rmc.status, 'A');
        assert_eq!(rmc.latitude, 5540.123);
        assert_eq!(rmc.lat_direction, 'N');
    }

    #[test]
    fn test_valid_gsa_from_reference() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,09,12,24,25,29,31,,,,,1.8,1.0,1.5*33\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
        
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::GSA);
        
        let gsa = msg.as_gsa().expect("Should parse as GSA");
        assert_eq!(gsa.mode, 'A');
        assert_eq!(gsa.fix_type, 3);
    }

    #[test]
    fn test_valid_gsv_from_reference() {
        let parser = NmeaParser::new();
        let sentence = b"$GPGSV,3,1,12,02,17,315,44,04,77,268,47,05,55,147,45,07,32,195,42*70\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
        
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::GSV);
        
        let gsv = msg.as_gsv().expect("Should parse as GSV");
        assert_eq!(gsv.num_messages, 3);
        assert_eq!(gsv.message_num, 1);
        assert_eq!(gsv.satellites_in_view, 12);
    }

    // Tests based on references/nmea_edge_cases.txt
    #[test]
    fn test_edge_case_gga_empty_fields() {
        let parser = NmeaParser::new();
        // GGA with all empty fields (should fail - mandatory fields missing)
        let sentence = b"$GPGGA,123519,,,,,,0,00,99.99,,,,,,*48\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::InvalidMessage);
        assert_eq!(consumed, sentence.len());
    }

    #[test]
    fn test_edge_case_rmc_zero_coordinates() {
        let parser = NmeaParser::new();
        // RMC with zero coordinates (valid but unusual)
        let sentence = b"$GPRMC,000000,A,0000.000,N,00000.000,E,000.0,000.0,000000,000.0,W*7C\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
        
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::RMC);
        
        let rmc = msg.as_rmc().expect("Should parse as RMC");
        assert_eq!(rmc.latitude, 0.0);
        assert_eq!(rmc.longitude, 0.0);
    }

    #[test]
    fn test_edge_case_ais_message_with_exclamation() {
        let parser = NmeaParser::new();
        // AIS message starts with '!' instead of '$' - should not parse
        let sentence = b"!AIVDM,1,1,,B,15N:;R0P00PD;88MD5MTDwwP0<0L,0*5C\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        // Should consume spurious data and return None
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert_eq!(consumed, sentence.len());
    }

    #[test]
    fn test_edge_case_concatenated_messages() {
        let parser = NmeaParser::new();
        // Two messages concatenated - parser will parse up to the first checksum
        // and treat the rest as part of the same line
        let data = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47$GPRMC,235947,A,5540.123,N,01231.456,E,000.0,360.0,130694,011.3,E*62\r\n";
        
        let (result, consumed) = parser.parse_bytes(data);
        // The parser finds the first * and parses up to that point as the first message
        // This successfully parses the GGA message
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert_eq!(msg.message_type(), MessageType::GGA);
        assert_eq!(consumed, data.len());
    }

    #[test]
    fn test_edge_case_unsupported_message_types() {
        let parser = NmeaParser::new();
        
        // GPTXT - text message (not supported)
        let txt_sentence = b"$GPTXT,01,01,02,Software Version 7.03.00 (12345)*6E\r\n";
        let (result, consumed) = parser.parse_bytes(txt_sentence);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::InvalidMessage);
        assert_eq!(consumed, txt_sentence.len());
        
        // GPXTE - cross-track error (not supported)
        let xte_sentence = b"$GPXTE,A,A,0.67,L,N*6F\r\n";
        let (result2, consumed2) = parser.parse_bytes(xte_sentence);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), ParseError::InvalidMessage);
        assert_eq!(consumed2, xte_sentence.len());
    }

    // Tests based on references/nmea_invalid.txt
    #[test]
    fn test_invalid_wrong_checksum() {
        let parser = NmeaParser::new();
        // Valid GGA structure but wrong checksum (*00 instead of *47)
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00\r\n";
        
        // Parser doesn't validate checksum, so this will parse successfully
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
    }

    #[test]
    fn test_invalid_rmc_void_status() {
        let parser = NmeaParser::new();
        // RMC with status 'V' (void/invalid) - still valid structure
        let sentence = b"$GPRMC,235947,V,5540.123,N,01231.456,E,000.0,360.0,130694,011.3,E*00\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
        
        let msg = msg.unwrap();
        let rmc = msg.as_rmc().expect("Should parse as RMC");
        assert_eq!(rmc.status, 'V'); // Status V means data is invalid but structure is valid
    }

    #[test]
    fn test_invalid_missing_checksum() {
        let parser = NmeaParser::new();
        // GSA without checksum marker
        let sentence = b"$GPGSA,A,3,04,05,09,12,24,25,29,31,,,,,1.8,1.0,1.5\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        // Should still parse (checksum is optional in parsing)
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.is_some());
        assert_eq!(consumed, sentence.len());
    }

    #[test]
    fn test_invalid_missing_dollar_sign() {
        let parser = NmeaParser::new();
        // GSV without starting '$'
        let sentence = b"GPGSV,3,1,12,02,17,315,44,04,77,268,47,05,55,147,45,07,32,195,42*70\r\n";
        
        let (result, consumed) = parser.parse_bytes(sentence);
        // Should consume as spurious data and return None
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert_eq!(consumed, sentence.len());
    }

    #[test]
    fn test_multiple_valid_messages_in_sequence() {
        let parser = NmeaParser::new();
        
        // Parse first message
        let gga = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        let (result1, consumed1) = parser.parse_bytes(gga);
        assert!(result1.is_ok());
        assert!(result1.unwrap().is_some());
        assert_eq!(consumed1, gga.len());
        
        // Parse second message
        let rmc = b"$GPRMC,235947,A,5540.123,N,01231.456,E,000.0,360.0,130694,011.3,E*62\r\n";
        let (result2, consumed2) = parser.parse_bytes(rmc);
        assert!(result2.is_ok());
        assert!(result2.unwrap().is_some());
        assert_eq!(consumed2, rmc.len());
        
        // Parse third message
        let gsa = b"$GPGSA,A,3,04,05,09,12,24,25,29,31,,,,,1.8,1.0,1.5*33\r\n";
        let (result3, consumed3) = parser.parse_bytes(gsa);
        assert!(result3.is_ok());
        assert!(result3.unwrap().is_some());
        assert_eq!(consumed3, gsa.len());
    }
}
