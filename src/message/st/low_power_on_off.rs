//! $PSTMLOWPOWERON
//! Message sent in response of command $PSTMLOWPOWERONOFF
//!
//! Parameter                       Format              Description
//!
//! Adaptive mode settings
//! EHPE threshold                  Decimal, 3 digits   EHPE average threshold [m]
//! Max tracked sats                Decimal, 2 digits   first N satellites (with higher elevation) used
//!                                                         for the position calculation (Active channel management) in LOW POWER STATE
//! Switch constellation features   Decimal, 1 digits   Switch constellation features (enable it only for GNSS constellation case)
//!
//! Cyclic mode settings
//! Duty Cycle enable/disable       Decimal, 1 digits   Duty Cycle features enable/disable
//! Duty Cycle ms signal off        Decimal, 3 digits   Estimated Horizontal Position Error Average
//!
//! Periodic mode settings
//! Periodic mode                   Decimal, 1 digit    Setup Active or Standby periodic mode
//!                                                         0: OFF
//!                                                         1: Active Periodic mode
//!                                                         3: Standby Periodic mode
//! FixPeriod                       Decimal, 5 digits   Interval between two fixes [s]
//! FixOnTime                       Decimal, 2 digits   Number of fixes reported for each interval
//! Ephemeris refresh               Decimal, 1 digit    Enable/Disable the refresh of ephemeris data
//!                                                         O: OFF, 1: ON
//! RTC calibration                 Decimal, 1 digit    Enable/Disable the RTC calibration
//!                                                         0: OFF, 1: ON
//! NoFixCnt                        Decimal, 2 digits   Time to declare fix loss [s] in HOT conditions
//! NoFixOff                        Decimal, 2 digits   Period of off period after a fix loss [s]
//!
//! $PSTMLOWPOWERON, ‹EHPE_threshold›, ‹Max_tracked_sats>, <Switch_constellation_features>, <Duty_Cycle_enable>,
//! ‹Duty_Cycle_fix_period›,‹Periodic_mode›, ‹Fix_period›, ‹Number_of_fix>, <Ephemeris_re fresh>,
//! <RTC_refresh›, ‹No_Fix_timeout>, <No_Fix_timeout_0ff_duration›*<checksum›<cr><lf>

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PeriodicMode {
    Off,
    Active,
    Standby,
}

impl TryFrom<u8> for PeriodicMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PeriodicMode::Off),
            1 => Ok(PeriodicMode::Active),
            3 => Ok(PeriodicMode::Standby),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LowPowerOnOff {
    pub ehpe_threshold: u16,
    pub max_tracked_sats: u8,
    pub switch_constellation_features: u8,
    pub duty_cycle_enable: bool,
    pub duty_cycle_ms_signal_off: bool,
    pub periodic_mode: PeriodicMode,
    pub fix_period: u32,
    pub fix_on_time: u8,
    pub ephemeris_refresh: bool,
    pub rtc_calibration: bool,
    pub no_fix_cnt: u8,
    pub no_fix_off: u8,
}

impl LowPowerOnOff {
    pub(crate) fn parse(sentence: &crate::message::ParsedSentence) -> Option<Self> {
        let ehpe_threshold = sentence.parse_field(1)?;
        let max_tracked_sats = sentence.parse_field(2)?;
        let switch_constellation_features = sentence.parse_field(3)?;
        let duty_cycle_enable = sentence.parse_field(4)?;
        let duty_cycle_ms_signal_off = sentence.parse_field(5)?;
        let periodic_mode = sentence
            .parse_field::<u8>(6)
            .and_then(|v| PeriodicMode::try_from(v).ok())?;
        let fix_period = sentence.parse_field(7)?;
        let fix_on_time = sentence.parse_field(8)?;
        let ephemeris_refresh = sentence.parse_field(9)?;
        let rtc_calibration = sentence.parse_field(10)?;
        let no_fix_cnt = sentence.parse_field(11)?;
        let no_fix_off = sentence.parse_field(12)?;

        Some(Self {
            ehpe_threshold,
            max_tracked_sats,
            switch_constellation_features,
            duty_cycle_enable,
            duty_cycle_ms_signal_off,
            periodic_mode,
            fix_period,
            fix_on_time,
            ephemeris_refresh,
            rtc_calibration,
            no_fix_cnt,
            no_fix_off,
        })
    }
}
