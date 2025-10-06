#![no_std]

/// Maximum buffer size for NMEA sentence
const MAX_SENTENCE_LENGTH: usize = 82;

/// Maximum number of fields in an NMEA sentence
const MAX_FIELDS: usize = 20;

/// Represents the different NMEA message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    GGA,  // Global Positioning System Fix Data
    RMC,  // Recommended Minimum Navigation Information
    GSA,  // GPS DOP and active satellites
    GSV,  // GPS Satellites in view
    GLL,  // Geographic Position - Latitude/Longitude
    VTG,  // Track Made Good and Ground Speed
    Unknown,
}

/// Parsed NMEA message data
#[derive(Debug, Clone)]
pub struct NmeaMessage {
    pub message_type: MessageType,
    pub fields: [Option<Field>; MAX_FIELDS],
    pub field_count: usize,
    pub timestamp: u64,
}

/// Represents a field value in an NMEA message
#[derive(Debug, Clone, Copy)]
pub struct Field {
    data: [u8; 32],
    len: usize,
}

impl Field {
    fn new() -> Self {
        Field {
            data: [0; 32],
            len: 0,
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut field = Field::new();
        let copy_len = bytes.len().min(32);
        field.data[..copy_len].copy_from_slice(&bytes[..copy_len]);
        field.len = copy_len;
        field
    }

    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.data[..self.len]).ok()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

/// Stored message with timestamp
#[derive(Debug, Clone)]
struct StoredMessage {
    message: Option<NmeaMessage>,
}

impl StoredMessage {
    fn new() -> Self {
        StoredMessage { message: None }
    }
}

/// Main NMEA parser structure
pub struct NmeaParser {
    buffer: [u8; MAX_SENTENCE_LENGTH],
    buffer_pos: usize,
    timestamp_counter: u64,
    stored_gga: StoredMessage,
    stored_rmc: StoredMessage,
    stored_gsa: StoredMessage,
    stored_gsv: StoredMessage,
    stored_gll: StoredMessage,
    stored_vtg: StoredMessage,
}

impl NmeaParser {
    /// Create a new NMEA parser instance
    pub fn new() -> Self {
        NmeaParser {
            buffer: [0; MAX_SENTENCE_LENGTH],
            buffer_pos: 0,
            timestamp_counter: 0,
            stored_gga: StoredMessage::new(),
            stored_rmc: StoredMessage::new(),
            stored_gsa: StoredMessage::new(),
            stored_gsv: StoredMessage::new(),
            stored_gll: StoredMessage::new(),
            stored_vtg: StoredMessage::new(),
        }
    }

    /// Parse a character stream and return a complete message if found
    pub fn parse_char(&mut self, c: u8) -> Option<NmeaMessage> {
        // Handle sentence start
        if c == b'$' {
            self.buffer_pos = 0;
            self.buffer[self.buffer_pos] = c;
            self.buffer_pos += 1;
            return None;
        }

        // Handle sentence end
        if c == b'\n' || c == b'\r' {
            if self.buffer_pos > 0 && self.buffer[0] == b'$' {
                let result = self.parse_sentence();
                self.buffer_pos = 0;
                return result;
            }
            self.buffer_pos = 0;
            return None;
        }

        // Add character to buffer
        if self.buffer_pos < MAX_SENTENCE_LENGTH {
            self.buffer[self.buffer_pos] = c;
            self.buffer_pos += 1;
        }

        None
    }

    /// Parse a complete NMEA sentence from the buffer
    fn parse_sentence(&mut self) -> Option<NmeaMessage> {
        if self.buffer_pos < 7 {
            return None;
        }

        // Verify checksum if present
        let sentence_end = if let Some(pos) = self.find_byte(b'*') {
            pos
        } else {
            self.buffer_pos
        };

        if sentence_end < 7 {
            return None;
        }

        // Extract message type
        let message_type = self.identify_message_type(&self.buffer[3..6]);

        // Parse fields
        let mut fields = [None; MAX_FIELDS];
        let mut field_count = 0;
        let mut field_start = 1; // Skip '$'

        for i in 1..sentence_end {
            if self.buffer[i] == b',' || i == sentence_end - 1 {
                let field_end = if self.buffer[i] == b',' { i } else { i + 1 };
                
                if field_count < MAX_FIELDS {
                    let field_bytes = &self.buffer[field_start..field_end];
                    if field_bytes.len() > 0 {
                        fields[field_count] = Some(Field::from_bytes(field_bytes));
                    }
                    field_count += 1;
                }
                field_start = i + 1;
            }
        }

        self.timestamp_counter += 1;
        let message = NmeaMessage {
            message_type,
            fields,
            field_count,
            timestamp: self.timestamp_counter,
        };

        // Store the message
        self.store_message(&message);

        Some(message)
    }

    /// Identify the message type from the talker ID and message code
    fn identify_message_type(&self, type_bytes: &[u8]) -> MessageType {
        if type_bytes.len() < 3 {
            return MessageType::Unknown;
        }

        // Check last 3 characters (message type)
        match &type_bytes[type_bytes.len() - 3..] {
            b"GGA" => MessageType::GGA,
            b"RMC" => MessageType::RMC,
            b"GSA" => MessageType::GSA,
            b"GSV" => MessageType::GSV,
            b"GLL" => MessageType::GLL,
            b"VTG" => MessageType::VTG,
            _ => MessageType::Unknown,
        }
    }

    /// Find a byte in the buffer
    fn find_byte(&self, byte: u8) -> Option<usize> {
        for i in 0..self.buffer_pos {
            if self.buffer[i] == byte {
                return Some(i);
            }
        }
        None
    }

    /// Store a message based on its type
    fn store_message(&mut self, message: &NmeaMessage) {
        let stored = match message.message_type {
            MessageType::GGA => &mut self.stored_gga,
            MessageType::RMC => &mut self.stored_rmc,
            MessageType::GSA => &mut self.stored_gsa,
            MessageType::GSV => &mut self.stored_gsv,
            MessageType::GLL => &mut self.stored_gll,
            MessageType::VTG => &mut self.stored_vtg,
            MessageType::Unknown => return,
        };
        stored.message = Some(message.clone());
    }

    /// Get the last message of a specific type
    pub fn get_last_message(&self, msg_type: MessageType) -> Option<&NmeaMessage> {
        let stored = match msg_type {
            MessageType::GGA => &self.stored_gga,
            MessageType::RMC => &self.stored_rmc,
            MessageType::GSA => &self.stored_gsa,
            MessageType::GSV => &self.stored_gsv,
            MessageType::GLL => &self.stored_gll,
            MessageType::VTG => &self.stored_vtg,
            MessageType::Unknown => return None,
        };
        stored.message.as_ref()
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.buffer_pos = 0;
        self.timestamp_counter = 0;
    }
}

impl Default for NmeaParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_initialization() {
        let parser = NmeaParser::new();
        assert_eq!(parser.buffer_pos, 0);
        assert_eq!(parser.timestamp_counter, 0);
    }

    #[test]
    fn test_parse_gga_message() {
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
        assert_eq!(msg.message_type, MessageType::GGA);
        assert!(msg.field_count > 0);
        assert_eq!(msg.timestamp, 1);
    }

    #[test]
    fn test_parse_rmc_message() {
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
        assert_eq!(msg.message_type, MessageType::RMC);
        assert!(msg.field_count > 0);
    }

    #[test]
    fn test_parse_gsa_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::GSA);
    }

    #[test]
    fn test_parse_gsv_message() {
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
        assert_eq!(msg.message_type, MessageType::GSV);
    }

    #[test]
    fn test_parse_gll_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A,*1D\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::GLL);
    }

    #[test]
    fn test_parse_vtg_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::VTG);
    }

    #[test]
    fn test_get_last_message() {
        let mut parser = NmeaParser::new();
        
        // Parse a GGA message
        let gga_sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        for &c in gga_sentence.iter() {
            parser.parse_char(c);
        }

        // Parse an RMC message
        let rmc_sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";
        for &c in rmc_sentence.iter() {
            parser.parse_char(c);
        }

        // Verify we can retrieve both messages
        let gga_msg = parser.get_last_message(MessageType::GGA);
        assert!(gga_msg.is_some());
        assert_eq!(gga_msg.unwrap().message_type, MessageType::GGA);
        assert_eq!(gga_msg.unwrap().timestamp, 1);

        let rmc_msg = parser.get_last_message(MessageType::RMC);
        assert!(rmc_msg.is_some());
        assert_eq!(rmc_msg.unwrap().message_type, MessageType::RMC);
        assert_eq!(rmc_msg.unwrap().timestamp, 2);

        // Verify we get None for message types we haven't parsed
        let gsa_msg = parser.get_last_message(MessageType::GSA);
        assert!(gsa_msg.is_none());
    }

    #[test]
    fn test_multiple_messages_stream() {
        let mut parser = NmeaParser::new();
        let stream = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n\
                       $GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n\
                       $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let mut message_count = 0;
        for &c in stream.iter() {
            if parser.parse_char(c).is_some() {
                message_count += 1;
            }
        }

        assert_eq!(message_count, 3);
    }

    #[test]
    fn test_field_extraction() {
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
        
        // Check first field (sentence ID)
        assert!(msg.fields[0].is_some());
        let field0 = msg.fields[0].as_ref().unwrap();
        assert_eq!(field0.as_str(), Some("GPGGA"));
        
        // Check time field
        assert!(msg.fields[1].is_some());
        let field1 = msg.fields[1].as_ref().unwrap();
        assert_eq!(field1.as_str(), Some("123519"));
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        for &c in sentence.iter() {
            parser.parse_char(c);
        }

        assert_eq!(parser.timestamp_counter, 1);
        
        parser.reset();
        assert_eq!(parser.timestamp_counter, 0);
        assert_eq!(parser.buffer_pos, 0);
    }

    #[test]
    fn test_invalid_sentence() {
        let mut parser = NmeaParser::new();
        let invalid = b"INVALID DATA\r\n";

        let mut result = None;
        for &c in invalid.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_none());
    }

    #[test]
    fn test_partial_sentence() {
        let mut parser = NmeaParser::new();
        let partial = b"$GPGGA,123519,4807.038,N";

        let mut result = None;
        for &c in partial.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should not complete without \r\n
        assert!(result.is_none());
    }

    #[test]
    fn test_message_overwrite() {
        let mut parser = NmeaParser::new();
        
        // Parse first GGA message
        let gga1 = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        for &c in gga1.iter() {
            parser.parse_char(c);
        }

        let first_msg = parser.get_last_message(MessageType::GGA);
        assert_eq!(first_msg.unwrap().timestamp, 1);

        // Parse second GGA message
        let gga2 = b"$GPGGA,133519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        for &c in gga2.iter() {
            parser.parse_char(c);
        }

        let second_msg = parser.get_last_message(MessageType::GGA);
        assert_eq!(second_msg.unwrap().timestamp, 2);
    }
}
