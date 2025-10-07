//! NMEA message types and data structures

use crate::message::{GgaData, GllData, GsaData, GsvData, RmcData, VtgData};

/// Parse error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// Checksum verification failed
    InvalidChecksum,
    /// Message is syntactically complete but missing mandatory fields
    InvalidMessage,
}

/// Represents the GNSS constellation (talker ID)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TalkerId {
    GP, // GPS
    GL, // GLONASS
    GA, // Galileo
    GB, // BeiDou
    GN, // Multi-GNSS (combined)
    BD, // BeiDou (alternative)
    QZ, // QZSS (Quasi-Zenith Satellite System)
    Unknown,
}

/// Represents the different NMEA message type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    GGA, // Global Positioning System Fix Data
    RMC, // Recommended Minimum Navigation Information
    GSA, // GPS DOP and active satellites
    GSV, // GPS Satellites in view
    GLL, // Geographic Position - Latitude/Longitude
    VTG, // Track Made Good and Ground Speed
    Unknown,
}

/// Parsed NMEA message with associated data
#[derive(Debug, Clone)]
pub enum NmeaMessage {
    GGA(GgaData),
    RMC(RmcData),
    GSA(GsaData),
    GSV(GsvData),
    GLL(GllData),
    VTG(VtgData),
}

impl NmeaMessage {
    /// Get the message type
    pub fn message_type(&self) -> MessageType {
        match self {
            NmeaMessage::GGA(_) => MessageType::GGA,
            NmeaMessage::RMC(_) => MessageType::RMC,
            NmeaMessage::GSA(_) => MessageType::GSA,
            NmeaMessage::GSV(_) => MessageType::GSV,
            NmeaMessage::GLL(_) => MessageType::GLL,
            NmeaMessage::VTG(_) => MessageType::VTG,
        }
    }

    /// Get the talker ID
    pub fn talker_id(&self) -> TalkerId {
        match self {
            NmeaMessage::GGA(d) => d.talker_id,
            NmeaMessage::RMC(d) => d.talker_id,
            NmeaMessage::GSA(d) => d.talker_id,
            NmeaMessage::GSV(d) => d.talker_id,
            NmeaMessage::GLL(d) => d.talker_id,
            NmeaMessage::VTG(d) => d.talker_id,
        }
    }

    /// Extract GGA data if this is a GGA message
    pub fn as_gga(&self) -> Option<&GgaData> {
        if let NmeaMessage::GGA(data) = self {
            Some(data)
        } else {
            None
        }
    }

    /// Extract RMC data if this is an RMC message
    pub fn as_rmc(&self) -> Option<&RmcData> {
        if let NmeaMessage::RMC(data) = self {
            Some(data)
        } else {
            None
        }
    }

    /// Extract GSA data if this is a GSA message
    pub fn as_gsa(&self) -> Option<&GsaData> {
        if let NmeaMessage::GSA(data) = self {
            Some(data)
        } else {
            None
        }
    }

    /// Extract GSV data if this is a GSV message
    pub fn as_gsv(&self) -> Option<&GsvData> {
        if let NmeaMessage::GSV(data) = self {
            Some(data)
        } else {
            None
        }
    }

    /// Extract GLL data if this is a GLL message
    pub fn as_gll(&self) -> Option<&GllData> {
        if let NmeaMessage::GLL(data) = self {
            Some(data)
        } else {
            None
        }
    }

    /// Extract VTG data if this is a VTG message
    pub fn as_vtg(&self) -> Option<&VtgData> {
        if let NmeaMessage::VTG(data) = self {
            Some(data)
        } else {
            None
        }
    }
}
