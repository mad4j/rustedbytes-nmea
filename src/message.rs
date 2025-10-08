//! NMEA message representation and field parsing
//!
//! This module provides the core data structures for representing parsed NMEA messages
//! and fields. Message-specific parsing implementations are included in separate
//! submodules for each message type.

use crate::types::*;

// Message type implementations
mod gga;
mod gll;
mod gns;
mod gsa;
mod gsv;
mod rmc;
mod vtg;

// Re-export message data structures
pub use gga::GgaData;
pub use gll::GllData;
pub use gns::GnsData;
pub use gsa::GsaData;
pub use gsv::{GsvData, SatelliteInfo};
pub use rmc::RmcData;
pub use vtg::VtgData;

/// Maximum number of fields in an NMEA sentence
pub(crate) const MAX_FIELDS: usize = 20;

/// Parsed NMEA sentence data (internal representation)
///
/// Represents a single parsed NMEA sentence with its type, fields, and metadata.
/// This is an internal structure used during parsing.
#[derive(Debug, Clone)]
pub(crate) struct ParsedSentence {
    pub message_type: MessageType,
    pub talker_id: TalkerId,
    pub fields: [Option<Field>; MAX_FIELDS],
    pub field_count: usize,
}

impl ParsedSentence {
    /// Helper to get a field as a string slice
    pub(crate) fn get_field_str(&self, index: usize) -> Option<&str> {
        if index < self.field_count {
            self.fields[index].as_ref()?.as_str()
        } else {
            None
        }
    }

    /// Generic helper to parse a field using FromStr trait
    pub(crate) fn parse_field<T>(&self, index: usize) -> Option<T>
    where
        T: core::str::FromStr,
    {
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
    data: [u8; 16], // Reduced from 32 to 16 bytes - sufficient for most NMEA fields
    len: u8,        // Changed from usize to u8 for memory efficiency
}

impl Field {
    /// Create a field from a byte slice
    ///
    /// Copies up to 16 bytes from the input slice.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        let copy_len = bytes.len().min(16);
        let mut data = [0; 16];
        data[..copy_len].copy_from_slice(&bytes[..copy_len]);

        Field {
            data,
            len: copy_len as u8,
        }
    }

    /// Get the field as a string slice
    ///
    /// Returns `None` if the field contains invalid UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.data[..self.len as usize]).ok()
    }

    /// Get the field as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}
