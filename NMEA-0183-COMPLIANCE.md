# NMEA 0183 Compliance Matrix

This document describes the compliance of the `rustedbytes-nmea` library with the NMEA 0183 specification.

## Overview

The library implements a subset of the NMEA 0183 standard, focusing on the most commonly used GPS/GNSS message types. The implementation follows the `no_std` design principle for embedded systems compatibility.

## Multiconstellation Support

The library supports NMEA messages from multiple GNSS constellations through the **Talker ID** field. Each parsed message includes a `talker_id` field that indicates which constellation provided the data.

### Supported Talker IDs

| Talker ID | Constellation | Description |
|-----------|---------------|-------------|
| **GP** | GPS | Global Positioning System (USA) |
| **GL** | GLONASS | Russian Global Navigation Satellite System |
| **GA** | Galileo | European Global Navigation Satellite System |
| **GB** | BeiDou | Chinese Navigation Satellite System (GBxxxx format) |
| **BD** | BeiDou | Chinese Navigation Satellite System (BDxxxx format) |
| **GN** | Multi-GNSS | Combined data from multiple constellation systems |
| **QZ** | QZSS | Japanese Quasi-Zenith Satellite System |

All message types (GGA, RMC, GSA, GSV, GLL, VTG) automatically track and report their source constellation through the `talker_id` field in their respective data structures.

## Supported Message Types

| Message Type | Description | Implementation Status | Notes |
|--------------|-------------|----------------------|-------|
| **GGA** | Global Positioning System Fix Data | ✅ Fully Supported | All standard fields implemented |
| **RMC** | Recommended Minimum Navigation Information | ✅ Fully Supported | All standard fields implemented |
| **GSA** | GPS DOP and Active Satellites | ✅ Fully Supported | All standard fields implemented |
| **GSV** | GPS Satellites in View | ✅ Fully Supported | Supports up to 4 satellites per message |
| **GLL** | Geographic Position - Latitude/Longitude | ✅ Fully Supported | All standard fields implemented |
| **VTG** | Track Made Good and Ground Speed | ✅ Fully Supported | All standard fields implemented |
| **GNS** | GNSS Fix Data | ✅ Fully Supported | All standard fields implemented |

## Unsupported Message Types

The following NMEA 0183 message types are **not currently supported**:

| Message Type | Description | Priority |
|--------------|-------------|----------|
| **AAM** | Waypoint Arrival Alarm | Low |
| **ALM** | GPS Almanac Data | Low |
| **APA** | Autopilot Sentence A | Low |
| **APB** | Autopilot Sentence B | Low |
| **BOD** | Bearing Origin to Destination | Low |
| **BWC** | Bearing and Distance to Waypoint | Low |
| **BWR** | Bearing and Distance to Waypoint - Rhumb Line | Low |
| **BWW** | Bearing Waypoint to Waypoint | Low |
| **DBK** | Depth Below Keel | Low |
| **DBS** | Depth Below Surface | Low |
| **DBT** | Depth Below Transducer | Low |
| **DCN** | Decca Position | Low |
| **DPT** | Depth | Low |
| **DTM** | Datum Reference | Medium |
| **FSI** | Frequency Set Information | Low |
| **GBS** | GPS Satellite Fault Detection | Medium |
| **GST** | GPS Pseudorange Noise Statistics | Medium |
| **GTD** | Geographic Location in Time Differences | Low |
| **GXA** | TRANSIT Position | Low |
| **HDG** | Heading, Deviation and Variation | Low |
| **HDM** | Heading - Magnetic | Low |
| **HDT** | Heading - True | Low |
| **HSC** | Heading Steering Command | Low |
| **LCD** | Loran-C Signal Data | Low |
| **MSK** | MSK Receiver Interface | Low |
| **MSS** | MSK Receiver Signal Status | Low |
| **MTW** | Water Temperature | Low |
| **MWD** | Wind Direction and Speed | Low |
| **MWV** | Wind Speed and Angle | Low |
| **OLN** | Omega Lane Numbers | Low |
| **OSD** | Own Ship Data | Low |
| **R00** | Waypoint List | Low |
| **RMA** | Recommended Minimum Specific Loran-C Data | Low |
| **RMB** | Recommended Minimum Navigation Information | Medium |
| **RME** | Recommended Minimum Specific GPS/TRANSIT Data | Low |
| **ROT** | Rate of Turn | Low |
| **RPM** | Revolutions | Low |
| **RSA** | Rudder Sensor Angle | Low |
| **RSD** | RADAR System Data | Low |
| **RTE** | Routes | Medium |
| **SFI** | Scanning Frequency Information | Low |
| **STN** | Multiple Data ID | Low |
| **TRF** | TRANSIT Fix Data | Low |
| **TTM** | Tracked Target Message | Low |
| **VBW** | Dual Ground/Water Speed | Low |
| **VDR** | Set and Drift | Low |
| **VHW** | Water Speed and Heading | Low |
| **VLW** | Distance Traveled through Water | Low |
| **VPW** | Speed - Measured Parallel to Wind | Low |
| **VWR** | Relative Wind Speed and Angle | Low |
| **VWT** | True Wind Speed and Angle | Low |
| **WCV** | Waypoint Closure Velocity | Low |
| **WNC** | Distance - Waypoint to Waypoint | Low |
| **WPL** | Waypoint Location | Low |
| **XDR** | Transducer Measurement | Low |
| **XTE** | Cross-Track Error | Medium |
| **XTR** | Cross Track Error - Dead Reckoning | Low |
| **ZDA** | Time & Date | Medium |
| **ZFO** | UTC & Time from Origin Waypoint | Low |
| **ZTG** | UTC & Time to Destination Waypoint | Low |

## Detailed Field Implementation

### GGA - Global Positioning System Fix Data

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | UTC Time | ✅ Mandatory | `&str` |
| 2 | Latitude | ✅ Mandatory | `f64` |
| 3 | N/S Indicator | ✅ Mandatory | `char` |
| 4 | Longitude | ✅ Mandatory | `f64` |
| 5 | E/W Indicator | ✅ Mandatory | `char` |
| 6 | Fix Quality | ✅ Mandatory | `u8` |
| 7 | Number of Satellites | ✅ Optional | `Option<u8>` |
| 8 | HDOP | ✅ Optional | `Option<f32>` |
| 9 | Altitude | ✅ Optional | `Option<f32>` |
| 10 | Altitude Units | ✅ Optional | `Option<char>` |
| 11 | Geoid Separation | ✅ Optional | `Option<f32>` |
| 12 | Geoid Units | ✅ Optional | `Option<char>` |
| 13 | Age of Differential | ✅ Optional | `Option<f32>` |
| 14 | Differential Station ID | ✅ Optional | `Option<&str>` |

### RMC - Recommended Minimum Navigation Information

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | UTC Time | ✅ Mandatory | `&str` |
| 2 | Status | ✅ Mandatory | `char` |
| 3 | Latitude | ✅ Mandatory | `f64` |
| 4 | N/S Indicator | ✅ Mandatory | `char` |
| 5 | Longitude | ✅ Mandatory | `f64` |
| 6 | E/W Indicator | ✅ Mandatory | `char` |
| 7 | Speed (knots) | ✅ Mandatory | `f32` |
| 8 | Track Angle | ✅ Mandatory | `f32` |
| 9 | Date | ✅ Mandatory | `&str` |
| 10 | Magnetic Variation | ✅ Optional | `Option<f32>` |
| 11 | E/W Indicator | ✅ Optional | `Option<char>` |
| 12 | Mode Indicator | ❌ Not Implemented | - |

### GSA - GPS DOP and Active Satellites

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | Mode (M/A) | ✅ Mandatory | `char` |
| 2 | Fix Type | ✅ Mandatory | `u8` |
| 3-14 | Satellite IDs | ✅ Optional | `[Option<u8>; 12]` |
| 15 | PDOP | ✅ Optional | `Option<f32>` |
| 16 | HDOP | ✅ Optional | `Option<f32>` |
| 17 | VDOP | ✅ Optional | `Option<f32>` |

### GSV - GPS Satellites in View

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | Number of Messages | ✅ Mandatory | `u8` |
| 2 | Message Number | ✅ Mandatory | `u8` |
| 3 | Satellites in View | ✅ Mandatory | `u8` |
| 4-7 | Satellite 1 Info | ✅ Optional | `Option<SatelliteInfo>` |
| 8-11 | Satellite 2 Info | ✅ Optional | `Option<SatelliteInfo>` |
| 12-15 | Satellite 3 Info | ✅ Optional | `Option<SatelliteInfo>` |
| 16-19 | Satellite 4 Info | ✅ Optional | `Option<SatelliteInfo>` |

Each `SatelliteInfo` contains:
- PRN (Satellite ID) - `Option<u8>`
- Elevation (0-90°) - `Option<u16>`
- Azimuth (0-359°) - `Option<u16>`
- SNR (Signal-to-Noise Ratio) - `Option<u8>`

### GLL - Geographic Position - Latitude/Longitude

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | Latitude | ✅ Mandatory | `f64` |
| 2 | N/S Indicator | ✅ Mandatory | `char` |
| 3 | Longitude | ✅ Mandatory | `f64` |
| 4 | E/W Indicator | ✅ Mandatory | `char` |
| 5 | UTC Time | ✅ Mandatory | `&str` |
| 6 | Status | ✅ Mandatory | `char` |
| 7 | Mode Indicator | ❌ Not Implemented | - |

### VTG - Track Made Good and Ground Speed

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | Track True | ✅ Optional | `Option<f32>` |
| 2 | T Indicator | ✅ Optional | `Option<char>` |
| 3 | Track Magnetic | ✅ Optional | `Option<f32>` |
| 4 | M Indicator | ✅ Optional | `Option<char>` |
| 5 | Speed (knots) | ✅ Optional | `Option<f32>` |
| 6 | N Indicator | ✅ Optional | `Option<char>` |
| 7 | Speed (km/h) | ✅ Optional | `Option<f32>` |
| 8 | K Indicator | ✅ Optional | `Option<char>` |
| 9 | Mode Indicator | ❌ Not Implemented | - |

### GNS - GNSS Fix Data

| Field | Description | Status | Type |
|-------|-------------|--------|------|
| - | Talker ID | ✅ Auto-extracted | `TalkerId` |
| 1 | UTC Time | ✅ Mandatory | `&str` |
| 2 | Latitude | ✅ Mandatory | `f64` |
| 3 | N/S Indicator | ✅ Mandatory | `char` |
| 4 | Longitude | ✅ Mandatory | `f64` |
| 5 | E/W Indicator | ✅ Mandatory | `char` |
| 6 | Mode Indicator | ✅ Mandatory | `&str` |
| 7 | Number of Satellites | ✅ Mandatory | `u8` |
| 8 | HDOP | ✅ Optional | `Option<f32>` |
| 9 | Altitude | ✅ Optional | `Option<f32>` |
| 10 | Geoid Separation | ✅ Optional | `Option<f32>` |
| 11 | Age of Differential | ✅ Optional | `Option<f32>` |
| 12 | Differential Station ID | ✅ Optional | `Option<&str>` |
| 13 | Nav Status | ✅ Optional | `Option<char>` |

## Protocol Features

### Supported Features

| Feature | Status | Notes |
|---------|--------|-------|
| Sentence parsing | ✅ Supported | Character-by-character stream parsing |
| Checksum validation | ✅ Supported | Automatic checksum verification |
| Field extraction | ✅ Supported | Type-safe field access |
| Message storage | ✅ Supported | Last message per type cached |
| Timestamp tracking | ✅ Supported | Internal timestamp counter |
| `no_std` compatibility | ✅ Supported | Works in embedded environments |
| Talker ID support | ✅ Supported | GP, GN, GL, etc. |

### Not Supported Features

| Feature | Status | Priority |
|---------|--------|----------|
| Multi-sentence messages | ❌ Not Supported | Medium |
| AIS messages | ❌ Not Supported | Low |
| Proprietary sentences | ❌ Not Supported | Low |
| Query sentences | ❌ Not Supported | Low |
| NMEA 0183 v4.x features | ❌ Not Supported | Medium |
| Sentence generation | ❌ Not Supported | Medium |

## Standard Compliance Notes

### Parsing Behavior

1. **Mandatory Fields**: If any mandatory field is missing or cannot be parsed, the message extraction method returns `None`
2. **Optional Fields**: Optional fields return `None` if missing or unparseable
3. **Field Validation**: Basic type validation is performed during parsing
4. **Checksum**: Sentences with invalid checksums are rejected
5. **Buffer Limits**: Maximum sentence length is 82 characters (per NMEA 0183 spec)
6. **Field Limits**: Maximum 20 fields per sentence

### Known Limitations

1. **Mode Indicator**: NMEA 0183 v3.0+ mode indicator (A/D/E/N) not implemented for RMC, GLL, and VTG
2. **Multi-part Messages**: Some message types (like GSV) may span multiple sentences; the library parses each sentence independently
3. **Coordinate Formats**: Coordinates are provided in NMEA format (DDMM.MMMM); no automatic conversion to decimal degrees
4. **Time Formats**: Time and date are provided as strings; no automatic parsing to datetime structures

## Testing Coverage

The library includes comprehensive unit tests for:
- ✅ All supported message types
- ✅ Mandatory field validation
- ✅ Optional field handling
- ✅ Empty field handling
- ✅ Invalid sentence rejection
- ✅ Checksum validation
- ✅ Stream parsing with multiple messages
- ✅ Message storage and retrieval

## Future Enhancements

Priority items for future development:

1. **High Priority**
   - Mode indicator support (v3.0+)
   - ZDA message support (date/time)

2. **Medium Priority**
   - GBS message support (satellite fault detection)
   - GST message support (error statistics)
   - DTM message support (datum reference)
   - RMB message support (waypoint navigation)
   - XTE message support (cross-track error)
   - Sentence generation/encoding

3. **Low Priority**
   - Additional message types as needed
   - Proprietary sentence handling
   - Multi-sentence message handling

## References

- NMEA 0183 Standard v2.30
- NMEA 0183 Standard v3.01
- NMEA 0183 Standard v4.11
- [NMEA Revealed](https://gpsd.gitlab.io/gpsd/NMEA.html) - ESR's NMEA documentation

## Version History

- **v0.1.0** (Current)
  - Initial implementation
  - Support for GGA, RMC, GSA, GSV, GLL, VTG messages
  - Character stream parsing
  - `no_std` compatibility
