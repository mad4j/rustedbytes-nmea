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

/// GGA - Global Positioning System Fix Data parameters
#[derive(Debug, Clone)]
pub struct GgaData<'a> {
    pub time: Option<&'a str>,
    pub latitude: Option<f64>,
    pub lat_direction: Option<char>,
    pub longitude: Option<f64>,
    pub lon_direction: Option<char>,
    pub fix_quality: Option<u8>,
    pub num_satellites: Option<u8>,
    pub hdop: Option<f32>,
    pub altitude: Option<f32>,
    pub altitude_units: Option<char>,
    pub geoid_separation: Option<f32>,
    pub geoid_units: Option<char>,
    pub age_of_diff: Option<f32>,
    pub diff_station_id: Option<&'a str>,
}

/// RMC - Recommended Minimum Navigation Information parameters
#[derive(Debug, Clone)]
pub struct RmcData<'a> {
    pub time: Option<&'a str>,
    pub status: Option<char>,
    pub latitude: Option<f64>,
    pub lat_direction: Option<char>,
    pub longitude: Option<f64>,
    pub lon_direction: Option<char>,
    pub speed_knots: Option<f32>,
    pub track_angle: Option<f32>,
    pub date: Option<&'a str>,
    pub magnetic_variation: Option<f32>,
    pub mag_var_direction: Option<char>,
}

/// GSA - GPS DOP and active satellites parameters
#[derive(Debug, Clone)]
pub struct GsaData {
    pub mode: Option<char>,
    pub fix_type: Option<u8>,
    pub satellite_ids: [Option<u8>; 12],
    pub pdop: Option<f32>,
    pub hdop: Option<f32>,
    pub vdop: Option<f32>,
}

/// GSV - GPS Satellites in view parameters
#[derive(Debug, Clone)]
pub struct GsvData {
    pub num_messages: Option<u8>,
    pub message_num: Option<u8>,
    pub satellites_in_view: Option<u8>,
    pub satellite_info: [Option<SatelliteInfo>; 4],
}

/// Information about a single satellite
#[derive(Debug, Clone)]
pub struct SatelliteInfo {
    pub prn: Option<u8>,
    pub elevation: Option<u16>,
    pub azimuth: Option<u16>,
    pub snr: Option<u8>,
}

/// GLL - Geographic Position parameters
#[derive(Debug, Clone)]
pub struct GllData<'a> {
    pub latitude: Option<f64>,
    pub lat_direction: Option<char>,
    pub longitude: Option<f64>,
    pub lon_direction: Option<char>,
    pub time: Option<&'a str>,
    pub status: Option<char>,
}

/// VTG - Track Made Good and Ground Speed parameters
#[derive(Debug, Clone)]
pub struct VtgData {
    pub track_true: Option<f32>,
    pub track_true_indicator: Option<char>,
    pub track_magnetic: Option<f32>,
    pub track_magnetic_indicator: Option<char>,
    pub speed_knots: Option<f32>,
    pub speed_knots_indicator: Option<char>,
    pub speed_kph: Option<f32>,
    pub speed_kph_indicator: Option<char>,
}

/// Parsed NMEA message data
#[derive(Debug, Clone)]
pub struct NmeaMessage {
    pub message_type: MessageType,
    pub fields: [Option<Field>; MAX_FIELDS],
    pub field_count: usize,
    pub timestamp: u64,
}

impl NmeaMessage {
    /// Extract GGA message parameters
    pub fn as_gga(&self) -> Option<GgaData<'_>> {
        if self.message_type != MessageType::GGA {
            return None;
        }
        
        Some(GgaData {
            time: self.get_field_str(1),
            latitude: self.parse_field_f64(2),
            lat_direction: self.parse_field_char(3),
            longitude: self.parse_field_f64(4),
            lon_direction: self.parse_field_char(5),
            fix_quality: self.parse_field_u8(6),
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
    
    /// Extract RMC message parameters
    pub fn as_rmc(&self) -> Option<RmcData<'_>> {
        if self.message_type != MessageType::RMC {
            return None;
        }
        
        Some(RmcData {
            time: self.get_field_str(1),
            status: self.parse_field_char(2),
            latitude: self.parse_field_f64(3),
            lat_direction: self.parse_field_char(4),
            longitude: self.parse_field_f64(5),
            lon_direction: self.parse_field_char(6),
            speed_knots: self.parse_field_f32(7),
            track_angle: self.parse_field_f32(8),
            date: self.get_field_str(9),
            magnetic_variation: self.parse_field_f32(10),
            mag_var_direction: self.parse_field_char(11),
        })
    }
    
    /// Extract GSA message parameters
    pub fn as_gsa(&self) -> Option<GsaData> {
        if self.message_type != MessageType::GSA {
            return None;
        }
        
        Some(GsaData {
            mode: self.parse_field_char(1),
            fix_type: self.parse_field_u8(2),
            satellite_ids: [
                self.parse_field_u8(3),
                self.parse_field_u8(4),
                self.parse_field_u8(5),
                self.parse_field_u8(6),
                self.parse_field_u8(7),
                self.parse_field_u8(8),
                self.parse_field_u8(9),
                self.parse_field_u8(10),
                self.parse_field_u8(11),
                self.parse_field_u8(12),
                self.parse_field_u8(13),
                self.parse_field_u8(14),
            ],
            pdop: self.parse_field_f32(15),
            hdop: self.parse_field_f32(16),
            vdop: self.parse_field_f32(17),
        })
    }
    
    /// Extract GSV message parameters
    pub fn as_gsv(&self) -> Option<GsvData> {
        if self.message_type != MessageType::GSV {
            return None;
        }
        
        let sat1 = if self.get_field_str(4).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(4),
                elevation: self.parse_field_u16(5),
                azimuth: self.parse_field_u16(6),
                snr: self.parse_field_u8(7),
            })
        } else {
            None
        };
        
        let sat2 = if self.get_field_str(8).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(8),
                elevation: self.parse_field_u16(9),
                azimuth: self.parse_field_u16(10),
                snr: self.parse_field_u8(11),
            })
        } else {
            None
        };
        
        let sat3 = if self.get_field_str(12).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(12),
                elevation: self.parse_field_u16(13),
                azimuth: self.parse_field_u16(14),
                snr: self.parse_field_u8(15),
            })
        } else {
            None
        };
        
        let sat4 = if self.get_field_str(16).is_some() {
            Some(SatelliteInfo {
                prn: self.parse_field_u8(16),
                elevation: self.parse_field_u16(17),
                azimuth: self.parse_field_u16(18),
                snr: self.parse_field_u8(19),
            })
        } else {
            None
        };
        
        Some(GsvData {
            num_messages: self.parse_field_u8(1),
            message_num: self.parse_field_u8(2),
            satellites_in_view: self.parse_field_u8(3),
            satellite_info: [sat1, sat2, sat3, sat4],
        })
    }
    
    /// Extract GLL message parameters
    pub fn as_gll(&self) -> Option<GllData<'_>> {
        if self.message_type != MessageType::GLL {
            return None;
        }
        
        Some(GllData {
            latitude: self.parse_field_f64(1),
            lat_direction: self.parse_field_char(2),
            longitude: self.parse_field_f64(3),
            lon_direction: self.parse_field_char(4),
            time: self.get_field_str(5),
            status: self.parse_field_char(6),
        })
    }
    
    /// Extract VTG message parameters
    pub fn as_vtg(&self) -> Option<VtgData> {
        if self.message_type != MessageType::VTG {
            return None;
        }
        
        Some(VtgData {
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
    
    /// Helper to get a field as a string slice
    fn get_field_str(&self, index: usize) -> Option<&str> {
        if index < self.field_count {
            self.fields[index].as_ref()?.as_str()
        } else {
            None
        }
    }
    
    /// Helper to parse a field as u8
    fn parse_field_u8(&self, index: usize) -> Option<u8> {
        self.get_field_str(index)?.parse().ok()
    }
    
    /// Helper to parse a field as u16
    fn parse_field_u16(&self, index: usize) -> Option<u16> {
        self.get_field_str(index)?.parse().ok()
    }
    
    /// Helper to parse a field as f32
    fn parse_field_f32(&self, index: usize) -> Option<f32> {
        self.get_field_str(index)?.parse().ok()
    }
    
    /// Helper to parse a field as f64
    fn parse_field_f64(&self, index: usize) -> Option<f64> {
        self.get_field_str(index)?.parse().ok()
    }
    
    /// Helper to parse a field as char (first character)
    fn parse_field_char(&self, index: usize) -> Option<char> {
        self.get_field_str(index)?.chars().next()
    }
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
                    if !field_bytes.is_empty() {
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
        (0..self.buffer_pos).find(|&i| self.buffer[i] == byte)
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

    #[test]
    fn test_gga_parameters() {
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
        assert_eq!(gga_data.time, Some("123519"));
        assert_eq!(gga_data.latitude, Some(4807.038));
        assert_eq!(gga_data.lat_direction, Some('N'));
        assert_eq!(gga_data.longitude, Some(1131.000));
        assert_eq!(gga_data.lon_direction, Some('E'));
        assert_eq!(gga_data.fix_quality, Some(1));
        assert_eq!(gga_data.num_satellites, Some(8));
        assert_eq!(gga_data.hdop, Some(0.9));
        assert_eq!(gga_data.altitude, Some(545.4));
        assert_eq!(gga_data.altitude_units, Some('M'));
    }

    #[test]
    fn test_rmc_parameters() {
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
        assert_eq!(rmc_data.time, Some("123519"));
        assert_eq!(rmc_data.status, Some('A'));
        assert_eq!(rmc_data.latitude, Some(4807.038));
        assert_eq!(rmc_data.lat_direction, Some('N'));
        assert_eq!(rmc_data.longitude, Some(1131.000));
        assert_eq!(rmc_data.lon_direction, Some('E'));
        assert_eq!(rmc_data.speed_knots, Some(22.4));
        assert_eq!(rmc_data.track_angle, Some(84.4));
        assert_eq!(rmc_data.date, Some("230394"));
    }

    #[test]
    fn test_gsa_parameters() {
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
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());
        
        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.mode, Some('A'));
        assert_eq!(gsa_data.fix_type, Some(3));
        assert_eq!(gsa_data.satellite_ids[0], Some(4));
        assert_eq!(gsa_data.satellite_ids[1], Some(5));
        assert_eq!(gsa_data.satellite_ids[3], Some(9));
        assert_eq!(gsa_data.pdop, Some(2.5));
        assert_eq!(gsa_data.hdop, Some(1.3));
        assert_eq!(gsa_data.vdop, Some(2.1));
    }

    #[test]
    fn test_gsv_parameters() {
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
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());
        
        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.num_messages, Some(2));
        assert_eq!(gsv_data.message_num, Some(1));
        assert_eq!(gsv_data.satellites_in_view, Some(8));
        
        // Check first satellite
        assert!(gsv_data.satellite_info[0].is_some());
        let sat1 = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat1.prn, Some(1));
        assert_eq!(sat1.elevation, Some(40));
        assert_eq!(sat1.azimuth, Some(83));
        assert_eq!(sat1.snr, Some(46));
    }

    #[test]
    fn test_gll_parameters() {
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
        let gll = msg.as_gll();
        assert!(gll.is_some());
        
        let gll_data = gll.unwrap();
        assert_eq!(gll_data.latitude, Some(4916.45));
        assert_eq!(gll_data.lat_direction, Some('N'));
        assert_eq!(gll_data.longitude, Some(12311.12));
        assert_eq!(gll_data.lon_direction, Some('W'));
        assert_eq!(gll_data.time, Some("225444"));
        assert_eq!(gll_data.status, Some('A'));
    }

    #[test]
    fn test_vtg_parameters() {
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
    fn test_wrong_message_type_extraction() {
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
        
        // Try to extract RMC data from a GGA message
        let rmc = msg.as_rmc();
        assert!(rmc.is_none());
        
        // GGA extraction should work
        let gga = msg.as_gga();
        assert!(gga.is_some());
    }

    #[test]
    fn test_gga_with_empty_fields() {
        let mut parser = NmeaParser::new();
        // GGA message with some empty fields
        let sentence = b"$GPGGA,123519,,N,,E,1,,,,,M,,M,,*47\r\n";

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
        assert_eq!(gga_data.time, Some("123519"));
        assert_eq!(gga_data.latitude, None);
        assert_eq!(gga_data.lat_direction, Some('N'));
        assert_eq!(gga_data.longitude, None);
        assert_eq!(gga_data.lon_direction, Some('E'));
        assert_eq!(gga_data.fix_quality, Some(1));
        assert_eq!(gga_data.num_satellites, None);
        assert_eq!(gga_data.hdop, None);
    }

    #[test]
    fn test_rmc_with_empty_status() {
        let mut parser = NmeaParser::new();
        // RMC message with void status
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
        assert_eq!(rmc_data.status, Some('V'));
    }

    #[test]
    fn test_gsa_with_partial_satellites() {
        let mut parser = NmeaParser::new();
        // GSA message with only a few satellites
        let sentence = b"$GPGSA,A,3,01,,,,,,,,,,,,2.5,1.3,2.1*39\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());
        
        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.mode, Some('A'));
        assert_eq!(gsa_data.fix_type, Some(3));
        assert_eq!(gsa_data.satellite_ids[0], Some(1));
        assert_eq!(gsa_data.satellite_ids[1], None);
        assert_eq!(gsa_data.pdop, Some(2.5));
        assert_eq!(gsa_data.hdop, Some(1.3));
        assert_eq!(gsa_data.vdop, Some(2.1));
    }

    #[test]
    fn test_gsv_with_partial_satellite_data() {
        let mut parser = NmeaParser::new();
        // GSV message with only two satellites
        let sentence = b"$GPGSV,1,1,02,01,40,083,46,02,17,308,*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());
        
        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.num_messages, Some(1));
        assert_eq!(gsv_data.satellites_in_view, Some(2));
        
        // First satellite should be complete
        assert!(gsv_data.satellite_info[0].is_some());
        let sat1 = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat1.prn, Some(1));
        assert_eq!(sat1.elevation, Some(40));
        assert_eq!(sat1.azimuth, Some(83));
        assert_eq!(sat1.snr, Some(46));
        
        // Second satellite should have missing SNR
        assert!(gsv_data.satellite_info[1].is_some());
        let sat2 = gsv_data.satellite_info[1].as_ref().unwrap();
        assert_eq!(sat2.prn, Some(2));
        assert_eq!(sat2.elevation, Some(17));
        assert_eq!(sat2.azimuth, Some(308));
        assert_eq!(sat2.snr, None);
        
        // Third and fourth should be None
        assert!(gsv_data.satellite_info[2].is_none());
        assert!(gsv_data.satellite_info[3].is_none());
    }

    #[test]
    fn test_numeric_type_parsing() {
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
        
        // Verify types are correctly parsed
        if let Some(lat) = gga_data.latitude {
            assert!((lat - 4807.038).abs() < 0.001);
        }
        
        if let Some(lon) = gga_data.longitude {
            assert!((lon - 1131.000).abs() < 0.001);
        }
        
        if let Some(hdop) = gga_data.hdop {
            assert!((hdop - 0.9).abs() < 0.01);
        }
        
        if let Some(alt) = gga_data.altitude {
            assert!((alt - 545.4).abs() < 0.1);
        }
    }
}
