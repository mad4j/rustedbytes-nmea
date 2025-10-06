# rustedbytes-nmea

[![Crates.io](https://img.shields.io/crates/v/rustedbytes-nmea.svg)](https://crates.io/crates/rustedbytes-nmea)
[![Documentation](https://docs.rs/rustedbytes-nmea/badge.svg)](https://docs.rs/rustedbytes-nmea)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Downloads](https://img.shields.io/crates/d/rustedbytes-nmea.svg)](https://crates.io/crates/rustedbytes-nmea)

Rust `no_std` library for parsing NMEA messages from a GNSS receiver.

## Features

- `no_std` compatible - can be used in embedded systems
- Character stream parsing - feed characters one at a time
- Supports common NMEA message types:
  - GGA (Global Positioning System Fix Data)
  - RMC (Recommended Minimum Navigation Information)
  - GSA (GPS DOP and active satellites)
  - GSV (GPS Satellites in view)
  - GLL (Geographic Position - Latitude/Longitude)
  - VTG (Track Made Good and Ground Speed)
- Message storage with timestamp tracking
- Query last received message by type
- Structured parameter extraction for each message type

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustedbytes-nmea = "0.1.0"
```

### Basic Example

```rust
use rustedbytes_nmea::{NmeaParser, MessageType};

fn main() {
    let mut parser = NmeaParser::new();
    
    // NMEA sentence as bytes
    let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
    
    // Parse character by character
    for &byte in sentence.iter() {
        if let Some(message) = parser.parse_char(byte) {
            println!("Received message type: {:?}", message.message_type);
            println!("Timestamp: {}", message.timestamp);
            println!("Field count: {}", message.field_count);
            
            // Access fields
            if let Some(ref field) = message.fields[0] {
                println!("First field: {:?}", field.as_str());
            }
        }
    }
    
    // Query last received GGA message
    if let Some(last_gga) = parser.get_last_message(MessageType::GGA) {
        println!("Last GGA message timestamp: {}", last_gga.timestamp);
        
        // Extract structured parameters
        if let Some(gga_data) = last_gga.as_gga() {
            println!("Time: {:?}", gga_data.time);
            println!("Latitude: {:?} {}", gga_data.latitude, gga_data.lat_direction.unwrap_or(""));
            println!("Longitude: {:?} {}", gga_data.longitude, gga_data.lon_direction.unwrap_or(""));
            println!("Altitude: {:?} {}", gga_data.altitude, gga_data.altitude_units.unwrap_or(""));
            println!("Satellites: {:?}", gga_data.num_satellites);
        }
    }
}
```

### Streaming Example

```rust
use rustedbytes_nmea::{NmeaParser, MessageType};

fn main() {
    let mut parser = NmeaParser::new();
    
    // Simulate a stream of multiple NMEA sentences
    let stream = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n\
                   $GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n\
                   $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n";
    
    for &byte in stream.iter() {
        if let Some(message) = parser.parse_char(byte) {
            println!("Parsed {:?} message with {} fields", 
                     message.message_type, message.field_count);
        }
    }
    
    // Retrieve last messages of each type
    if let Some(gga) = parser.get_last_message(MessageType::GGA) {
        println!("Last GGA at timestamp: {}", gga.timestamp);
    }
    
    if let Some(rmc) = parser.get_last_message(MessageType::RMC) {
        println!("Last RMC at timestamp: {}", rmc.timestamp);
    }
    
    if let Some(gsa) = parser.get_last_message(MessageType::GSA) {
        println!("Last GSA at timestamp: {}", gsa.timestamp);
    }
}
```

## API

### `NmeaParser`

The main parser structure.

#### Methods

- `new()` - Create a new parser instance
- `parse_char(c: u8) -> Option<NmeaMessage>` - Parse a single character. Returns `Some(NmeaMessage)` when a complete message is parsed.
- `get_last_message(msg_type: MessageType) -> Option<&NmeaMessage>` - Get the last received message of a specific type
- `reset()` - Reset the parser state

### `NmeaMessage`

Represents a parsed NMEA message.

#### Fields

- `message_type: MessageType` - The type of NMEA message
- `fields: [Option<Field>; MAX_FIELDS]` - Array of parsed fields
- `field_count: usize` - Number of fields in the message
- `timestamp: u64` - Internal timestamp counter

#### Methods

- `as_gga() -> Option<GgaData>` - Extract GGA message parameters
- `as_rmc() -> Option<RmcData>` - Extract RMC message parameters
- `as_gsa() -> Option<GsaData>` - Extract GSA message parameters
- `as_gsv() -> Option<GsvData>` - Extract GSV message parameters
- `as_gll() -> Option<GllData>` - Extract GLL message parameters
- `as_vtg() -> Option<VtgData>` - Extract VTG message parameters

### `MessageType`

Enumeration of supported NMEA message types:
- `GGA` - Global Positioning System Fix Data
- `RMC` - Recommended Minimum Navigation Information
- `GSA` - GPS DOP and active satellites
- `GSV` - GPS Satellites in view
- `GLL` - Geographic Position - Latitude/Longitude
- `VTG` - Track Made Good and Ground Speed
- `Unknown` - Unrecognized message type

### `Field`

Represents a field value in an NMEA message.

#### Methods

- `as_str() -> Option<&str>` - Get the field as a string slice
- `as_bytes() -> &[u8]` - Get the field as a byte slice

### Parameter Structures

The library provides typed parameter structures for each NMEA message type, allowing structured access to message-specific fields.

#### `GgaData`

Global Positioning System Fix Data parameters:
- `time` - UTC time (hhmmss format)
- `latitude` - Latitude value
- `lat_direction` - N or S
- `longitude` - Longitude value
- `lon_direction` - E or W
- `fix_quality` - Fix quality (0=invalid, 1=GPS fix, 2=DGPS fix, etc.)
- `num_satellites` - Number of satellites in use
- `hdop` - Horizontal Dilution of Precision
- `altitude` - Altitude above mean sea level
- `altitude_units` - Units of altitude (M for meters)
- `geoid_separation` - Height of geoid above WGS84 ellipsoid
- `geoid_units` - Units of geoid separation
- `age_of_diff` - Age of differential GPS data
- `diff_station_id` - Differential reference station ID

#### `RmcData`

Recommended Minimum Navigation Information parameters:
- `time` - UTC time (hhmmss format)
- `status` - Status (A=active/valid, V=void/invalid)
- `latitude` - Latitude value
- `lat_direction` - N or S
- `longitude` - Longitude value
- `lon_direction` - E or W
- `speed_knots` - Speed over ground in knots
- `track_angle` - Track angle in degrees
- `date` - Date (ddmmyy format)
- `magnetic_variation` - Magnetic variation
- `mag_var_direction` - E or W

#### `GsaData`

GPS DOP and active satellites parameters:
- `mode` - Mode (M=manual, A=automatic)
- `fix_type` - Fix type (1=no fix, 2=2D, 3=3D)
- `satellite_ids` - Array of up to 12 satellite PRN numbers
- `pdop` - Position Dilution of Precision
- `hdop` - Horizontal Dilution of Precision
- `vdop` - Vertical Dilution of Precision

#### `GsvData`

GPS Satellites in view parameters:
- `num_messages` - Total number of GSV messages
- `message_num` - Current message number
- `satellites_in_view` - Total number of satellites in view
- `satellite_info` - Array of up to 4 satellite information structures

Each `SatelliteInfo` contains:
- `prn` - Satellite PRN number
- `elevation` - Elevation in degrees (0-90)
- `azimuth` - Azimuth in degrees (0-359)
- `snr` - Signal-to-Noise Ratio in dB

#### `GllData`

Geographic Position parameters:
- `latitude` - Latitude value
- `lat_direction` - N or S
- `longitude` - Longitude value
- `lon_direction` - E or W
- `time` - UTC time (hhmmss format)
- `status` - Status (A=active/valid, V=void/invalid)

#### `VtgData`

Track Made Good and Ground Speed parameters:
- `track_true` - True track angle
- `track_true_indicator` - T (true)
- `track_magnetic` - Magnetic track angle
- `track_magnetic_indicator` - M (magnetic)
- `speed_knots` - Speed in knots
- `speed_knots_indicator` - N (knots)
- `speed_kph` - Speed in kilometers per hour
- `speed_kph_indicator` - K (km/h)

## Testing

Run the test suite:

```bash
cargo test
```

## License

MIT License - see LICENSE file for details.

