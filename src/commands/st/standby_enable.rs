//! $PSTMSTANDBYENABLE
//! When the Periodic mode is configured with $PSTMLOWPOWERONOFF, this command allows/disallows the
//! Teseo to go in Standby mode between the fixes.
//!
//! Parameter                   Format              Description
//! Without parameter                               Request the internal status
//!
//! With parameter
//! on_off                      Decimal, 1 digits   Set the standby enable status
//!                                                     0: Active Periodic mode
//!                                                     1: Periodic mode, standby allowed
//! Without parameter:
//! $PSTMSTANDBYENABLE,<checksum><cr><lf>
//! With parameter:
//! $PSTMSTANDBYENABLE,<on_off>*<checksum><cr><lf>

use crate::commands::nmea_checksum;
use crate::Command;
use heapless::{format, String};

#[derive(Debug, Clone)]
pub struct StandbyEnableCheckStatus {}

impl Command for StandbyEnableCheckStatus {
    const MAX_LEN: usize = 23;
    const CMD: &'static str = "PSTMSTANDBYENABLE";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let mut s = format!("${},", Self::CMD).map_err(|_| ())?;
        let checksum = nmea_checksum(s.as_str());
        let checksum: String<2> = format!("{:X}", checksum).map_err(|_| ())?;
        s.push_str(checksum.as_str()).map_err(|_| ())?;
        s.push_str("\r\n").map_err(|_| ())?;
        Ok(s)
    }
}

#[derive(Debug, Clone)]
pub struct ConfigureStandbyEnable {
    pub on_off: bool,
}

impl Command for ConfigureStandbyEnable {
    const MAX_LEN: usize = 25;
    const CMD: &'static str = "PSTMSTANDBYENABLE";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let mut s =
            format!("${},{}", Self::CMD, if self.on_off { 1 } else { 0 }).map_err(|_| ())?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standby_enable_check_status_to_string() {
        let cmd = StandbyEnableCheckStatus {};
        let expected = "$PSTMSTANDBYENABLE,60\r\n";
        assert_eq!(cmd.to_string().unwrap(), expected);
    }

    #[test]
    fn test_configure_standby_enable_on_to_string() {
        let cmd = ConfigureStandbyEnable { on_off: true };
        let expected = "$PSTMSTANDBYENABLE,1*51\r\n";
        assert_eq!(cmd.to_string().unwrap(), expected);
    }

    #[test]
    fn test_configure_standby_enable_off_to_string() {
        let cmd = ConfigureStandbyEnable { on_off: false };
        let expected = "$PSTMSTANDBYENABLE,0*50\r\n";
        assert_eq!(cmd.to_string().unwrap(), expected);
    }
}
