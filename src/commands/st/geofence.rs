//! $PSTMCFGGEOFENCE
//! Allows to configure Geofencing feature enabling circles and choosing tolerance.
//!
//! Parameter           Format              Description
//! en                  Decimal, 1 digit    Enable/Disable the geofencing:
//!                                             0 = Geo fencing disabled
//!                                             1 = Geo fencing enabled
//! tol                 Decimal, 1 digit    Tolerance:
//!                                             0 = none
//!                                             1 = level 1
//!                                             2 = level 2
//!                                             3 = level 3
//!
//! $PSTMCGGEOFENCE, <en>, <tol>*<checksum><cr><lf>

use heapless::{format, String};
use crate::Command;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GeofenceToleranceLevel {
    None = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3
}

#[derive(Debug, Clone)]
pub struct ConfigureEnableGeofenceCircles {
    pub en: bool,
    pub tol: GeofenceToleranceLevel,
}

impl Command for ConfigureEnableGeofenceCircles {
    const MAX_LEN: usize = 22 + 2;
    const CMD: &'static str = "PSTMCGGEOFENCE";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let mut s = format!(
            "${},{},{}",
            Self::CMD,
            if self.en { 1 } else { 0 },
            self.tol as u8
        ).map_err(|_| ())?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configure_enable_geofence_circles() {
        let cmd = ConfigureEnableGeofenceCircles {
            en: true,
            tol: GeofenceToleranceLevel::Level1,
        };
        assert_eq!(
            cmd.to_string().unwrap(),
            "$PSTMCGGEOFENCE,1,1*18\r\n"
        );
    }

    #[test]
    fn test_configure_enable_geofence_circles_disabled() {
        let cmd = ConfigureEnableGeofenceCircles {
            en: false,
            tol: GeofenceToleranceLevel::None,
        };
        assert_eq!(
            cmd.to_string().unwrap(),
            "$PSTMCGGEOFENCE,0,0*18\r\n"
        );
    }

    #[test]
    fn test_configure_enable_geofence_circles_level3() {
        let cmd = ConfigureEnableGeofenceCircles {
            en: true,
            tol: GeofenceToleranceLevel::Level3,
        };
        assert_eq!(
            cmd.to_string().unwrap(),
            "$PSTMCGGEOFENCE,1,3*1A\r\n"
        );
    }

}

