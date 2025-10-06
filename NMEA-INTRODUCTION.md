# Introduction to NMEA 0183

This document provides a comprehensive introduction to the NMEA 0183 protocol, its message format, and commonly implemented message types.

## What is NMEA 0183?

NMEA 0183 is a combined electrical and data specification for communication between marine electronics such as echo sounder, sonars, anemometer, gyrocompass, autopilot, GPS receivers, and many other types of instruments. It has been defined by, and is controlled by, the U.S. National Marine Electronics Association (NMEA).

**Key Points:**
- **Established:** 1983
- **Protocol Type:** Serial communications protocol
- **Primary Use:** GPS/GNSS receivers and marine navigation equipment
- **Data Format:** Human-readable ASCII sentences
- **Transmission:** Asynchronous serial communication at 4800 baud (standard)

The standard has evolved over time, with major versions including:
- **NMEA 0183 v2.0** - Original widespread version
- **NMEA 0183 v3.0** - Added mode indicators and additional fields
- **NMEA 0183 v4.0+** - Extended message types and GNSS support

## Why NMEA 0183?

Despite being a relatively old standard, NMEA 0183 remains widely used because:

1. **Simplicity**: Human-readable ASCII format makes debugging easy
2. **Ubiquity**: Supported by virtually all GPS/GNSS receivers
3. **Compatibility**: Works with legacy and modern equipment
4. **Self-describing**: Each sentence contains a type identifier
5. **Lightweight**: Minimal overhead, suitable for embedded systems

## NMEA 0183 Message Format

### Basic Structure

Every NMEA 0183 sentence follows this general format:

```
$<talker_id><sentence_id>,<field_1>,<field_2>,...,<field_n>*<checksum><CR><LF>
```

### Components

#### 1. Start Character (`$`)
- Every sentence begins with a dollar sign (`$`)
- Marks the beginning of a new message
- ASCII value: 0x24

#### 2. Talker ID (2 characters)
- Identifies the source device or GNSS constellation
- Examples:
  - `GP` - GPS (USA)
  - `GL` - GLONASS (Russia)
  - `GA` - Galileo (Europe)
  - `GB` or `BD` - BeiDou (China)
  - `GN` - Multi-GNSS (combined systems)
  - `QZ` - QZSS (Japan)

#### 3. Sentence ID (3 characters)
- Identifies the message type
- Examples:
  - `GGA` - Global Positioning System Fix Data
  - `RMC` - Recommended Minimum Navigation Information
  - `GSA` - GPS DOP and Active Satellites
  - `GSV` - GPS Satellites in View
  - `GLL` - Geographic Position - Latitude/Longitude
  - `VTG` - Track Made Good and Ground Speed

#### 4. Data Fields
- Comma-separated values
- Can be empty (consecutive commas indicate missing data)
- Field types vary by message type (strings, integers, floats)
- Maximum sentence length: 82 characters (including `$` and `*<checksum>`)

#### 5. Checksum (`*<HH>`)
- Optional but recommended
- Asterisk (`*`) followed by two hexadecimal digits
- XOR of all characters between `$` and `*` (exclusive)
- Provides error detection

#### 6. Terminator (`<CR><LF>`)
- Carriage Return (0x0D) and Line Feed (0x0A)
- Marks the end of the sentence

### Example Sentence Breakdown

```
$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47<CR><LF>
```

Breaking this down:
- `$` - Start character
- `GP` - Talker ID (GPS)
- `GGA` - Sentence ID (Fix Data)
- `123519` - Field 1: UTC time (12:35:19)
- `4807.038` - Field 2: Latitude (48°07.038')
- `N` - Field 3: North hemisphere
- `01131.000` - Field 4: Longitude (11°31.000')
- `E` - Field 5: East hemisphere
- `1` - Field 6: Fix quality (GPS fix)
- `08` - Field 7: Number of satellites
- `0.9` - Field 8: HDOP (horizontal dilution)
- `545.4` - Field 9: Altitude
- `M` - Field 10: Altitude units (meters)
- `46.9` - Field 11: Geoid separation
- `M` - Field 12: Geoid units (meters)
- `` - Field 13: Empty (age of differential)
- `` - Field 14: Empty (differential station ID)
- `*47` - Checksum
- `<CR><LF>` - Terminator

## Checksum Calculation

The checksum is calculated by XORing all characters between the `$` and `*` (exclusive).

### Algorithm

```
1. Start with checksum = 0
2. For each character between '$' and '*' (exclusive):
   checksum = checksum XOR character
3. Convert checksum to two-digit hexadecimal
```

### Example in Pseudocode

```
sentence = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47"
checksum = 0

for char in sentence[1:sentence.index('*')]:
    checksum = checksum XOR ascii_value(char)

# checksum should equal 0x47 (71 decimal)
```

### Example in Rust

```rust
fn calculate_checksum(sentence: &str) -> u8 {
    let start = sentence.find('$').map(|i| i + 1).unwrap_or(0);
    let end = sentence.find('*').unwrap_or(sentence.len());
    
    sentence[start..end]
        .bytes()
        .fold(0u8, |checksum, byte| checksum ^ byte)
}
```

## Coordinate Format

NMEA 0183 uses a specific format for latitude and longitude:

### Latitude Format: `DDMM.MMMM`
- `DD` - Degrees (00-90)
- `MM.MMMM` - Minutes with decimal (00.0000-59.9999)
- Followed by direction indicator: `N` (North) or `S` (South)

**Example:** `4807.038,N` = 48°07.038' North

### Longitude Format: `DDDMM.MMMM`
- `DDD` - Degrees (000-180)
- `MM.MMMM` - Minutes with decimal (00.0000-59.9999)
- Followed by direction indicator: `E` (East) or `W` (West)

**Example:** `01131.000,E` = 11°31.000' East

### Converting to Decimal Degrees

To convert NMEA coordinates to decimal degrees:

```
decimal_degrees = degrees + (minutes / 60)
```

If the direction is South or West, negate the result.

**Example:**
```
4807.038 N = 48 + (07.038 / 60) = 48.1173° N
01131.000 E = 11 + (31.000 / 60) = 11.5167° E
```

## Time and Date Format

### Time Format: `HHMMSS.sss`
- `HH` - Hours (00-23) in UTC
- `MM` - Minutes (00-59)
- `SS.sss` - Seconds with optional decimal (00.000-59.999)

**Example:** `123519` or `123519.00` = 12:35:19 UTC

### Date Format: `DDMMYY`
- `DD` - Day (01-31)
- `MM` - Month (01-12)
- `YY` - Year (00-99, representing 2000-2099)

**Example:** `230394` = March 23, 1994

**Note:** All times in NMEA are UTC (Coordinated Universal Time), not local time.

## Commonly Implemented Message Types

### 1. GGA - Global Positioning System Fix Data

**Purpose:** Provides essential GPS fix data including position, time, and quality indicators.

**Format:**
```
$GPGGA,hhmmss.ss,llll.ll,a,yyyyy.yy,a,x,xx,x.x,x.x,M,x.x,M,x.x,xxxx*hh
```

**Key Fields:**
- UTC time
- Latitude and longitude
- Fix quality (0=invalid, 1=GPS, 2=DGPS, etc.)
- Number of satellites
- HDOP (horizontal dilution of precision)
- Altitude above mean sea level
- Geoid separation

**Example:**
```
$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47
```

**Use Cases:**
- Basic position determination
- Quality monitoring
- Altitude tracking
- Multi-constellation comparison

---

### 2. RMC - Recommended Minimum Navigation Information

**Purpose:** Provides the minimum navigation data including position, speed, course, and date. Often called the "Recommended Minimum" sentence.

**Format:**
```
$GPRMC,hhmmss.ss,A,llll.ll,a,yyyyy.yy,a,x.x,x.x,ddmmyy,x.x,a*hh
```

**Key Fields:**
- UTC time
- Status (A=active/valid, V=void/invalid)
- Latitude and longitude
- Speed over ground (knots)
- Track angle (degrees)
- Date
- Magnetic variation

**Example:**
```
$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A
```

**Use Cases:**
- Complete navigation snapshot
- Speed and heading tracking
- Date/time synchronization
- Minimal data logging

---

### 3. GSA - GPS DOP and Active Satellites

**Purpose:** Provides information about satellite selection, fix mode, and dilution of precision values.

**Format:**
```
$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39
```

**Key Fields:**
- Mode (M=manual, A=automatic)
- Fix type (1=no fix, 2=2D, 3=3D)
- PRN numbers of satellites used (up to 12)
- PDOP (position dilution of precision)
- HDOP (horizontal dilution of precision)
- VDOP (vertical dilution of precision)

**Example:**
```
$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39
```

**DOP Value Interpretation:**
- < 1: Ideal
- 1-2: Excellent
- 2-5: Good
- 5-10: Moderate
- 10-20: Fair
- > 20: Poor

**Use Cases:**
- Signal quality assessment
- Satellite geometry monitoring
- Precision estimation
- Fix reliability evaluation

---

### 4. GSV - GPS Satellites in View

**Purpose:** Provides detailed information about all satellites in view, including elevation, azimuth, and signal strength.

**Format:**
```
$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75
```

**Key Fields:**
- Total number of GSV messages
- Current message number
- Total satellites in view
- For each satellite (up to 4 per message):
  - PRN (satellite number)
  - Elevation (0-90°)
  - Azimuth (0-359°)
  - SNR (signal-to-noise ratio, 0-99 dB)

**Example:**
```
$GPGSV,2,1,08,01,40,083,46,02,17,308,41,12,07,344,39,14,22,228,45*75
$GPGSV,2,2,08,17,40,208,44,19,38,120,43,24,25,047,42,27,27,311,40*7B
```

**Use Cases:**
- Satellite tracking
- Sky view visualization
- Signal strength monitoring
- Troubleshooting reception issues

---

### 5. GLL - Geographic Position - Latitude/Longitude

**Purpose:** Provides basic geographic position and time, simpler alternative to GGA.

**Format:**
```
$GPGLL,llll.ll,a,yyyyy.yy,a,hhmmss.ss,A*hh
```

**Key Fields:**
- Latitude and longitude
- UTC time
- Status (A=valid, V=invalid)

**Example:**
```
$GPGLL,4916.45,N,12311.12,W,225444,A*1D
```

**Use Cases:**
- Simple position tracking
- Low-bandwidth applications
- Position-only logging

---

### 6. VTG - Track Made Good and Ground Speed

**Purpose:** Provides velocity information including true and magnetic track angles and ground speed.

**Format:**
```
$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48
```

**Key Fields:**
- True track angle (degrees from true north)
- True indicator (T)
- Magnetic track angle (degrees from magnetic north)
- Magnetic indicator (M)
- Ground speed in knots
- Knots indicator (N)
- Ground speed in km/h
- Km/h indicator (K)

**Example:**
```
$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48
```

**Use Cases:**
- Navigation and heading
- Speed monitoring
- Course tracking
- Magnetic variation calculation

## Multi-Constellation Support

Modern GNSS receivers can use multiple satellite constellations simultaneously. The talker ID indicates the source:

### Constellation-Specific Messages

```
$GPGGA,... - GPS only
$GLGGA,... - GLONASS only
$GAGGA,... - Galileo only
$GBGGA,... - BeiDou only
$QZGGA,... - QZSS only
```

### Combined Messages

```
$GNGGA,... - Multi-GNSS (combined data)
```

**Best Practice:** Use `GN` messages when available for best accuracy, as they leverage all available satellite systems.

## Parsing Strategies

### Character-by-Character Parsing

Suitable for embedded systems and stream processing:

```rust
let mut parser = NmeaParser::new();

for byte in uart.read_bytes() {
    if let Some(message) = parser.parse_char(byte) {
        // Process complete message
        handle_message(message);
    }
}
```

**Advantages:**
- Low memory overhead
- No buffering required
- Real-time processing
- Suitable for embedded systems

### Line-by-Line Parsing

Suitable for applications with buffered input:

```rust
let mut parser = NmeaParser::new();

for line in reader.lines() {
    for byte in line.bytes() {
        if let Some(message) = parser.parse_char(byte) {
            handle_message(message);
        }
    }
}
```

**Advantages:**
- Simpler for file processing
- Better error recovery
- Easier debugging

## Best Practices

### 1. Validate Checksums
Always verify checksums when present to detect transmission errors.

### 2. Handle Empty Fields
NMEA sentences can have empty fields (consecutive commas). Always check for `None` or empty values.

### 3. Check Mandatory Fields
Some fields are mandatory. If they're missing, the entire message should be considered invalid.

### 4. Use Appropriate Message Types
Choose the message type that provides the data you need:
- Position only → GLL
- Position + quality → GGA
- Position + speed + date → RMC
- Satellite info → GSA, GSV

### 5. Consider Update Rates
Different message types may have different update rates:
- GGA/RMC: Typically 1-10 Hz
- GSA: Usually 1 Hz
- GSV: Often 1 Hz, multiple sentences per update

### 6. Handle Multi-Constellation Data
Modern receivers output messages from multiple constellations. Track the talker ID to identify the source.

### 7. Buffer Management
Keep buffers sized appropriately:
- Maximum sentence length: 82 characters
- Typical sentence length: 40-80 characters
- Allow room for maximum field count (typically 20-30 fields)

### 8. Time Synchronization
NMEA provides UTC time. Convert to local time as needed in your application layer, not during parsing.

## Common Pitfalls

### 1. Coordinate Conversion Errors
Remember that NMEA coordinates are in degrees and minutes, not decimal degrees.

**Wrong:**
```rust
// Don't use the value directly
let latitude = 4807.038; // This is NOT 48.07038 degrees!
```

**Correct:**
```rust
// Convert properly
let degrees = 48.0;
let minutes = 07.038;
let latitude = degrees + (minutes / 60.0); // 48.1173 degrees
```

### 2. Ignoring Empty Fields
Empty fields are valid in NMEA. Don't assume all fields are populated.

```rust
// Always check for None
if let Some(altitude) = gga_data.altitude {
    println!("Altitude: {} m", altitude);
} else {
    println!("Altitude not available");
}
```

### 3. Mixing UTC and Local Time
NMEA always uses UTC. Don't confuse it with local time.

### 4. Assuming Fixed Update Rates
GPS receivers may output messages at varying rates depending on configuration and conditions.

### 5. Not Validating Fix Quality
Always check fix quality before using position data:

```rust
if gga_data.fix_quality == 0 {
    // No fix - position data is invalid
    return;
}
```

### 6. Ignoring DOP Values
High DOP values indicate poor satellite geometry and reduced accuracy.

```rust
if let Some(hdop) = gga_data.hdop {
    if hdop > 5.0 {
        println!("Warning: Poor satellite geometry");
    }
}
```

## Troubleshooting

### Problem: No Messages Received
**Possible Causes:**
- Wrong baud rate (should be 4800 baud by default)
- Incorrect serial port settings
- GPS receiver not initialized
- Antenna not connected or poor signal

### Problem: Checksum Errors
**Possible Causes:**
- Transmission interference
- Wrong character encoding
- Incomplete sentences
- Hardware issues

**Solution:** Implement retry logic and verify serial connection quality.

### Problem: Empty Position Fields
**Possible Causes:**
- No GPS fix
- Cold start (receiver needs time to acquire satellites)
- Poor signal (obstructed view of sky)

**Solution:** Wait for fix, check fix quality field, improve antenna placement.

### Problem: Position Jumps
**Possible Causes:**
- Poor satellite geometry (high DOP)
- Multipath interference
- Switching between constellations

**Solution:** Filter positions, check DOP values, implement Kalman filtering.

## Advanced Topics

### Differential GPS (DGPS)
DGPS improves accuracy using corrections from a base station. Look for:
- Fix quality = 2 in GGA messages
- Age of differential data field
- Differential station ID

### Real-Time Kinematic (RTK)
RTK provides centimeter-level accuracy. Look for:
- Fix quality = 4 (RTK fixed) or 5 (RTK float) in GGA messages
- Very low HDOP values

### Satellite-Based Augmentation Systems (SBAS)
Systems like WAAS, EGNOS, MSAS improve accuracy. Look for:
- Fix quality = 2 (DGPS/SBAS)
- Additional satellites in GSV with high PRN numbers

## Resources

### NMEA 0183 Standards
- NMEA 0183 v2.30
- NMEA 0183 v3.01
- NMEA 0183 v4.11

### Online Resources
- [NMEA Revealed](https://gpsd.gitlab.io/gpsd/NMEA.html) - Comprehensive NMEA documentation
- [GPS.gov](https://www.gps.gov) - Official GPS information
- [NMEA.org](https://www.nmea.org) - Official NMEA website

### Related Standards
- NMEA 2000 - CAN bus-based protocol for marine electronics
- NMEA OneNet - Ethernet-based protocol
- UBX - u-blox proprietary protocol

## Conclusion

NMEA 0183 remains a fundamental protocol for GPS/GNSS communication due to its simplicity, ubiquity, and human-readable format. Understanding its message structure, coordinate formats, and common message types enables effective integration of GPS functionality into applications ranging from embedded systems to mobile apps and desktop software.

Key takeaways:
1. **Simple but powerful** - ASCII-based, easy to debug
2. **Standardized** - Widely supported across devices
3. **Flexible** - Multiple message types for different needs
4. **Extensible** - Supports multiple GNSS constellations
5. **Reliable** - Checksum validation ensures data integrity

For implementation details specific to the `rustedbytes-nmea` library, see the [README.md](README.md) and [NMEA 0183 Compliance Matrix](NMEA-0183-COMPLIANCE.md).
