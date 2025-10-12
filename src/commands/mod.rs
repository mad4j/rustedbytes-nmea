use heapless::{format, String};

#[cfg(feature = "st-teseo-liv3")]
mod st;

#[cfg(feature = "st-teseo-liv3")]
pub use st::{
    lpa::{ConfigureLowPowerAlgorithm, LowPowerAlgorithmFeature},
    odometer::ConfigureOdometer
};

/// Generate NMEA checksum for a sentence
pub(crate) fn nmea_checksum(sentence: &str) -> u8 {
    // Strip leading '$' and trailing checksum (if any)
    let trimmed = sentence.trim_start_matches('$')
        .split('*')
        .next()
        .unwrap_or(sentence);

    // XOR all characters
    trimmed.bytes().fold(0u8, |acc, b| acc ^ b)
}

pub trait Command {
    const MAX_LEN: usize;
    const CMD: &'static str;

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, ()>;

    fn append_checksum_and_crlf(&self, val: &mut String<{ Self::MAX_LEN }>) -> Result<(), ()> {
        let checksum = nmea_checksum(val.as_str());
        val.push_str("*").map_err(|_| ())?;
        let checksum: String<2> = format!("{:X}", checksum).map_err(|_| ())?; // Max 0xFF, so 2 characters should be enough
        val.push_str(checksum.as_str()).map_err(|_| ())?;
        val.push_str("\r\n").map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nmea_checksum() {
        [
            ("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,", 71),
            ("GPRMC,235947,A,5540.505,N,03736.280,E,000.0,360.0,130998,011.3,E", 123),
            ("$GPGSA,A,3,04,05,09,12,24,25,29,31,32,,,1.8,1.0,1.5*00",16),
            ("", 0),
        ].into_iter().for_each(|(input, checksum)| {
            assert_eq!(super::nmea_checksum(input), checksum);
        })
    }
}