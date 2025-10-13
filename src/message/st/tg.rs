//! $PSTMTG
//! Time and Satellites Information
//!
//! | **Parameter**          | **Format**              | **Description** |
//! | :--------------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
//! | `Week`                 | Decimal, 4 digits       | Week Number                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
//! | `TOW`                  | Decimal, 10 digits      | Time of Week                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
//! | `Tot-Sat`              | Decimal, 2 digits       | Total Number of satellites used for fix                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
//! | `CPU-Time`             | Decimal, 10 digits      | CPU Time                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
//! | `Timevalid`            | Decimal, 2 digits       | 0 = no time <br>1 = time read from flash <br>2 = time set by user <br>3 = time set user RTC <br>4 = RTC time <br>5 = RTC time, accurate <br>6 = time approximate <br>7 = "not used" <br>8 = time accurate <br>9 = position time <br>10 = Ephemeris time  |
//! | `NCO`                  | Decimal, 9 digits       | NCO value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
//! | `kf_config_status`     | Hexadecimal, 2 digits   | Kalman Filter Configuration. <br>For each bit: <br>• 0 means feature disabled <br>• 1 means feature enabled <br>See Table 141.                                                                                                                                                                                                                                                                                                                                                      |
//! | `constellation_mask`   | Decimal, 3 digits max   | It is a bit mask where each bit enables/disables a specific constellation independently of the others: <br>bit 0: GPS constellation enabling/disabling <br>bit 1: GLONASS constellation enabling/disabling <br>bit 2: QZSS constellation enabling/disabling <br>bit 3: GALILELO constellation enabling/disabling <br>bit 7: BAIDEU constellation enabling/disabling                                                                                                |
//! | `time_best_sat_type`   | Decimal                 | Selected best time satellite type                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
//! | `time_master_sat_type` | Decimal                 | Master time satellite type                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
//! | `time_aux_sat_type`    | Decimal                 | Auxiliary time satellite type                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
//! | `time_master_week_n`   | Decimal                 | Master time week number                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
//! | `time_master_tow`      | Floating                | Master time TOW                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
//! | `time_master_validity` | Decimal                 | Master week number time validity                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
//! | `time_aux_week_n`      | Decimal                 | Auxiliary time                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
//! | `time_aux_tow`         | Floating                | Auxiliary time TOW                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
//! | `time_aux_validity`    | Decimal                 | Auxiliary time validity                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
//!
//! ---
//!
//! ## Table 141: $PSTMTG Kalman Filter Configuration
//!
//! | **Bit** | **Configuration** |
//! | :------ | :---------------------------------------------------------------------------------- |
//! | 0       | Walking mode ON                                                                     |
//! | 1       | Stop Detection ON                                                                   |
//! | 2       | Frequency Ramp On (only Xtal mode)                                                  |
//! | 3       | Velocity estimator model: <br>• 1 means MULTIPLE MODEL <br>• 0 means SINGLE MODEL   |
//! | 4       | Velocity estimator filter: <br>• 1 means SLOW <br>• 0 means FAST                    |
//! | 5       | FDE Status ON                                                                       |

//! $PSTMTG,<Week>,<TOW>,<TotSat>,<CPUTime><Timevalid><NCO><kf_config_status><constellation_mask>
//! <time_best_sat_type><time_master_sat_type><time_aux_sat_type><time_master_week_n><time_master
//! _tow><time_master_validity><time_aux_week_n><time_aux_tow><time_aux_validity>*<checksum><cr><lf>
use crate::message::ParsedSentence;

#[derive(Debug, Clone)]
pub enum KalmanVelocityEstimatorModel {
    SingleModel,
    MultipleModel,
}

#[derive(Debug, Clone)]
pub enum KalmanVelocityEstimatorFilter {
    Slow,
    Fast,
}

#[derive(Debug, Clone)]
pub struct KalmanFilterConfiguration {
    pub walking_mode: bool,
    pub stop_detection: bool,
    pub frequency_ramp_on: bool,
    pub velocity_estimator_model: KalmanVelocityEstimatorModel,
    pub velocity_estimator_filter: KalmanVelocityEstimatorFilter,
    pub fde_status: bool,
}

impl From<u8> for KalmanFilterConfiguration {
    fn from(kf_config_status: u8) -> Self {
        Self {
            walking_mode: (kf_config_status & 0b0000_0001) != 0,
            stop_detection: (kf_config_status & 0b0000_0010) != 0,
            frequency_ramp_on: (kf_config_status & 0b0000_0100) != 0,
            velocity_estimator_model: if (kf_config_status & 0b0000_1000) != 0 {
                KalmanVelocityEstimatorModel::MultipleModel
            } else {
                KalmanVelocityEstimatorModel::SingleModel
            },
            velocity_estimator_filter: if (kf_config_status & 0b0001_0000) != 0 {
                KalmanVelocityEstimatorFilter::Fast
            } else {
                KalmanVelocityEstimatorFilter::Slow
            },
            fde_status: (kf_config_status & 0b0010_0000) != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeAndSatelliteInformation {
    pub week: u16,
    pub tow: u32,
    pub total_satellites: u8,
    pub cpu_time: u32,
    pub time_valid: u8,
    pub nco: u32,
    pub kf_config_status: KalmanFilterConfiguration,
    pub constellation_mask: u8,
    pub time_best_sat_type: u8,
    pub time_master_sat_type: u8,
    pub time_aux_sat_type: u8,
    pub time_master_week_n: u16,
    pub time_master_tow: f32,
    pub time_master_validity: u8,
    pub time_aux_week_n: u16,
    pub time_aux_tow: f32,
    pub time_aux_validity: u8,
}

impl TimeAndSatelliteInformation {
    pub(crate) fn parse(sentence: &ParsedSentence) -> Option<Self> {
        let week = sentence.parse_field::<u16>(1)?;
        let tow = sentence.parse_field::<u32>(2)?;
        let total_satellites = sentence.parse_field::<u8>(3)?;
        let cpu_time = sentence.parse_field::<u32>(4)?;
        let time_valid = sentence.parse_field::<u8>(5)?;
        let nco = sentence.parse_field::<u32>(6)?;
        let kf_config_status = sentence.parse_hex_field::<u8>(7)?;
        let constellation_mask = sentence.parse_field::<u8>(8)?;
        let time_best_sat_type = sentence.parse_field::<u8>(9)?;
        let time_master_sat_type = sentence.parse_field::<u8>(10)?;
        let time_aux_sat_type = sentence.parse_field::<u8>(11)?;
        let time_master_week_n = sentence.parse_field::<u16>(12)?;
        let time_master_tow = sentence.parse_field::<f32>(13)?;
        let time_master_validity = sentence.parse_field::<u8>(14)?;
        let time_aux_week_n = sentence.parse_field::<u16>(15)?;
        let time_aux_tow = sentence.parse_field::<f32>(16)?;
        let time_aux_validity = sentence.parse_field::<u8>(17)?;

        let kf_config_status = KalmanFilterConfiguration {
            walking_mode: (kf_config_status & 0b0000_0001) != 0,
            stop_detection: (kf_config_status & 0b0000_0010) != 0,
            frequency_ramp_on: (kf_config_status & 0b0000_0100) != 0,
            velocity_estimator_model: if (kf_config_status & 0b0000_1000) != 0 {
                KalmanVelocityEstimatorModel::MultipleModel
            } else {
                KalmanVelocityEstimatorModel::SingleModel
            },
            velocity_estimator_filter: if (kf_config_status & 0b0001_0000) != 0 {
                KalmanVelocityEstimatorFilter::Fast
            } else {
                KalmanVelocityEstimatorFilter::Slow
            },
            fde_status: (kf_config_status & 0b0010_0000) != 0,
        };

        Some(TimeAndSatelliteInformation {
            week,
            tow,
            total_satellites,
            cpu_time,
            time_valid,
            nco,
            kf_config_status,
            constellation_mask,
            time_best_sat_type,
            time_master_sat_type,
            time_aux_sat_type,
            time_master_week_n,
            time_master_tow,
            time_master_validity,
            time_aux_week_n,
            time_aux_tow,
            time_aux_validity,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_kf_config_status() {
        // Test case for kf_config_status = 0x21 (00100001)
        let kf_config_status_val = 0x21;
        let kf_config = KalmanFilterConfiguration::from(kf_config_status_val);

        assert!(kf_config.walking_mode); // Bit 0 is 1
        assert!(!kf_config.stop_detection); // Bit 1 is 0
        assert!(!kf_config.frequency_ramp_on); // Bit 2 is 0
        assert!(matches!(
            kf_config.velocity_estimator_model,
            KalmanVelocityEstimatorModel::SingleModel
        )); // Bit 3 is 0
        assert!(!matches!(
            kf_config.velocity_estimator_filter,
            KalmanVelocityEstimatorFilter::Fast
        )); // Bit 4 is 0
        assert!(kf_config.fde_status); // Bit 5 is 1

        // Test case for kf_config_status = 0x1E (00011110)
        let kf_config_status_val = 0x1E;
        let kf_config = KalmanFilterConfiguration::from(kf_config_status_val);

        assert!(!kf_config.walking_mode); // Bit 0 is 0
        assert!(kf_config.stop_detection); // Bit 1 is 1
        assert!(kf_config.frequency_ramp_on); // Bit 2 is 1
        assert!(matches!(
            kf_config.velocity_estimator_model,
            KalmanVelocityEstimatorModel::MultipleModel
        )); // Bit 3 is 1
        assert!(matches!(
            kf_config.velocity_estimator_filter,
            KalmanVelocityEstimatorFilter::Fast
        )); // Bit 4 is 1
        assert!(!kf_config.fde_status); // Bit 5 is 0

        // Test case for kf_config_status = 0x00 (00000000)
        let kf_config_status_val = 0x00;
        let kf_config = KalmanFilterConfiguration::from(kf_config_status_val);

        assert!(!kf_config.walking_mode);
        assert!(!kf_config.stop_detection);
        assert!(!kf_config.frequency_ramp_on);
        assert!(matches!(
            kf_config.velocity_estimator_model,
            KalmanVelocityEstimatorModel::SingleModel
        ));
        assert!(matches!(
            kf_config.velocity_estimator_filter,
            KalmanVelocityEstimatorFilter::Slow
        ));
        assert!(!kf_config.fde_status);
    }

    #[test]
    fn test_parse_pstmtg() {
        let parser = NmeaParser::new();
        let (msg, _size) = parser
            .parse_bytes(b"$PSTMTG,2267,395020000,12,1234567890,8,123456789,21,15,1,2,3,2267,395020.000,1,2267,395020.000,1*0C\r\n")
            .unwrap();
        let msg = match msg.unwrap() {
            crate::NmeaMessage::StPropriety(crate::message::StMessageData::TimeAndSatelliteInformation(msg)) => msg,
            _ => panic!("Unexpected message type"),
        };

        assert_eq!(msg.week, 2267);
        assert_eq!(msg.tow, 395020000);
        assert_eq!(msg.total_satellites, 12);
        assert_eq!(msg.cpu_time, 1234567890);
        assert_eq!(msg.time_valid, 8);
        assert_eq!(msg.nco, 123456789);
        assert_eq!(msg.constellation_mask, 15);
        assert_eq!(msg.time_best_sat_type, 1);
    }
}