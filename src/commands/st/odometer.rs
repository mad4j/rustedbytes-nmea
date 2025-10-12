//! $PSTMCFGODO
//!
//! Configure the Odometer.
//!
//! Parameter           Format              Description
//! en                  Decimal, 1 digit    Enable/Disable the odometer:
//!                                             0 = Odometer disabled
//!                                             1 = Odometer enabled
//! enmsg               Decimal, 1 digit    Enable/Disable odometer related periodic messages:
//!                                             0 = Periodic message disabled
//!                                             1 = Periodic message enabled
//! alarm               0 to 65535          Distance travelled between two NMEA messages
//!
//! $PSTMCFGODO,<en>,<enmsg>,<alarm>*<checksum><cr><lf>

use heapless::{format, String};
use crate::commands::Command;

#[derive(Debug, Clone)]
pub struct ConfigureOdometer {
    pub en: bool,
    pub enmsg: bool,
    pub alarm: u16,
}

impl Command for ConfigureOdometer {

    // 31 for command, commas, checksum and crlf
    const MAX_LEN: usize = 19 + 7;
    const CMD: &'static str = "PSTMCFGODO";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let en = if self.en { 1 } else { 0 };
        let enmsg = if self.enmsg { 1 } else { 0 };
        let mut s = format!(
            "${},{},{},{}",
            Self::CMD,
            en,
            enmsg,
            self.alarm
        ).map_err(|_| ())?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_configure_odometer_to_string() {
        let cmd = ConfigureOdometer {
            en: true,
            enmsg: false,
            alarm: 100,
        };
        let expected = "$PSTMCFGODO,1,0,100*0\r\n";
        let actual = cmd.to_string().unwrap();
        assert_eq!(actual, expected);

        let cmd = ConfigureOdometer {
            en: false,
            enmsg: true,
            alarm: 65535,
        };
        let expected = "$PSTMCFGODO,0,1,65535*1\r\n";
        let actual = cmd.to_string().unwrap();
        assert_eq!(actual, expected);
    }

}