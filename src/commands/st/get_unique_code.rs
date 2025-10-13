//! $PSTMGETUCODE
//!
//! This command reads the unique code from the secondary boot flash memory partition.
//!
//! $PSTMGETUCODE*<checksum><cr><lf>

use crate::{Command, CommandError};
use heapless::{format, String};

#[derive(Debug, Clone)]
pub struct GetUniqueCode {}

impl Command for GetUniqueCode {
    const MAX_LEN: usize = 18;
    const CMD: &'static str = "PSTMGETUCODE";

    fn to_string(&self) -> Result<String<{ Self::MAX_LEN }>, CommandError> {
        let mut s = format!("${}", Self::CMD)?;
        self.append_checksum_and_crlf(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_code_to_string() {
        let get_unique_code = GetUniqueCode {};
        let expected_string = "$PSTMGETUCODE*14\r\n";
        let actual_string = get_unique_code.to_string().unwrap();
        assert_eq!(actual_string, expected_string);
    }
}
