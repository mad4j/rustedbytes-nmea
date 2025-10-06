//! NMEA message types and data structures

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

/// GGA - Global Positioning System Fix Data parameters
#[derive(Debug, Clone)]
pub struct GgaData<'a> {
    pub time: &'a str,
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    pub fix_quality: u8,
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
    pub time: &'a str,
    pub status: char,
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    pub speed_knots: f32,
    pub track_angle: f32,
    pub date: &'a str,
    pub magnetic_variation: Option<f32>,
    pub mag_var_direction: Option<char>,
}

/// GSA - GPS DOP and active satellites parameters
#[derive(Debug, Clone)]
pub struct GsaData {
    pub mode: char,
    pub fix_type: u8,
    pub satellite_ids: [Option<u8>; 12],
    pub pdop: Option<f32>,
    pub hdop: Option<f32>,
    pub vdop: Option<f32>,
}

/// GSV - GPS Satellites in view parameters
#[derive(Debug, Clone)]
pub struct GsvData {
    pub num_messages: u8,
    pub message_num: u8,
    pub satellites_in_view: u8,
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
    pub latitude: f64,
    pub lat_direction: char,
    pub longitude: f64,
    pub lon_direction: char,
    pub time: &'a str,
    pub status: char,
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
