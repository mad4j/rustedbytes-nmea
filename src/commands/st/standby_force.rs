//! $PSTMFORCESTANDBY
//! Force the platform to go in standby mode.
//!
//! Parameter                   Format              Description
//! duration                    Decimal, 5 digits   Duration of the standby time in seconds
//!
//! $PSTMFORCESTANDBY,<duration>*<checksum><cr><lf>
use crate::Command;
use heapless::{format, String};

#[derive(Debug, Clone)]
pub struct ConfigureStandbyForce {
    pub duration_seconds: u32,
}

impl Command for ConfigureStandbyForce {
    const MAX_LEN: usize = 28;
    const CMD: &'static str = "PSTMFORCESTANDBY";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let duration_seconds = if self.duration_seconds > 99999 {
            99999
        } else {
            self.duration_seconds
        };
        let mut s = format!("${},{}", Self::CMD, duration_seconds)
            .map_err(|_| duration_seconds)
            .map_err(|_| ())?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_force_standby_command() {
        let command = ConfigureStandbyForce {
            duration_seconds: 120,
        };
        let expected_string = "$PSTMFORCESTANDBY,120*F\r\n";
        assert_eq!(command.to_string().unwrap(), expected_string);
    }

    #[test]
    fn test_force_standby_max_duration() {
        let command = ConfigureStandbyForce {
            duration_seconds: 100000, // Should be capped at 99999
        };
        let expected_string = "$PSTMFORCESTANDBY,99999*5\r\n";
        assert_eq!(command.to_string().unwrap(), expected_string);
    }

    #[test]
    fn test_force_standby_zero_duration() {
        let command = ConfigureStandbyForce {
            duration_seconds: 0,
        };
        let expected_string = "$PSTMFORCESTANDBY,0*C\r\n";
        assert_eq!(command.to_string().unwrap(), expected_string);
    }
}
