//! NMEA sentence parser implementation

use crate::message::{Field, ParsedSentence, MAX_FIELDS};
use crate::types::{MessageType, NmeaMessage, TalkerId};

/// Maximum buffer size for NMEA sentence
const MAX_SENTENCE_LENGTH: usize = 82;

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

        // Extract talker ID and message type
        let (talker_id, message_type) = self.identify_message(&self.buffer[1..6]);

        // Parse fields
        let mut fields = [None; MAX_FIELDS];
        let mut field_count = 0;
        let mut field_start = 1; // Skip '$'

        for i in 1..sentence_end {
            if self.buffer[i] == b',' || i == sentence_end - 1 {
                let field_end = if self.buffer[i] == b',' { i } else { i + 1 };

                if field_count < MAX_FIELDS {
                    let field_bytes = &self.buffer[field_start..field_end];
                    if !field_bytes.is_empty() {
                        fields[field_count] = Some(Field::from_bytes(field_bytes));
                    }
                    field_count += 1;
                }
                field_start = i + 1;
            }
        }

        self.timestamp_counter += 1;
        let parsed = ParsedSentence {
            message_type,
            talker_id,
            fields,
            field_count,
            timestamp: self.timestamp_counter,
        };

        // Convert parsed sentence to typed message
        let message = match message_type {
            MessageType::GGA => parsed.as_gga().map(NmeaMessage::GGA),
            MessageType::RMC => parsed.as_rmc().map(NmeaMessage::RMC),
            MessageType::GSA => parsed.as_gsa().map(NmeaMessage::GSA),
            MessageType::GSV => parsed.as_gsv().map(NmeaMessage::GSV),
            MessageType::GLL => parsed.as_gll().map(NmeaMessage::GLL),
            MessageType::VTG => parsed.as_vtg().map(NmeaMessage::VTG),
            MessageType::Unknown => None,
        };

        // Store the message
        if let Some(ref msg) = message {
            self.store_message(msg);
        }

        message
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

    /// Find a byte in the buffer
    fn find_byte(&self, byte: u8) -> Option<usize> {
        (0..self.buffer_pos).find(|&i| self.buffer[i] == byte)
    }

    /// Store a message based on its type
    fn store_message(&mut self, message: &NmeaMessage) {
        let stored = match message {
            NmeaMessage::GGA(_) => &mut self.stored_gga,
            NmeaMessage::RMC(_) => &mut self.stored_rmc,
            NmeaMessage::GSA(_) => &mut self.stored_gsa,
            NmeaMessage::GSV(_) => &mut self.stored_gsv,
            NmeaMessage::GLL(_) => &mut self.stored_gll,
            NmeaMessage::VTG(_) => &mut self.stored_vtg,
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

    #[cfg(test)]
    pub(crate) fn buffer_pos(&self) -> usize {
        self.buffer_pos
    }

    #[cfg(test)]
    pub(crate) fn timestamp_counter(&self) -> u64 {
        self.timestamp_counter
    }
}

impl Default for NmeaParser {
    fn default() -> Self {
        Self::new()
    }
}
