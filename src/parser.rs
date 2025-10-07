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
        if buffer.len() < 7 {
            return None;
        }

        // Verify it starts with $
        if buffer[0] != b'$' {
            return None;
        }

        // Verify checksum if present
        let sentence_end = buffer.iter().position(|&b| b == b'*').unwrap_or(buffer.len());

        if sentence_end < 7 {
            return None;
        }

        // Extract talker ID and message type
        let (talker_id, message_type) = self.identify_message(&buffer[1..6]);

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

        // Extract talker ID (first 2 characters)
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

        // Extract message type (last 3 characters)
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
