# rustedbytes-nmea

[![Crates.io](https://img.shields.io/crates/v/rustedbytes-nmea.svg)](https://crates.io/crates/rustedbytes-nmea)
[![Documentation](https://docs.rs/rustedbytes-nmea/badge.svg)](https://docs.rs/rustedbytes-nmea)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Tests](https://github.com/mad4j/rustedbytes-nmea/actions/workflows/test.yml/badge.svg)](https://github.com/mad4j/rustedbytes-nmea/actions/workflows/test.yml)

Rust `no_std` library for parsing NMEA messages from a GNSS receiver.

## Features

- `no_std` compatible - can be used in embedded systems
- Character stream parsing - feed characters one at a time
- **Multiconstellation support** - tracks which GNSS constellation provided each message
  - GPS (GP), GLONASS (GL), Galileo (GA), BeiDou (GB/BD), Multi-GNSS (GN), QZSS (QZ)
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
            println!("Time: {}", gga_data.time);
            println!("Constellation: {:?}", gga_data.talker_id);
            println!("Latitude: {} {}", gga_data.latitude, gga_data.lat_direction);
            println!("Longitude: {} {}", gga_data.longitude, gga_data.lon_direction);
            println!("Altitude: {:?} {:?}", gga_data.altitude, gga_data.altitude_units);
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

### Multiconstellation Support

The library automatically tracks which GNSS constellation provided each message through the `talker_id` field:

```rust
use rustedbytes_nmea::{NmeaParser, TalkerId};

fn main() {
    let mut parser = NmeaParser::new();
    
    // Parse messages from different constellations
    let sentences = [
        b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // GPS
        b"$GLGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // GLONASS
        b"$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // Galileo
        b"$GNGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // Multi-GNSS
    ];
    
    for sentence in &sentences {
        for &byte in sentence.iter() {
            if let Some(message) = parser.parse_char(byte) {
                if let Some(gga_data) = message.as_gga() {
                    match gga_data.talker_id {
                        TalkerId::GP => println!("GPS fix: {}", gga_data.time),
                        TalkerId::GL => println!("GLONASS fix: {}", gga_data.time),
                        TalkerId::GA => println!("Galileo fix: {}", gga_data.time),
                        TalkerId::GN => println!("Multi-GNSS fix: {}", gga_data.time),
                        _ => println!("Other constellation fix"),
                    }
                }
            }
        }
    }
}
```

### Supported Constellations

| Talker ID | Constellation | Description |
|-----------|---------------|-------------|
| `GP` | GPS | Global Positioning System (USA) |
| `GL` | GLONASS | Russian satellite navigation |
| `GA` | Galileo | European satellite navigation |
| `GB` | BeiDou | Chinese satellite navigation (GBxxxx format) |
| `BD` | BeiDou | Chinese satellite navigation (BDxxxx format) |
| `GN` | Multi-GNSS | Combined data from multiple systems |
| `QZ` | QZSS | Japanese Quasi-Zenith Satellite System |


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
- `time` - **Mandatory** - UTC time (hhmmss format)
- `latitude` - **Mandatory** - Latitude value
- `lat_direction` - **Mandatory** - N or S
- `longitude` - **Mandatory** - Longitude value
- `lon_direction` - **Mandatory** - E or W
- `fix_quality` - **Mandatory** - Fix quality (0=invalid, 1=GPS fix, 2=DGPS fix, etc.)
- `num_satellites` - *Optional* - Number of satellites in use
- `hdop` - *Optional* - Horizontal Dilution of Precision
- `altitude` - *Optional* - Altitude above mean sea level
- `altitude_units` - *Optional* - Units of altitude (M for meters)
- `geoid_separation` - *Optional* - Height of geoid above WGS84 ellipsoid
- `geoid_units` - *Optional* - Units of geoid separation
- `age_of_diff` - *Optional* - Age of differential GPS data
- `diff_station_id` - *Optional* - Differential reference station ID

**Note:** If any mandatory field is missing or cannot be parsed, `as_gga()` returns `None`.

#### `RmcData`

Recommended Minimum Navigation Information parameters:
- `time` - **Mandatory** - UTC time (hhmmss format)
- `status` - **Mandatory** - Status (A=active/valid, V=void/invalid)
- `latitude` - **Mandatory** - Latitude value
- `lat_direction` - **Mandatory** - N or S
- `longitude` - **Mandatory** - Longitude value
- `lon_direction` - **Mandatory** - E or W
- `speed_knots` - **Mandatory** - Speed over ground in knots
- `track_angle` - **Mandatory** - Track angle in degrees
- `date` - **Mandatory** - Date (ddmmyy format)
- `magnetic_variation` - *Optional* - Magnetic variation
- `mag_var_direction` - *Optional* - E or W

**Note:** If any mandatory field is missing or cannot be parsed, `as_rmc()` returns `None`.

#### `GsaData`

GPS DOP and active satellites parameters:
- `mode` - **Mandatory** - Mode (M=manual, A=automatic)
- `fix_type` - **Mandatory** - Fix type (1=no fix, 2=2D, 3=3D)
- `satellite_ids` - *Optional* - Array of up to 12 satellite PRN numbers
- `pdop` - *Optional* - Position Dilution of Precision
- `hdop` - *Optional* - Horizontal Dilution of Precision
- `vdop` - *Optional* - Vertical Dilution of Precision

**Note:** If any mandatory field is missing or cannot be parsed, `as_gsa()` returns `None`.

#### `GsvData`

GPS Satellites in view parameters:
- `num_messages` - **Mandatory** - Total number of GSV messages
- `message_num` - **Mandatory** - Current message number
- `satellites_in_view` - **Mandatory** - Total number of satellites in view
- `satellite_info` - *Optional* - Array of up to 4 satellite information structures

Each `SatelliteInfo` contains:
- `prn` - *Optional* - Satellite PRN number
- `elevation` - *Optional* - Elevation in degrees (0-90)
- `azimuth` - *Optional* - Azimuth in degrees (0-359)
- `snr` - *Optional* - Signal-to-Noise Ratio in dB

**Note:** If any mandatory field is missing or cannot be parsed, `as_gsv()` returns `None`.

#### `GllData`

Geographic Position parameters:
- `latitude` - **Mandatory** - Latitude value
- `lat_direction` - **Mandatory** - N or S
- `longitude` - **Mandatory** - Longitude value
- `lon_direction` - **Mandatory** - E or W
- `time` - **Mandatory** - UTC time (hhmmss format)
- `status` - **Mandatory** - Status (A=active/valid, V=void/invalid)

**Note:** If any mandatory field is missing or cannot be parsed, `as_gll()` returns `None`.

#### `VtgData`

Track Made Good and Ground Speed parameters (all fields are optional):
- `track_true` - *Optional* - True track angle
- `track_true_indicator` - *Optional* - T (true)
- `track_magnetic` - *Optional* - Magnetic track angle
- `track_magnetic_indicator` - *Optional* - M (magnetic)
- `speed_knots` - *Optional* - Speed in knots
- `speed_knots_indicator` - *Optional* - N (knots)
- `speed_kph` - *Optional* - Speed in kilometers per hour
- `speed_kph_indicator` - *Optional* - K (km/h)

**Note:** VTG messages can be parsed even with all fields empty, as all fields are optional.

## NMEA 0183 Compliance

For detailed information about the library's compliance with the NMEA 0183 standard, including supported and unsupported message types and fields, see the [NMEA 0183 Compliance Matrix](NMEA-183-COMPLIANCE.md).

## Testing

Run the test suite:

```bash
cargo test
```

## License

MIT License - see LICENSE file for details.

