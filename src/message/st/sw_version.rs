//! $PSTMVER
//! Message sent in response to command $PSTMGETSWVER
//! Parameter               Format          Description
//! Lib                     Text, fixed     Text String identifying the Library that the command is requiring the version:
//!                                             GNSSLIB if type = 0
//!                                             OS20LIB if type = 1
//!                                             GPSAPP if type = 2
//!                                             BINIMG if type = 6
//!                                             SWCG if type = 11
//!                                             PID if type = 12
//! Ver                     X.X.X.X         GNSS Library Version: example 7.1.1.15
//! Type                    ARM, GNU        Compiler Type: ARM or GNU
//!
//! $PSTMVER,<Lib>_<Ver>_<Type>*<checksum>‹cr><lf>

use crate::message::ParsedSentence;
use core::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Library {
    GnssLib,
    Os20Lib,
    GpsApp,
    BinImg,
    SwCg,
    Pid,
}

impl FromStr for Library {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GNSSLIB" => Ok(Self::GnssLib),
            "OS20LIB" => Ok(Self::Os20Lib),
            "GPSAPP" => Ok(Self::GpsApp),
            "BINIMG" => Ok(Self::BinImg),
            "SWCG" => Ok(Self::SwCg),
            "PID" => Ok(Self::Pid),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CompilerType {
    Arm,
    Gnu,
}

impl FromStr for CompilerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "ARM" => Ok(Self::Arm),
            "GNU" => Ok(Self::Gnu),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u8,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let minor = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let patch = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let build = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SoftwareVersion {
    pub library: Library,
    pub version: Version,
    pub compiler_type: CompilerType,
}

impl SoftwareVersion {
    pub(crate) fn parse(sentence: &ParsedSentence) -> Option<Self> {
        let val = sentence.get_field_str(1)?;
        let mut s = val.split("_");
        let library = s.next()?.parse().ok()?;
        let version = s.next()?.parse().ok()?;
        let compiler_type = s.next()?.parse().ok()?;

        Some(SoftwareVersion {
            library,
            version,
            compiler_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::st::sw_version::{CompilerType, Library};
    use crate::message::StMessageData;
    use crate::{NmeaMessage, NmeaParser};

    #[test]
    fn test_sw_version() {
        let parser = NmeaParser::new();
        let (msg, _size) = parser
            .parse_bytes(b"$PSTMVER,GNSSLIB_7.1.1.15_ARM*0A\r\n")
            .unwrap();
        let msg = msg.unwrap();
        if let NmeaMessage::StPropriety(StMessageData::SoftwareVersion(sw_version)) = msg {
            assert_eq!(sw_version.library, Library::GnssLib);
            assert_eq!(sw_version.version.major, 7);
            assert_eq!(sw_version.version.minor, 1);
            assert_eq!(sw_version.version.patch, 1);
            assert_eq!(sw_version.version.build, 15);
            assert_eq!(sw_version.compiler_type, CompilerType::Arm);
        } else {
            panic!("Unexpected message type");
        }
    }
}
