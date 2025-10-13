use heapless::{format, CapacityError, String};

#[cfg(feature = "st-teseo-liv3")]
mod st;

#[cfg(feature = "st-teseo-liv3")]
pub use st::{
    anti_jam::{ConfigureAntiJamming, NotchFilterMode},
    geofence::{ConfigureEnableGeofenceCircles, GeofenceToleranceLevel},
    geofence_circle::ConfigureGeofenceCircle,
    get_sw_version::GetSoftwareVersion,
    get_unique_code::GetUniqueCode,
    low_power_on_off::{ConfigureLowPowerOnOff, ConstellationMask},
    lpa::{ConfigureLowPowerAlgorithm, LowPowerAlgorithmFeature},
    odometer::ConfigureOdometer,
    standby_enable::{ConfigureStandbyEnable, StandbyEnableCheckStatus},
    standby_force::ConfigureStandbyForce,
};

/// Generate NMEA checksum for a sentence
pub(crate) fn nmea_checksum(sentence: &str) -> u8 {
    // Strip leading '$' and trailing checksum (if any)
    let trimmed = sentence
        .trim_start_matches('$')
        .split('*')
        .next()
        .unwrap_or(sentence);

    // XOR all characters
    trimmed.bytes().fold(0u8, |acc, b| acc ^ b)
}

#[derive(Debug)]
pub enum CommandError {
    CapacityError,
    FormatError,
}

impl From<CapacityError> for CommandError {
    fn from(_value: CapacityError) -> Self {
        CommandError::CapacityError
    }
}

impl From<core::fmt::Error> for CommandError {
    fn from(_value: core::fmt::Error) -> Self {
        CommandError::FormatError
    }
}

pub trait Command {
    const MAX_LEN: usize;
    const CMD: &'static str;

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, CommandError>;

    fn append_checksum_and_crlf(
        &self,
        val: &mut String<{ Self::MAX_LEN }>,
    ) -> Result<(), CommandError> {
        let checksum = nmea_checksum(val.as_str());
        val.push_str("*")?;
        let checksum: String<2> = format!("{:X}", checksum)?; // Max 0xFF, so 2 characters should be enough
        val.push_str(checksum.as_str())?;
        val.push_str("\r\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use heapless::String;
    use crate::CommandError;

    #[test]
    fn test_nmea_checksum() {
        [
            (
                "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,",
                71,
            ),
            (
                "GPRMC,235947,A,5540.505,N,03736.280,E,000.0,360.0,130998,011.3,E",
                123,
            ),
            ("$GPGSA,A,3,04,05,09,12,24,25,29,31,32,,,1.8,1.0,1.5*00", 16),
            ("", 0),
        ]
        .into_iter()
        .for_each(|(input, checksum)| {
            assert_eq!(super::nmea_checksum(input), checksum);
        })
    }

    #[test]
    fn convert_from_errors() {
        let mut s = String::<2>::new();
        let err = s.push_str("Not going to be pushed").unwrap_err();
        let ce: CommandError = err.into();
        assert!(matches!(ce, CommandError::CapacityError));

        let ce: CommandError = core::fmt::Error.into();
        assert!(matches!(ce, CommandError::FormatError));

    }
}
