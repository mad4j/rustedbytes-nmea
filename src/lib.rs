#![no_std]

//! NMEA 0183 parser library
//!
//! This library provides a `no_std` compatible NMEA 0183 parser for parsing
//! GPS/GNSS data from receivers.

mod message;
mod parser;
mod types;

// Re-export public API
pub use message::{Field, NmeaMessage};
pub use parser::NmeaParser;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_initialization() {
        let parser = NmeaParser::new();
        assert_eq!(parser.buffer_pos(), 0);
        assert_eq!(parser.timestamp_counter(), 0);
    }

    #[test]
    fn test_parse_gga_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::GGA);
        assert!(msg.field_count > 0);
        assert_eq!(msg.timestamp, 1);
    }

    #[test]
    fn test_parse_rmc_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::RMC);
        assert!(msg.field_count > 0);
    }

    #[test]
    fn test_parse_gsa_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::GSA);
    }

    #[test]
    fn test_parse_gsv_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::GSV);
    }

    #[test]
    fn test_parse_gll_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A,*1D\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::GLL);
    }

    #[test]
    fn test_parse_vtg_message() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        assert_eq!(msg.message_type, MessageType::VTG);
    }

    #[test]
    fn test_get_last_message() {
        let mut parser = NmeaParser::new();

        // Parse a GGA message
        let gga_sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        for &c in gga_sentence.iter() {
            parser.parse_char(c);
        }

        // Parse an RMC message
        let rmc_sentence =
            b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";
        for &c in rmc_sentence.iter() {
            parser.parse_char(c);
        }

        // Verify we can retrieve both messages
        let gga_msg = parser.get_last_message(MessageType::GGA);
        assert!(gga_msg.is_some());
        assert_eq!(gga_msg.unwrap().message_type, MessageType::GGA);
        assert_eq!(gga_msg.unwrap().timestamp, 1);

        let rmc_msg = parser.get_last_message(MessageType::RMC);
        assert!(rmc_msg.is_some());
        assert_eq!(rmc_msg.unwrap().message_type, MessageType::RMC);
        assert_eq!(rmc_msg.unwrap().timestamp, 2);

        // Verify we get None for message types we haven't parsed
        let gsa_msg = parser.get_last_message(MessageType::GSA);
        assert!(gsa_msg.is_none());
    }

    #[test]
    fn test_multiple_messages_stream() {
        let mut parser = NmeaParser::new();
        let stream = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n\
                       $GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n\
                       $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let mut message_count = 0;
        for &c in stream.iter() {
            if parser.parse_char(c).is_some() {
                message_count += 1;
            }
        }

        assert_eq!(message_count, 3);
    }

    #[test]
    fn test_field_extraction() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();

        // Check first field (sentence ID)
        assert!(msg.fields[0].is_some());
        let field0 = msg.fields[0].as_ref().unwrap();
        assert_eq!(field0.as_str(), Some("GPGGA"));

        // Check time field
        assert!(msg.fields[1].is_some());
        let field1 = msg.fields[1].as_ref().unwrap();
        assert_eq!(field1.as_str(), Some("123519"));
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        for &c in sentence.iter() {
            parser.parse_char(c);
        }

        assert_eq!(parser.timestamp_counter(), 1);

        parser.reset();
        assert_eq!(parser.timestamp_counter(), 0);
        assert_eq!(parser.buffer_pos(), 0);
    }

    #[test]
    fn test_invalid_sentence() {
        let mut parser = NmeaParser::new();
        let invalid = b"INVALID DATA\r\n";

        let mut result = None;
        for &c in invalid.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_none());
    }

    #[test]
    fn test_partial_sentence() {
        let mut parser = NmeaParser::new();
        let partial = b"$GPGGA,123519,4807.038,N";

        let mut result = None;
        for &c in partial.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        // Should not complete without \r\n
        assert!(result.is_none());
    }

    #[test]
    fn test_message_overwrite() {
        let mut parser = NmeaParser::new();

        // Parse first GGA message
        let gga1 = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        for &c in gga1.iter() {
            parser.parse_char(c);
        }

        let first_msg = parser.get_last_message(MessageType::GGA);
        assert_eq!(first_msg.unwrap().timestamp, 1);

        // Parse second GGA message
        let gga2 = b"$GPGGA,133519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
        for &c in gga2.iter() {
            parser.parse_char(c);
        }

        let second_msg = parser.get_last_message(MessageType::GGA);
        assert_eq!(second_msg.unwrap().timestamp, 2);
    }

    #[test]
    fn test_gga_parameters() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();
        assert_eq!(gga_data.time, "123519");
        assert_eq!(gga_data.latitude, 4807.038);
        assert_eq!(gga_data.lat_direction, 'N');
        assert_eq!(gga_data.longitude, 1131.000);
        assert_eq!(gga_data.lon_direction, 'E');
        assert_eq!(gga_data.fix_quality, 1);
        assert_eq!(gga_data.num_satellites, Some(8));
        assert_eq!(gga_data.hdop, Some(0.9));
        assert_eq!(gga_data.altitude, Some(545.4));
        assert_eq!(gga_data.altitude_units, Some('M'));
    }

    #[test]
    fn test_rmc_parameters() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.time, "123519");
        assert_eq!(rmc_data.status, 'A');
        assert_eq!(rmc_data.latitude, 4807.038);
        assert_eq!(rmc_data.lat_direction, 'N');
        assert_eq!(rmc_data.longitude, 1131.000);
        assert_eq!(rmc_data.lon_direction, 'E');
        assert_eq!(rmc_data.speed_knots, 22.4);
        assert_eq!(rmc_data.track_angle, 84.4);
        assert_eq!(rmc_data.date, "230394");
    }

    #[test]
    fn test_gsa_parameters() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.mode, 'A');
        assert_eq!(gsa_data.fix_type, 3);
        assert_eq!(gsa_data.satellite_ids[0], Some(4));
        assert_eq!(gsa_data.satellite_ids[1], Some(5));
        assert_eq!(gsa_data.satellite_ids[3], Some(9));
        assert_eq!(gsa_data.pdop, Some(2.5));
        assert_eq!(gsa_data.hdop, Some(1.3));
        assert_eq!(gsa_data.vdop, Some(2.1));
    }

    #[test]
    fn test_gsv_parameters() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.num_messages, 2);
        assert_eq!(gsv_data.message_num, 1);
        assert_eq!(gsv_data.satellites_in_view, 8);

        // Check first satellite
        assert!(gsv_data.satellite_info[0].is_some());
        let sat1 = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat1.prn, Some(1));
        assert_eq!(sat1.elevation, Some(40));
        assert_eq!(sat1.azimuth, Some(83));
        assert_eq!(sat1.snr, Some(46));
    }

    #[test]
    fn test_gll_parameters() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,A,*1D\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        assert!(gll.is_some());

        let gll_data = gll.unwrap();
        assert_eq!(gll_data.latitude, 4916.45);
        assert_eq!(gll_data.lat_direction, 'N');
        assert_eq!(gll_data.longitude, 12311.12);
        assert_eq!(gll_data.lon_direction, 'W');
        assert_eq!(gll_data.time, "225444");
        assert_eq!(gll_data.status, 'A');
    }

    #[test]
    fn test_vtg_parameters() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let vtg = msg.as_vtg();
        assert!(vtg.is_some());

        let vtg_data = vtg.unwrap();
        assert_eq!(vtg_data.track_true, Some(54.7));
        assert_eq!(vtg_data.track_true_indicator, Some('T'));
        assert_eq!(vtg_data.track_magnetic, Some(34.4));
        assert_eq!(vtg_data.track_magnetic_indicator, Some('M'));
        assert_eq!(vtg_data.speed_knots, Some(5.5));
        assert_eq!(vtg_data.speed_knots_indicator, Some('N'));
        assert_eq!(vtg_data.speed_kph, Some(10.2));
        assert_eq!(vtg_data.speed_kph_indicator, Some('K'));
    }

    #[test]
    fn test_wrong_message_type_extraction() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();

        // Try to extract RMC data from a GGA message
        let rmc = msg.as_rmc();
        assert!(rmc.is_none());

        // GGA extraction should work
        let gga = msg.as_gga();
        assert!(gga.is_some());
    }

    #[test]
    fn test_gga_with_empty_fields() {
        let mut parser = NmeaParser::new();
        // GGA message with some empty mandatory fields should fail to parse
        let sentence = b"$GPGGA,123519,,N,,E,1,,,,,M,,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        // Should return None because mandatory fields (latitude, longitude) are empty
        assert!(gga.is_none());
    }

    #[test]
    fn test_rmc_with_empty_status() {
        let mut parser = NmeaParser::new();
        // RMC message with void status (still valid)
        let sentence = b"$GPRMC,123519,V,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        assert!(rmc.is_some());

        let rmc_data = rmc.unwrap();
        assert_eq!(rmc_data.status, 'V');
    }

    #[test]
    fn test_gsa_with_partial_satellites() {
        let mut parser = NmeaParser::new();
        // GSA message with only a few satellites
        let sentence = b"$GPGSA,A,3,01,,,,,,,,,,,,2.5,1.3,2.1*39\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        assert!(gsa.is_some());

        let gsa_data = gsa.unwrap();
        assert_eq!(gsa_data.mode, 'A');
        assert_eq!(gsa_data.fix_type, 3);
        assert_eq!(gsa_data.satellite_ids[0], Some(1));
        assert_eq!(gsa_data.satellite_ids[1], None);
        assert_eq!(gsa_data.pdop, Some(2.5));
        assert_eq!(gsa_data.hdop, Some(1.3));
        assert_eq!(gsa_data.vdop, Some(2.1));
    }

    #[test]
    fn test_gsv_with_partial_satellite_data() {
        let mut parser = NmeaParser::new();
        // GSV message with only two satellites
        let sentence = b"$GPGSV,1,1,02,01,40,083,46,02,17,308,*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        assert!(gsv.is_some());

        let gsv_data = gsv.unwrap();
        assert_eq!(gsv_data.num_messages, 1);
        assert_eq!(gsv_data.satellites_in_view, 2);

        // First satellite should be complete
        assert!(gsv_data.satellite_info[0].is_some());
        let sat1 = gsv_data.satellite_info[0].as_ref().unwrap();
        assert_eq!(sat1.prn, Some(1));
        assert_eq!(sat1.elevation, Some(40));
        assert_eq!(sat1.azimuth, Some(83));
        assert_eq!(sat1.snr, Some(46));

        // Second satellite should have missing SNR
        assert!(gsv_data.satellite_info[1].is_some());
        let sat2 = gsv_data.satellite_info[1].as_ref().unwrap();
        assert_eq!(sat2.prn, Some(2));
        assert_eq!(sat2.elevation, Some(17));
        assert_eq!(sat2.azimuth, Some(308));
        assert_eq!(sat2.snr, None);

        // Third and fourth should be None
        assert!(gsv_data.satellite_info[2].is_none());
        assert!(gsv_data.satellite_info[3].is_none());
    }

    #[test]
    fn test_numeric_type_parsing() {
        let mut parser = NmeaParser::new();
        let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        assert!(gga.is_some());

        let gga_data = gga.unwrap();

        // Verify types are correctly parsed
        assert!((gga_data.latitude - 4807.038).abs() < 0.001);
        assert!((gga_data.longitude - 1131.000).abs() < 0.001);

        if let Some(hdop) = gga_data.hdop {
            assert!((hdop - 0.9).abs() < 0.01);
        }

        if let Some(alt) = gga_data.altitude {
            assert!((alt - 545.4).abs() < 0.1);
        }
    }

    #[test]
    fn test_gga_missing_time() {
        let mut parser = NmeaParser::new();
        // GGA message without time (mandatory field)
        let sentence = b"$GPGGA,,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gga = msg.as_gga();
        // Should return None because time is mandatory
        assert!(gga.is_none());
    }

    #[test]
    fn test_rmc_missing_mandatory_field() {
        let mut parser = NmeaParser::new();
        // RMC message without date (mandatory field)
        let sentence = b"$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,,003.1,W*6A\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let rmc = msg.as_rmc();
        // Should return None because date is mandatory
        assert!(rmc.is_none());
    }

    #[test]
    fn test_gsa_missing_mode() {
        let mut parser = NmeaParser::new();
        // GSA message without mode (mandatory field)
        let sentence = b"$GPGSA,,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsa = msg.as_gsa();
        // Should return None because mode is mandatory
        assert!(gsa.is_none());
    }

    #[test]
    fn test_gsv_missing_mandatory_field() {
        let mut parser = NmeaParser::new();
        // GSV message without num_messages (mandatory field)
        let sentence = b"$GPGSV,,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gsv = msg.as_gsv();
        // Should return None because num_messages is mandatory
        assert!(gsv.is_none());
    }

    #[test]
    fn test_gll_missing_status() {
        let mut parser = NmeaParser::new();
        // GLL message without status (mandatory field)
        let sentence = b"$GPGLL,4916.45,N,12311.12,W,225444,,*1D\r\n";

        let mut result = None;
        for &c in sentence.iter() {
            if let Some(msg) = parser.parse_char(c) {
                result = Some(msg);
            }
        }

        assert!(result.is_some());
        let msg = result.unwrap();
        let gll = msg.as_gll();
        // Should return None because status is mandatory
        assert!(gll.is_none());
    }
}
