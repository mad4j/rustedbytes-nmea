//! NMEA message representation and field parsing

use crate::types::*;

/// Maximum number of fields in an NMEA sentence
pub(crate) const MAX_FIELDS: usize = 20;

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

        // Validate mandatory fields
        let time = self.get_field_str(1)?;
        let latitude = self.parse_field_f64(2)?;
        let lat_direction = self.parse_field_char(3)?;
        let longitude = self.parse_field_f64(4)?;
        let lon_direction = self.parse_field_char(5)?;
        let fix_quality = self.parse_field_u8(6)?;

        Some(GgaData {
            time,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            fix_quality,
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

        // Validate mandatory fields
        let time = self.get_field_str(1)?;
        let status = self.parse_field_char(2)?;
        let latitude = self.parse_field_f64(3)?;
        let lat_direction = self.parse_field_char(4)?;
        let longitude = self.parse_field_f64(5)?;
        let lon_direction = self.parse_field_char(6)?;
        let speed_knots = self.parse_field_f32(7)?;
        let track_angle = self.parse_field_f32(8)?;
        let date = self.get_field_str(9)?;

        Some(RmcData {
            time,
            status,
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            speed_knots,
            track_angle,
            date,
            magnetic_variation: self.parse_field_f32(10),
            mag_var_direction: self.parse_field_char(11),
        })
    }

    /// Extract GSA message parameters
    pub fn as_gsa(&self) -> Option<GsaData> {
        if self.message_type != MessageType::GSA {
            return None;
        }

        // Validate mandatory fields
        let mode = self.parse_field_char(1)?;
        let fix_type = self.parse_field_u8(2)?;

        Some(GsaData {
            mode,
            fix_type,
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

        // Validate mandatory fields
        let num_messages = self.parse_field_u8(1)?;
        let message_num = self.parse_field_u8(2)?;
        let satellites_in_view = self.parse_field_u8(3)?;

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
            num_messages,
            message_num,
            satellites_in_view,
            satellite_info: [sat1, sat2, sat3, sat4],
        })
    }

    /// Extract GLL message parameters
    pub fn as_gll(&self) -> Option<GllData<'_>> {
        if self.message_type != MessageType::GLL {
            return None;
        }

        // Validate mandatory fields
        let latitude = self.parse_field_f64(1)?;
        let lat_direction = self.parse_field_char(2)?;
        let longitude = self.parse_field_f64(3)?;
        let lon_direction = self.parse_field_char(4)?;
        let time = self.get_field_str(5)?;
        let status = self.parse_field_char(6)?;

        Some(GllData {
            latitude,
            lat_direction,
            longitude,
            lon_direction,
            time,
            status,
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
    pub(crate) fn new() -> Self {
        Field {
            data: [0; 32],
            len: 0,
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
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
