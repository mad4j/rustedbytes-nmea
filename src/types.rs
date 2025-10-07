//! NMEA message types and data structures

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

/// Represents the different NMEA message types
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
