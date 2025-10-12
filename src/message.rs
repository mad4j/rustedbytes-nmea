//! NMEA message representation and field parsing
//!
//! This module provides the core data structures for representing parsed NMEA messages
//! and fields. Message-specific parsing implementations are included in separate
//! submodules for each message type.

use core::ops::{BitOrAssign, ShlAssign};
use crate::types::*;

// Message type implementations
mod gga;
mod gll;
mod gns;
mod gsa;
mod gsv;
mod rmc;
mod st;
mod vtg;

// Re-export message data structures
pub use gga::GgaData;
pub use gll::GllData;
pub use gns::GnsData;
pub use gsa::GsaData;
pub use gsv::{GsvData, SatelliteInfo};
pub use rmc::RmcData;
pub use st::StMessageData;
pub use vtg::VtgData;

/// Maximum number of fields in an NMEA sentence
#[cfg(not(feature = "st-teseo-liv3"))]
pub(crate) const MAX_FIELDS: usize = 20;

#[cfg(not(feature = "st-teseo-liv3"))]
pub(crate) const FIELD_SIZE_BYTES: usize = 16;

#[cfg(feature = "st-teseo-liv3")]
pub(crate) const MAX_FIELDS: usize = 40;

#[cfg(feature = "st-teseo-liv3")]
pub(crate) const FIELD_SIZE_BYTES: usize = 32;

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

    /// Helper to parse a field as a hexadecimal
    pub(crate) fn parse_hex_field<T>(&self, index: usize) -> Option<T>
    where
        T: From<u8> + Default + ShlAssign + BitOrAssign,
    {
        let hex_str = self.get_field_str(index)?;
        let mut val = T::default();

        for c in hex_str.chars() {
            let c = match c {
                'a'..='f' => c as u8 - ('a' as u8) + 10,
                'A'..='F' => c as u8 - ('A' as u8) + 10,
                '0'..='9' => c as u8 - '0' as u8,
                _ => return None,
            };

            val <<= 4.into();
            val |= c.into();
        }

        Some(val)
    }
}

/// Represents a field value in an NMEA message
///
/// A field is a single data element within an NMEA sentence, stored as a
/// fixed-size byte array with length tracking. This provides `no_std` compatible
/// string storage without heap allocation.
#[derive(Debug, Clone, Copy)]
pub struct Field {
    data: [u8; FIELD_SIZE_BYTES], // Sufficient for most NMEA fields
    len: u8,        // Changed from usize to u8 for memory efficiency
}

impl Field {
    /// Create a field from a byte slice
    ///
    /// Copies up to FIELD_SIZE_BYTES bytes from the input slice.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        let copy_len = bytes.len().min(FIELD_SIZE_BYTES);
        let mut data = [0; FIELD_SIZE_BYTES];
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


#[cfg(test)]
mod tests {
    use crate::message::{ParsedSentence, MAX_FIELDS};
    use crate::{Field, MessageType, TalkerId};

    #[test]
    fn test_parse_hex_field() {
        let mut sentence = ParsedSentence {
            message_type: MessageType::GGA,
            talker_id: TalkerId::GP,
            fields: [None; MAX_FIELDS],
            field_count: 1,
        };
        [
            (b"1" as &[u8], Some(0x1)),
            (b"A1B2C3D4E5F6A7B8", Some(0xA1B2C3D4E5F6A7B8)),
            (b"ABC", Some(0xABC)),  // Uneven amount of characters
            (b"GHIJ", None),        // Invalid hex
            (b"", Some(0)),         // Empty field
        ].into_iter().for_each(|(s, o)| {
            sentence.fields[0] = Some(Field::from_bytes(s));
            assert_eq!(sentence.parse_hex_field::<u64>(0), o);
        });

        [
            (b"1" as &[u8], Some(0x1)),
            (b"A1B2C3D4", Some(0xA1B2C3D4)),
            (b"ABC", Some(0xABC)),  // Uneven amount of characters
            (b"GHIJ", None),        // Invalid hex
            (b"", Some(0)),         // Empty field
        ].into_iter().for_each(|(s, o)| {
            sentence.fields[0] = Some(Field::from_bytes(s));
            assert_eq!(sentence.parse_hex_field::<u32>(0), o);
        });

        [
            (b"1" as &[u8], Some(0x1)),
            (b"A1B2", Some(0xA1B2)),
            (b"ABC", Some(0xABC)),  // Uneven amount of characters
            (b"GHIJ", None),        // Invalid hex
            (b"", Some(0)),         // Empty field
        ].into_iter().for_each(|(s, o)| {
            sentence.fields[0] = Some(Field::from_bytes(s));
            assert_eq!(sentence.parse_hex_field::<u16>(0), o);
        });
    }
}