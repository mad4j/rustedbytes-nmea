//! $PSTMGETSWVER
//! Get the version string of the libraries embedded in the software application.
//! Parameter       Format              Description
//! id              Integer             Depending on the value of the <lib_id> parameter, the following
//!                                         version numbering is delivered by the command:
//!                                         0 = GNSS Library Version
//!                                         1 = OS20 Version
//!                                         2 = SDK App Version
//!                                         6 = Binary Image Version
//!                                         7 = STA8088 HW version
//!                                         11 = SW configuration ID
//!                                         12 = Product ID
//!                                         254 = configuration data block
//!                                         255 = all versions strings (as reported at the NMEA startup).
//!
//! $PSTMGETSWVER,<id>*<checksum><cr><lf>

use crate::{Command, CommandError};
use heapless::{format, String};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum LibraryId {
    GnssLibrary = 0,
    Os20 = 1,
    SdkApp = 2,
    BinaryImage = 6,
    Sta8088Hw = 7,
    SwConfigId = 11,
    ProductId = 12,
    ConfigData = 254,
    AllVersions = 255,
}

#[derive(Debug, Clone)]
pub struct GetSoftwareVersion {
    pub lib_id: LibraryId,
}

impl Command for GetSoftwareVersion {
    const MAX_LEN: usize = 20 + 4;
    const CMD: &'static str = "PSTMGETSWVER";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, CommandError> {
        let mut s = format!("${},{}", Self::CMD, self.lib_id as u8)?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_software_version() {
        let cmd = GetSoftwareVersion {
            lib_id: LibraryId::AllVersions,
        };
        assert_eq!(cmd.to_string().unwrap(), "$PSTMGETSWVER,255*17\r\n");
    }
}
