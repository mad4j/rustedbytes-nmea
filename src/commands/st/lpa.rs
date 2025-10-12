//! PSTMCFGLPA
//! Low power algorithm
//!
//! Parameter           Format              Description
//! en_pa               unsigned, 1 bytes   Enable Low Power Algorithm
//!                                             0 = LPA Disabled
//!                                             1 = LPA Enabled.
//! feat                unsigned, 1 bytes   Low Power Algorithm feature
//!                                             0 = Periodic mode disabled
//!                                             1 = Active Periodic mode
//!                                             2 = RESERVED
//!                                             3 = Standby Periodic mode
//! fix_period          From 0 to 86400     Fix period in seconds. 0 means the Fix will be given only on WAKEUP pin activation.
//!                                             Value 0 is only valid in Standby Periodic mode.
//!                                             Default is 10.
//! fix_on_time         unsigned, 2 bytes   Number of fix reported every Fix wakeup.
//!                                             Default is 1
//! no_fix_cnt          unsigned, 2 bytes   Number of no-fixes in hot conditions, before to signal a fix loss event.
//!                                             Default is 8
//! no_fix_cnt2         unsigned, 2 bytes   Number of no-fixes in non-hot conditions, before signaling a fix loss event.
//!                                             Default is 60
//! no fix_off          unsigned, 2 bytes   Off duration time after a fix loss event.
//!                                             Default is 180
//! adaptive_feat       unsigned, 1 bytes   Enable disable adaptive multi-constellation algorithm.
//!                                             0 = Adaptive Algorithm Disabled
//!                                             1 = Adaptive Algorithm Enabled
//!                                             Default is 0
//! adaptive_duty_cicle unsigned, 1 bytes   Enable disable trimming of correlation time for each cycle.
//!                                             0 = Adaptive Duty Cycle Disabled
//!                                             1 = Adaptive Duty Cycle Enabled
//!                                             Default is 0
//! ehpe_th             unsigned, 1 bytes   EHPE average threshold.
//!                                             Default is 15
//! num_of_sat          unsigned, 1 bytes   0 to 32
//!                                             Number of satellite used in Adaptive mode (first N with higher elevation)
//!                                             Default is 9
//! duty_off            unsigned, 2 bytes   100 to 740
//!                                             Duty cycle OFF period length in ms;
//!                                             Default is 700
//! const_type          unsigned, 1 bytes   RESERVED, set it as O
//!
//! $PSTMCFGLPA,<en_lpa>,<feat>,<fix_period>,<fix_on_time>,<no_fix_cnt>,<no_fix_cnt2>,<no_fix_off
//! >,<adaptive_feat>,<adaptive_duty_cicle>,<ehpe_th>,<num_of_sat>,<duty_off>,<const_type>*<check
//! sum><cr><lf>

use heapless::{format, String};
use crate::commands::Command;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum LowPowerAlgorithmFeature {
    PeriodicModeDisabled,
    ActivePeriodicMode,
    StandbyPeriodicMode,
}

#[derive(Debug, Clone)]
pub struct ConfigureLowPowerAlgorithm {
    pub en_pa: bool,
    pub feat: LowPowerAlgorithmFeature,
    pub fix_period: u16,
    pub fix_on_time: u16,
    pub no_fix_cnt: u16,
    pub no_fix_cnt2: u16,
    pub no_fix_off: u16,
    pub adaptive_feat: bool,
    pub adaptive_duty_cicle: bool,
    pub ehpe_th: u8,
    pub num_of_sat: u8,
    pub duty_off: u16,
    pub const_type: u8,
}

impl Command for ConfigureLowPowerAlgorithm {
    // 31 for command, commas, checksum and crlf
    const MAX_LEN: usize = 31 + 21;
    const CMD: &'static str = "PSTMCFGLPA";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let en_pa = if self.en_pa { 1 } else { 0 };
        let feat = match self.feat {
            LowPowerAlgorithmFeature::PeriodicModeDisabled => 0,
            LowPowerAlgorithmFeature::ActivePeriodicMode => 1,
            LowPowerAlgorithmFeature::StandbyPeriodicMode => 3,
        };
        let mut s = format!(
            "${},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            Self::CMD,
            en_pa,
            feat,
            self.fix_period,
            self.fix_on_time,
            self.no_fix_cnt,
            self.no_fix_cnt2,
            self.no_fix_off,
            if self.adaptive_feat { 1 } else { 0 },
            if self.adaptive_duty_cicle { 1 } else { 0 },
            self.ehpe_th,
            self.num_of_sat,
            self.duty_off,
            self.const_type
        ).map_err(|_| ())?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lpa_command() {
        let lpa = ConfigureLowPowerAlgorithm {
            en_pa: true,
            feat: LowPowerAlgorithmFeature::ActivePeriodicMode,
            fix_period: 10,
            fix_on_time: 1,
            no_fix_cnt: 8,
            no_fix_cnt2: 60,
            no_fix_off: 180,
            adaptive_feat: false,
            adaptive_duty_cicle: false,
            ehpe_th: 15,
            num_of_sat: 9,
            duty_off: 700,
            const_type: 0,
        };
        let command_string = lpa.to_string().unwrap();
        assert_eq!(
            command_string,
            "$PSTMCFGLPA,1,1,10,1,8,60,180,0,0,15,9,700,0*24\r\n"
        );
    }

    #[test]
    fn test_lpa_command_standby_periodic_mode() {
        let lpa = ConfigureLowPowerAlgorithm {
            en_pa: true,
            feat: LowPowerAlgorithmFeature::StandbyPeriodicMode,
            fix_period: 0, // Valid for Standby Periodic mode
            fix_on_time: 1,
            no_fix_cnt: 8,
            no_fix_cnt2: 60,
            no_fix_off: 180,
            adaptive_feat: true,
            adaptive_duty_cicle: true,
            ehpe_th: 20,
            num_of_sat: 12,
            duty_off: 500,
            const_type: 0,
        };
        let command_string = lpa.to_string().unwrap();
        assert_eq!(
            command_string,
            "$PSTMCFGLPA,1,3,0,1,8,60,180,1,1,20,12,500,0*29\r\n"
        );
    }
}