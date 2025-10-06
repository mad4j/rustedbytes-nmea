# rustedbytes-nmea

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

## Testing

Run the test suite:

```bash
cargo test
```

## License

MIT License - see LICENSE file for details.

