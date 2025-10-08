# rustedbytes-nmea

[![Crates.io](https://img.shields.io/crates/v/rustedbytes-nmea.svg)](https://crates.io/crates/rustedbytes-nmea)
[![Documentation](https://docs.rs/rustedbytes-nmea/badge.svg)](https://docs.rs/rustedbytes-nmea)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Tests](https://github.com/mad4j/rustedbytes-nmea/actions/workflows/test.yml/badge.svg)](https://github.com/mad4j/rustedbytes-nmea/actions/workflows/test.yml)
[![Test Count](https://img.shields.io/badge/tests-127-brightgreen.svg)]()

Rust `no_std` library for parsing NMEA messages from a GNSS receiver.

## Features

- `no_std` compatible - can be used in embedded systems
- **Stateless parser** - no internal buffers or state retention
- **Multi-byte parsing** - parse multiple bytes at once with bytes consumed tracking
- **Local time registration** - optionally record local reception time for each message
- **Multiconstellation support** - tracks which GNSS constellation provided each message
  - GPS (GP), GLONASS (GL), Galileo (GA), BeiDou (GB/BD), Multi-GNSS (GN), QZSS (QZ)
- Supports common NMEA message types:
  - GGA (Global Positioning System Fix Data)
  - RMC (Recommended Minimum Navigation Information)
  - GSA (GPS DOP and active satellites)
  - GSV (GPS Satellites in view)
  - GLL (Geographic Position - Latitude/Longitude)
  - VTG (Track Made Good and Ground Speed)
- Handles spurious characters between messages
- Structured parameter extraction for each message type

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustedbytes-nmea = "0.1.0"
```

### Basic Example

```rust
use rustedbytes_nmea::{NmeaParser, MessageType, NmeaMessage, ParseError};

fn main() {
    let parser = NmeaParser::new();
    
    // NMEA sentence as bytes (can contain multiple messages or partial data)
    let data = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
    
    // Parse the data
    let result = parser.parse_bytes(data);
    
    match result {
        Ok((Some(message), bytes_consumed)) => {
            // Successfully parsed a complete message
            match message {
                NmeaMessage::GGA(gga_data) => {
                    println!("GGA message from {:?}", gga_data.talker_id);
                    println!("Time: {}", gga_data.time());
                    println!("Latitude: {} {}", gga_data.latitude, gga_data.lat_direction);
                    println!("Longitude: {} {}", gga_data.longitude, gga_data.lon_direction);
                    println!("Altitude: {:?} {:?}", gga_data.altitude, gga_data.altitude_units);
                    println!("Satellites: {:?}", gga_data.num_satellites);
                }
                NmeaMessage::RMC(rmc_data) => {
                    println!("RMC message from {:?}", rmc_data.talker_id);
                    println!("Time: {}", rmc_data.time());
                    println!("Status: {}", rmc_data.status);
                    println!("Speed: {} knots", rmc_data.speed_knots);
                }
                _ => {} // Handle other message types
            }
            println!("Consumed {} bytes", bytes_consumed);
        }
        Ok((None, bytes_consumed)) => {
            // Partial message or spurious data - need more bytes
            println!("Partial message, consumed {} bytes", bytes_consumed);
        }
        Err((ParseError::InvalidMessage, bytes_consumed)) => {
            // Complete but invalid message (e.g., missing mandatory fields)
            println!("Invalid message found, consumed {} bytes", bytes_consumed);
        }
        Err((ParseError::InvalidChecksum, bytes_consumed)) => {
            // Checksum verification failed
            println!("Invalid checksum, consumed {} bytes", bytes_consumed);
        }
    }
}
```

### Streaming Example

```rust
use rustedbytes_nmea::{NmeaParser, MessageType};

fn main() {
    let parser = NmeaParser::new();
    
    // Simulate a stream of multiple NMEA sentences
    let mut data = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n\
                     $GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A\r\n\
                     $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39\r\n".as_slice();
    
    // Parse all messages in the stream
    while !data.is_empty() {
        match parser.parse_bytes(data) {
            Ok((msg, consumed)) => {
                if consumed == 0 {
                    // Partial message - would need more data in a real stream
                    break;
                }
                
                match msg {
                    Some(message) => {
                        println!("Parsed {:?} message", message.message_type());
                    }
                    None => {
                        // Spurious data consumed
                        println!("Consumed {} bytes of spurious data", consumed);
                    }
                }
                
                // Move to next message
                data = &data[consumed..];
            }
            Err((error, consumed)) => {
                println!("Parse error: {:?}, consumed {} bytes", error, consumed);
                // Move past the invalid message
                data = &data[consumed..];
            }
        }
    }
}
```

### Local Time Registration

The library supports recording the local reception time for each message. Since this is a `no_std` library, it cannot access system time directly. Instead, users provide timestamps when parsing:

```rust
use rustedbytes_nmea::NmeaParser;

fn main() {
    let parser = NmeaParser::new();
    let sentence = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
    
    // Get local timestamp from your system (milliseconds since some epoch)
    // In std environments, you might use:
    // use std::time::{SystemTime, UNIX_EPOCH};
    // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    
    // For this example, we'll use a fixed timestamp
    let local_timestamp_ms = 1234567890123u64;
    
    // Parse with timestamp
    let result = parser.parse_bytes_with_timestamp(sentence, Some(local_timestamp_ms));
    
    if let Ok((Some(msg), _)) = result {
        if let Some(gga) = msg.as_gga() {
            println!("UTC time from GPS: {}", gga.time());
            println!("Local reception time: {:?} ms", gga.local_timestamp_ms);
        }
    }
}
```

**Benefits:**
- Track message reception time independently from GPS time
- Useful for timestamping data in your local system's time base
- Works in embedded systems with RTCs or monotonic clocks
- Allows correlation between GPS time and local system time

**Note:** The timestamp is optional. If you use `parse_bytes()` instead of `parse_bytes_with_timestamp()`, all messages will have `local_timestamp_ms` set to `None`.

### Multiconstellation Support

The library automatically tracks which GNSS constellation provided each message through the `talker_id` field:

```rust
use rustedbytes_nmea::{NmeaParser, TalkerId};

fn main() {
    let parser = NmeaParser::new();
    
    // Parse messages from different constellations
    let sentences = [
        b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // GPS
        b"$GLGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // GLONASS
        b"$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // Galileo
        b"$GNGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n", // Multi-GNSS
    ];
    
    for sentence in &sentences {
        if let Ok((Some(message), _)) = parser.parse_bytes(sentence) {
            if let Some(gga_data) = message.as_gga() {
                match gga_data.talker_id {
                    TalkerId::GP => println!("GPS fix: {}", gga_data.time()),
                    TalkerId::GL => println!("GLONASS fix: {}", gga_data.time()),
                    TalkerId::GA => println!("Galileo fix: {}", gga_data.time()),
                    TalkerId::GN => println!("Multi-GNSS fix: {}", gga_data.time()),
                    _ => println!("Other constellation fix"),
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

The main parser structure. **The parser is now stateless** - it maintains no internal buffers or message storage.

#### Methods

- `new()` - Create a new parser instance
- `parse_bytes(data: &[u8]) -> Result<(Option<NmeaMessage>, usize), (ParseError, usize)>` - Parse bytes and return:
  - `Ok((Some(message), bytes_consumed))` - Successfully parsed a complete, valid message
  - `Ok((None, bytes_consumed))` - Partial message (need more data) or consumed spurious characters
  - `Err((ParseError::InvalidMessage, bytes_consumed))` - Complete message but missing mandatory fields
  - `Err((ParseError::InvalidChecksum, bytes_consumed))` - Checksum verification failed
- `parse_bytes_with_timestamp(data: &[u8], local_timestamp_ms: Option<u64>) -> Result<(Option<NmeaMessage>, usize), (ParseError, usize)>` - Parse bytes with local reception timestamp. Returns same as `parse_bytes()` but includes the timestamp in all message structures.

### `ParseError`

Error types returned when parsing fails:

- `InvalidMessage` - Message is syntactically complete but missing mandatory fields or invalid
- `InvalidChecksum` - Checksum verification failed (not yet fully implemented)

### `NmeaMessage`

Enum representing a parsed NMEA message with associated data.

#### Variants

- `GGA(GgaData)` - Global Positioning System Fix Data
- `RMC(RmcData)` - Recommended Minimum Navigation Information
- `GSA(GsaData)` - GPS DOP and active satellites
- `GSV(GsvData)` - GPS Satellites in view
- `GLL(GllData)` - Geographic Position - Latitude/Longitude
- `VTG(VtgData)` - Track Made Good and Ground Speed

#### Methods

- `message_type() -> MessageType` - Get the message type identifier
- `talker_id() -> TalkerId` - Get the talker ID (constellation identifier)
- `as_gga() -> Option<&GgaData>` - Extract GGA message parameters
- `as_rmc() -> Option<&RmcData>` - Extract RMC message parameters
- `as_gsa() -> Option<&GsaData>` - Extract GSA message parameters
- `as_gsv() -> Option<&GsvData>` - Extract GSV message parameters
- `as_gll() -> Option<&GllData>` - Extract GLL message parameters
- `as_vtg() -> Option<&VtgData>` - Extract VTG message parameters

### `MessageType`

Enumeration of NMEA message type identifiers:
- `GGA` - Global Positioning System Fix Data
- `RMC` - Recommended Minimum Navigation Information
- `GSA` - GPS DOP and active satellites
- `GSV` - GPS Satellites in view
- `GLL` - Geographic Position - Latitude/Longitude
- `VTG` - Track Made Good and Ground Speed
- `Unknown` - Unrecognized message type

### Parameter Structures

The library provides typed parameter structures for each NMEA message type, allowing structured access to message-specific fields.

#### `GgaData`

Global Positioning System Fix Data parameters:
- `time()` - **Mandatory** - UTC time (hhmmss format) - accessed via method
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
- `diff_station_id()` - *Optional* - Differential reference station ID - accessed via method
- `local_timestamp_ms` - *Optional* - Local reception timestamp in milliseconds (set by user during parsing)

**Note:** If any mandatory field is missing or cannot be parsed, the parser returns `None`.

#### `RmcData`

Recommended Minimum Navigation Information parameters:
- `time()` - **Mandatory** - UTC time (hhmmss format) - accessed via method
- `status` - **Mandatory** - Status (A=active/valid, V=void/invalid)
- `latitude` - **Mandatory** - Latitude value
- `lat_direction` - **Mandatory** - N or S
- `longitude` - **Mandatory** - Longitude value
- `lon_direction` - **Mandatory** - E or W
- `speed_knots` - **Mandatory** - Speed over ground in knots
- `track_angle` - **Mandatory** - Track angle in degrees
- `date()` - **Mandatory** - Date (ddmmyy format) - accessed via method
- `magnetic_variation` - *Optional* - Magnetic variation
- `mag_var_direction` - *Optional* - E or W
- `local_timestamp_ms` - *Optional* - Local reception timestamp in milliseconds (set by user during parsing)

**Note:** If any mandatory field is missing or cannot be parsed, the parser returns `None`.

#### `GsaData`

GPS DOP and active satellites parameters:
- `mode` - **Mandatory** - Mode (M=manual, A=automatic)
- `fix_type` - **Mandatory** - Fix type (1=no fix, 2=2D, 3=3D)
- `satellite_ids` - *Optional* - Array of up to 12 satellite PRN numbers
- `pdop` - *Optional* - Position Dilution of Precision
- `hdop` - *Optional* - Horizontal Dilution of Precision
- `vdop` - *Optional* - Vertical Dilution of Precision
- `local_timestamp_ms` - *Optional* - Local reception timestamp in milliseconds (set by user during parsing)

**Note:** If any mandatory field is missing or cannot be parsed, `as_gsa()` returns `None`.

#### `GsvData`

GPS Satellites in view parameters:
- `num_messages` - **Mandatory** - Total number of GSV messages
- `message_num` - **Mandatory** - Current message number
- `satellites_in_view` - **Mandatory** - Total number of satellites in view
- `satellite_info` - *Optional* - Array of up to 4 satellite information structures
- `local_timestamp_ms` - *Optional* - Local reception timestamp in milliseconds (set by user during parsing)

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
- `time()` - **Mandatory** - UTC time (hhmmss format) - accessed via method
- `status` - **Mandatory** - Status (A=active/valid, V=void/invalid)
- `local_timestamp_ms` - *Optional* - Local reception timestamp in milliseconds (set by user during parsing)

**Note:** If any mandatory field is missing or cannot be parsed, the parser returns `None`.

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
- `local_timestamp_ms` - *Optional* - Local reception timestamp in milliseconds (set by user during parsing)

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

