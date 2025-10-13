//! $PSTMCFGGEOCIR
//! Allows to configure a circle of geofencing feature.
//!
//! Parameter               Format              Description
//! circleid                Decimal, 1 digit    The circle ID
//!                                                 From 0 to 7
//! en                      Boolean             Enable disable the circle
//!                                                 0 = Disable,
//!                                                 1 = Enable
//! lat                     Double              N-th circle latitude
//! lon                     Double              N-th circle longitude
//! rad                     Double              N-th circle radius
//!
//! $PSTMCFGGEOCIR, <circleid›,<en>,<lat>,<lon›,<rad›*<checksum><cr><1f>

use crate::{Command, CommandError};
use heapless::{format, String};

#[derive(Debug, Clone)]
pub struct ConfigureGeofenceCircle {
    pub circle_id: u8,
    pub en: bool,
    pub lat: f64,
    pub lon: f64,
    pub rad: f64,
}

impl Command for ConfigureGeofenceCircle {
    const MAX_LEN: usize = 24 + 47;
    const CMD: &'static str = "PSTMCFGGEOCIR";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, CommandError> {
        let mut s = format!(
            "${},{},{},{:.8},{:.8},{:.8}",
            Self::CMD,
            self.circle_id,
            if self.en { 1 } else { 0 },
            self.lat,
            self.lon,
            self.rad
        )?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configure_geofence_circle_to_string() {
        let cmd = ConfigureGeofenceCircle {
            circle_id: 0,
            en: true,
            lat: 40.7128,
            lon: -74.0060,
            rad: 100.0,
        };
        let expected = "$PSTMCFGGEOCIR,0,1,40.71280000,-74.00600000,100.00000000*5F\r\n";
        let actual = cmd.to_string().unwrap();
        assert_eq!(actual, expected);

        let cmd = ConfigureGeofenceCircle {
            circle_id: 7,
            en: false,
            lat: 34.0522,
            lon: -118.2437,
            rad: 500.5,
        };
        let expected = "$PSTMCFGGEOCIR,7,0,34.05220000,-118.24370000,500.50000000*6D\r\n";
        let actual = cmd.to_string().unwrap();
        assert_eq!(actual, expected);
    }
}
