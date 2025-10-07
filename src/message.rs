//! NMEA message representation and field parsing
//!
//! This module provides the core data structures for representing parsed NMEA messages
//! and fields. Message-specific parsing implementations are included in separate
//! submodules for each message type.

use crate::types::*;

// Message type implementations
mod gga;
mod gll;
mod gsa;
mod gsv;
mod rmc;
mod vtg;

// Re-export message data structures
pub use gga::GgaData;
pub use gll::GllData;
pub use gsa::GsaData;
pub use gsv::{GsvData, SatelliteInfo};
pub use rmc::RmcData;
pub use vtg::VtgData;

/// Maximum number of fields in an NMEA sentence
pub(crate) const MAX_FIELDS: usize = 20;

/// Parsed NMEA message data
///
/// Represents a single parsed NMEA sentence with its type, fields, and metadata.
#[derive(Debug, Clone)]
pub struct NmeaMessage {
    pub message_type: MessageType,
    pub talker_id: TalkerId,
    pub fields: [Option<Field>; MAX_FIELDS],
    pub field_count: usize,
    pub timestamp: u64,
}

impl NmeaMessage {
    /// Helper to get a field as a string slice
    pub(crate) fn get_field_str(&self, index: usize) -> Option<&str> {
        if index < self.field_count {
            self.fields[index].as_ref()?.as_str()
        } else {
            None
        }
    }

    /// Helper to parse a field as u8
    pub(crate) fn parse_field_u8(&self, index: usize) -> Option<u8> {
        self.get_field_str(index)?.parse().ok()
    }

    /// Helper to parse a field as u16
    pub(crate) fn parse_field_u16(&self, index: usize) -> Option<u16> {
        self.get_field_str(index)?.parse().ok()
    }

    /// Helper to parse a field as f32
    pub(crate) fn parse_field_f32(&self, index: usize) -> Option<f32> {
        self.get_field_str(index)?.parse().ok()
    }

    /// Helper to parse a field as f64
    pub(crate) fn parse_field_f64(&self, index: usize) -> Option<f64> {
        self.get_field_str(index)?.parse().ok()
    }

    /// Helper to parse a field as char (first character)
    pub(crate) fn parse_field_char(&self, index: usize) -> Option<char> {
        self.get_field_str(index)?.chars().next()
    }
}

/// Represents a field value in an NMEA message
///
/// A field is a single data element within an NMEA sentence, stored as a
/// fixed-size byte array with length tracking. This provides `no_std` compatible
/// string storage without heap allocation.
#[derive(Debug, Clone, Copy)]
pub struct Field {
    data: [u8; 32],
    len: usize,
}

impl Field {
    /// Create a new empty field
    pub(crate) fn new() -> Self {
        Field {
            data: [0; 32],
            len: 0,
        }
    }

    /// Create a field from a byte slice
    ///
    /// Copies up to 32 bytes from the input slice.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        let mut field = Field::new();
        let copy_len = bytes.len().min(32);
        field.data[..copy_len].copy_from_slice(&bytes[..copy_len]);
        field.len = copy_len;
        field
    }

    /// Get the field as a string slice
    ///
    /// Returns `None` if the field contains invalid UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.data[..self.len]).ok()
    }

    /// Get the field as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }
}
