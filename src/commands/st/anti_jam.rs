//! $PSTMCFGAJM
//! Configure the Anti-Jamming Algorithm.
//!
//! Parameter       Format              Description
//! gpsmode         Decimal, 1 digit    Notch filter on GPS path:
//!                                         0 = Disable
//!                                         1 = Normal Mode
//!                                         2 = Auto Mode
//! glonassmode     Decimal, 1 digit    Notch filter on GLONASS path:
//!                                         0 = Disable
//!                                         1 = Normal Mode
//!                                         2 = Auto Mode
//!
//! $PSTMCFGAJM,‹gpsmode>,‹glonas smode›*<checksum><cr><lf>

use crate::Command;
use heapless::{format, String};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NotchFilterMode {
    Disable = 0,
    Normal = 1,
    Auto = 2,
}

#[derive(Debug, Clone)]
pub struct ConfigureAntiJamming {
    pub gpsmode: NotchFilterMode,
    pub glonassmode: NotchFilterMode,
}

impl Command for ConfigureAntiJamming {
    const MAX_LEN: usize = 18 + 2;
    const CMD: &'static str = "PSTMCFGAJM";
    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let mut s = format!(
            "${},{},{}",
            Self::CMD,
            self.gpsmode as u8,
            self.glonassmode as u8
        )
        .map_err(|_| ())?;
        self.append_checksum_and_crlf(&mut s)?;

        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_configure_anti_jamming() {
        let cmd = ConfigureAntiJamming {
            gpsmode: NotchFilterMode::Auto,
            glonassmode: NotchFilterMode::Normal,
        };
        assert_eq!(cmd.to_string().unwrap(), "$PSTMCFGAJM,2,1*1D\r\n");
    }
}
