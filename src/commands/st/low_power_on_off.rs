//! $PSTMLOWPOWERONOFF
//! Allow setting the low power algorithm parameters at run-time.
//!
//! Parameter                       Format              Description
//! Low power enable/ disable       Decimal, 1 digit    General Low Power features Enable/Disable
//!                                                     O: OFF, 1: ON
//! Adaptive mode settings
//! Constellation mask              Decimal, 3 digit    It is a bit mask where each bit enable/disable a specific constellation
//!                                                     independently by the others:
//!                                                         bit O: GPS constellation enabling/disabling
//!                                                         bit 1: GLONASS constellation enabling/disabling
//!                                                         bit 2: QZSS constellation enabling/disabling
//!                                                         bit 3: GALILEO constellation enabling/disabling
//!                                                         bit 7: BEIDOU constellation enabling/disabling
//! EHPE threshold                  Decimal, 3 digits   EHPE average threshold [m]
//! Max tracked sats                Decimal, 2 digits   First N satellites (with higher elevation) used for the
//!                                                     position calculation (Active channel management) in LOW POWER STATE
//! Switch constellation features   Decimal, 1 digit    Switch constellation features (enable it only for GNSS constellation case)
//!
//! Cyclic mode settings
//! Duty Cycle enable/ disable      Decimal, 1 digit    Enable/Disable the Cyclic mode
//!                                                     O: OFF, 1: ON
//!                                                     This parameter can only be enabled if "Periodic mode" parameter is 0
//! Duty Cycle fix period           Decimal, 1 digits   Time between 2 fixes
//!                                                     Typical value: 1, 3, 5
//!                                                     The receiver provide a fix every fix period
//! 
//! Periodic mode settings
//! Periodic mode                   Decimal, 1 digit    Setup Active or Standby periodic mode
//!                                                     0: OFF
//!                                                     1: Active Periodic mode
//!                                                     3: Standby Periodic mode
//! FixPeriod                       Decimal, 5 digits   Interval between two fixes [s]. O means no periodic fix is required.
//! FixOnTime                       Decimal, 2 digits   Number of fixes reported for each interval
//! Ephemeris refresh               Decimal, 1 digit    Enable/Disable the refresh of ephemeris data
//!                                                         O: OFF, 1: ON
//! RTC calibration                 Decimal, 1 digit    Enable/Disable the RTC calibration
//!                                                         O: OFF, 1: ON
//! NoFixCnt                        Decimal, 2 digits   Time to declare fix loss [s] in HOT conditions
//! NoFixOff                        Decimal, 2 digits   Period of off period after a fix loss [s]. 0 means the counter is not active.
//!                                                         The fix retry will be based on FixPeriod.
//!
//! $PSTMLOWPOWERONOFF,<low power enable/disable›,<constellation mask>,<EHPE threshold›, <Max tracked sats>,
//!         <Switch constellation features >,<Duty Cycle enable/disable>,<Duty Cycle fix period›,
//!         <Periodic mode›,<Fix period›,<Number of fix>,<Ephemeris refresh>,<RTC refresh›,<No Fix timeout>,
//!         <No Fix timeout Off duration>*<checksum><cr><lf>

//! To Disable Low Power mode:
//! $PSTMLOWPOWERONOFF, 0, <constellation mask›>*<checksum><cr><lf>
//! To Enable Adaptive/Cycling Mode:
//! $PSTMLOWPOWERONOFF, 1, ‹constellation mask›, <EHPE threshold›,‹Max tracked sats>,<Switch constellation features >, <DutyCycle enable/disable>,<Duty Cycle fix period>, 0,0,0,0,0, 0,0*<checksum><cr><lf>
//! To Enable Periodic Mode:
//! $PSTMLOWPOWERONOFF, 1,0, 0, 0, 0, 0, 0, <Periodic mode›, ‹Fixperiod›,<Number of fix>,<Ephemeris refresh›, <RTC refresh›,<No Fix timeout>,<No Fix timeout Off duration›*<checksum><cr><lf>

use heapless::{format, String};
use crate::Command;

#[derive(Debug, Clone)]
pub struct ConstellationMask {
    pub gps: bool,
    pub glonass: bool,
    pub qzss: bool,
    pub galileo: bool,
    pub beidou: bool
}

impl ConstellationMask {
    pub fn to_string(&self) -> Result<String<3>, ()> {
        let val = 0u32
            | if self.gps { 0b0000_0001 } else { 0 }
            | if self.glonass { 0b0000_0010 } else { 0 }
            | if self.qzss { 0b0000_0100 } else { 0 }
            | if self.galileo { 0b0000_1000 } else { 0 }
            | if self.beidou { 0b1000_0000 } else { 0 };
        let val: String<3>  = format!("{}", val).map_err(|_| ())?;
        Ok(val)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PeriodicMode {
    Off,
    Active,
    Standby
}

impl PeriodicMode {
    pub fn to_string(&self) -> Result<String<1>, ()> {
        let val = match self {
            PeriodicMode::Off => 0,
            PeriodicMode::Active => 1,
            PeriodicMode::Standby => 3
        };
        let val: String<1> = format!("{}", val).map_err(|_| ())?;
        Ok(val)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConfigureLowPowerOnOff {
    pub low_power_enable: bool,
    pub constellation_mask: ConstellationMask,
    pub ehpe_threshold: u32,
    pub max_tracked_satellites: u8,
    pub switch_constellation_features: bool,
    pub duty_cycle_enable: bool,
    pub duty_cycle_fix_period: u8,
    pub periodic_mode: PeriodicMode,
    pub fix_period: u32,
    pub fix_on_time: u8,
    pub ephemeris_refresh: bool,
    pub rtc_calibration: bool,
    pub no_fix_cnt: u8,
    pub no_fix_off: u8,
}

impl Command for ConfigureLowPowerOnOff {

    const MAX_LEN: usize = 35 + 26;
    const CMD: &'static str = "PSTMLOWPOWERONOFF";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()> {
        let Self {
            low_power_enable,
            constellation_mask,
            ehpe_threshold,
            max_tracked_satellites,
            switch_constellation_features,
            duty_cycle_enable,
            duty_cycle_fix_period,
            periodic_mode,
            fix_period,
            fix_on_time,
            ephemeris_refresh,
            rtc_calibration,
            no_fix_cnt,
            no_fix_off
        } = self;
        if *low_power_enable {
            if *periodic_mode != PeriodicMode::Off {
                let fix_period = if *fix_period > 99999 { 99999 } else { *fix_period };
                let fix_on_time = if *fix_on_time > 99 { 99 } else { *fix_on_time };
                let no_fix_cnt = if *no_fix_cnt > 99 { 99 } else { *no_fix_cnt };
                let no_fix_off = if *no_fix_off > 99 { 99 } else { *no_fix_off };
                let periodic_mode = periodic_mode.to_string()?;
                let mut s = format!(
                    "${},1,0,0,0,0,0,0,{},{},{},{},{},{},{}",
                    Self::CMD,
                    periodic_mode,
                    fix_period,
                    fix_on_time,
                    if *ephemeris_refresh { 1 } else { 0 },
                    if *rtc_calibration { 1 } else { 0 },
                    no_fix_cnt,
                    no_fix_off
                ).map_err(|_| ())?;
                self.append_checksum_and_crlf(&mut s)?;
                Ok(s)
            } else {
                let constellation_mask = constellation_mask.to_string()?;
                let ehpe_threshold = if *ehpe_threshold > 999 { 999 } else { *ehpe_threshold };
                let max_tracked_satellites = if *max_tracked_satellites > 99 { 99 } else { *max_tracked_satellites };
                let duty_cycle_fix_period = if *duty_cycle_fix_period > 9 { 9 } else { *duty_cycle_fix_period };
                let mut s = format!(
                    "${},1,{},{},{},{},{},{},0,0,0,0,0,0,0",
                    Self::CMD,
                    constellation_mask.as_str(),
                    ehpe_threshold,
                    max_tracked_satellites,
                    if *switch_constellation_features { 1 } else { 0 },
                    if *duty_cycle_enable { 1 } else { 0 },
                    duty_cycle_fix_period,
                ).map_err(|_| ())?;
                self.append_checksum_and_crlf(&mut s)?;
                Ok(s)
            }
        } else {
            let mut s = format!("${},0", Self::CMD).map_err(|_| ())?;
            self.append_checksum_and_crlf(&mut s)?;
            Ok(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constellation_mask_to_string() {
        let mask = ConstellationMask {
            gps: true,
            glonass: false,
            qzss: true,
            galileo: false,
            beidou: true,
        };
        assert_eq!(mask.to_string().unwrap(), "133"); // 0b10000101
    }

    #[test]
    fn test_low_power_off_command() {
        let cmd = ConfigureLowPowerOnOff {
            low_power_enable: false,
            constellation_mask: ConstellationMask {
                gps: false,
                glonass: false,
                qzss: false,
                galileo: false,
                beidou: false,
            },
            ehpe_threshold: 0,
            max_tracked_satellites: 0,
            switch_constellation_features: false,
            duty_cycle_enable: false,
            duty_cycle_fix_period: 0,
            periodic_mode: PeriodicMode::Off,
            fix_period: 0,
            fix_on_time: 0,
            ephemeris_refresh: false,
            rtc_calibration: false,
            no_fix_cnt: 0,
            no_fix_off: 0,
        };
        assert_eq!(cmd.to_string().unwrap(), "$PSTMLOWPOWERONOFF,0*43\r\n");
    }

    #[test]
    fn test_standby_periodic_mode() {
        let cmd = ConfigureLowPowerOnOff {
            low_power_enable: true,
            constellation_mask: ConstellationMask {
                gps: false,
                glonass: false,
                qzss: false,
                galileo: false,
                beidou: false,
            },
            ehpe_threshold: 0,
            max_tracked_satellites: 0,
            switch_constellation_features: false,
            duty_cycle_enable: false,
            duty_cycle_fix_period: 0,
            periodic_mode: PeriodicMode::Standby,
            fix_period: 100,
            fix_on_time: 10,
            ephemeris_refresh: true,
            rtc_calibration: true,
            no_fix_cnt: 5,
            no_fix_off: 5,
        };
        assert_eq!(cmd.to_string().unwrap(), "$PSTMLOWPOWERONOFF,1,0,0,0,0,0,0,3,100,10,1,1,5,5*6D\r\n");
    }

    #[test]
    fn test_non_periodic_mode() {
        let cmd = ConfigureLowPowerOnOff {
            low_power_enable: true,
            constellation_mask: ConstellationMask {
                gps: true,
                glonass: true,
                qzss: false,
                galileo: false,
                beidou: false,
            },
            ehpe_threshold: 100,
            max_tracked_satellites: 10,
            switch_constellation_features: true,
            duty_cycle_enable: true,
            duty_cycle_fix_period: 5,
            periodic_mode: PeriodicMode::Off,
            fix_period: 0,
            fix_on_time: 0,
            ephemeris_refresh: false,
            rtc_calibration: false,
            no_fix_cnt: 0,
            no_fix_off: 0,
        };
        assert_eq!(cmd.to_string().unwrap(), "$PSTMLOWPOWERONOFF,1,3,100,10,1,1,5,0,0,0,0,0,0,0*68\r\n");
    }
}