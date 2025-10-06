# GitHub Copilot Instructions for rustedbytes-nmea

This document provides guidelines for GitHub Copilot when working on the rustedbytes-nmea project.

## Project Overview

rustedbytes-nmea is a Rust `no_std` library for parsing NMEA 0183 messages from GNSS receivers. The library is designed for embedded systems and focuses on:
- Character-by-character stream parsing
- Support for common NMEA message types (GGA, RMC, GSA, GSV, GLL, VTG)
- Zero-allocation, fixed-size buffers
- Comprehensive error handling
- NMEA 0183 standard compliance

## Coding Standards

### General Rust Guidelines

1. **Edition**: Use Rust 2021 edition
2. **no_std Compatibility**: All code must be `no_std` compatible
   - Do not use `std::` imports
   - Use `core::` imports instead
   - Avoid heap allocations (no `Vec`, `String`, `Box`, etc.)
   - Use fixed-size arrays and buffers
3. **Safety**: Prefer safe Rust; use `unsafe` only when absolutely necessary and document it thoroughly
4. **Error Handling**: Use `Option` and `Result` types appropriately
5. **Documentation**: All public APIs must have doc comments

### Code Style

1. **Formatting**: Use `rustfmt` with default settings
2. **Naming Conventions**:
   - `snake_case` for functions, variables, and modules
   - `CamelCase` for types, traits, and enums
   - `SCREAMING_SNAKE_CASE` for constants
3. **Line Length**: Aim for 100 characters, but be flexible for readability
4. **Comments**: Use `//` for single-line comments, `///` for doc comments

### Project-Specific Conventions

1. **Message Parsing**:
   - All parsers must validate mandatory fields
   - Optional fields should use `Option<T>`
   - Return `None` if any mandatory field is missing or invalid
2. **Field Limits**:
   - Maximum sentence length: 82 characters (NMEA 0183 spec)
   - Maximum fields per message: 20
   - Use `MAX_FIELDS` constant for array sizes
3. **Type Safety**:
   - Use appropriate numeric types (u8, u16, f32, f64)
   - Validate ranges where applicable (e.g., azimuth 0-359°)
4. **Buffer Management**:
   - All buffers must be fixed-size
   - Use array-based storage, not dynamic allocations

## NMEA 0183 Compliance

### Supported Message Types

When working with NMEA messages, refer to the [NMEA-183-COMPLIANCE.md](../NMEA-183-COMPLIANCE.md) document for:
- Supported message types and their fields
- Field types and validation rules
- Mandatory vs. optional fields
- Known limitations

### Adding New Message Types

When adding support for a new NMEA message type:

1. **Update MessageType Enum**: Add the new message type to the `MessageType` enum
2. **Create Data Structure**: Define a new `*Data` struct with appropriate fields
   - Mark mandatory fields clearly
   - Use `Option<T>` for optional fields
3. **Implement Parser**: Add an `as_*()` method to `NmeaMessage`
   - Validate all mandatory fields
   - Return `None` if any mandatory field is missing/invalid
4. **Add Tests**: Create comprehensive unit tests
   - Test with valid complete messages
   - Test with missing mandatory fields
   - Test with empty optional fields
   - Test with partial data
5. **Update Documentation**:
   - Update README.md with API documentation
   - Update NMEA-183-COMPLIANCE.md with field implementation details
   - Add usage examples if needed

## Testing Requirements

### Test Coverage

All code changes must include tests. The test suite should cover:

1. **Happy Path**: Valid, complete messages
2. **Edge Cases**:
   - Empty optional fields
   - Missing mandatory fields
   - Partial data in array fields
   - Boundary values (max/min)
3. **Error Cases**:
   - Invalid checksums
   - Malformed sentences
   - Wrong message type extraction
4. **Stream Parsing**:
   - Multiple messages in sequence
   - Partial sentences
   - Message overwriting

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Test Naming Convention

- Descriptive names: `test_<feature>_<scenario>`
- Examples:
  - `test_gga_parameters`
  - `test_gga_with_empty_fields`
  - `test_gga_missing_time`

## Documentation Requirements

### Code Documentation

1. **Public APIs**: Must have `///` doc comments with:
   - Brief description
   - Parameter descriptions
   - Return value description
   - Example usage (if applicable)
2. **Modules**: Should have module-level documentation
3. **Complex Logic**: Use inline comments to explain non-obvious code

### External Documentation

When making changes, update:

1. **README.md**: 
   - API changes
   - Usage examples
   - Feature additions
2. **NMEA-183-COMPLIANCE.md**:
   - New message types
   - Field implementations
   - Compliance status updates

## Dependencies

### Current Policy

- **Zero Dependencies**: The library currently has no dependencies
- **Rationale**: Maintain `no_std` compatibility and minimal footprint
- **Adding Dependencies**: Should be avoided unless absolutely necessary
  - Must be `no_std` compatible
  - Must have MIT or compatible license
  - Must be well-maintained and widely used

## Performance Considerations

1. **Memory Usage**:
   - Minimize stack usage
   - Use fixed-size buffers
   - Avoid redundant copies
2. **Parsing Efficiency**:
   - Character-by-character parsing is intentional (stream-based)
   - Minimize string operations
   - Use integer parsing where possible
3. **Embedded Systems**:
   - Consider resource-constrained environments
   - No panic in normal operation
   - Predictable behavior

## Common Patterns

### Parsing Optional Fields

```rust
// Numeric optional field
let value = if field.as_str().unwrap_or("").is_empty() {
    None
} else {
    field.as_str()?.parse().ok()
};

// Character optional field
let value = field.as_str()
    .filter(|s| !s.is_empty())
    .and_then(|s| s.chars().next());
```

### Validating Mandatory Fields

```rust
// Check all mandatory fields exist and are valid
let time = msg.fields[0].as_ref()?.as_str()?;
let status = msg.fields[1].as_ref()?.as_str()?.chars().next()?;
let latitude = msg.fields[2].as_ref()?.as_str()?.parse().ok()?;
// ... etc
```

### Array Field Parsing

```rust
// Parse variable-length array (e.g., satellite IDs)
let mut satellite_ids = [None; 12];
for i in 0..12 {
    if let Some(ref field) = msg.fields[2 + i] {
        if let Some(s) = field.as_str() {
            if !s.is_empty() {
                satellite_ids[i] = s.parse().ok();
            }
        }
    }
}
```

## Version Management

### Current Version

- **v0.1.0**: Initial release

### Versioning Policy

Follow Semantic Versioning (SemVer):
- **Major** (x.0.0): Breaking API changes
- **Minor** (0.x.0): New features, backward compatible
- **Patch** (0.0.x): Bug fixes, backward compatible

### Release Checklist

When preparing a release:
1. Update version in `Cargo.toml`
2. Update version history in `NMEA-183-COMPLIANCE.md`
3. Update `README.md` if needed
4. Run full test suite
5. Update CHANGELOG (if exists)

## License

All contributions must be compatible with the MIT License.

## Common Pitfalls to Avoid

1. **Don't use std library**: Remember `no_std` requirement
2. **Don't use heap allocations**: No `Vec`, `String`, `Box`
3. **Don't panic**: Use `Option`/`Result` for error handling
4. **Don't ignore empty fields**: NMEA messages can have empty fields
5. **Don't assume field count**: Always check field existence before access
6. **Don't modify existing tests**: Unless fixing a bug; add new tests instead
7. **Don't break NMEA compliance**: Follow the standard specifications

## Additional Resources

- [NMEA 0183 Standard Documentation](https://gpsd.gitlab.io/gpsd/NMEA.html)
- [Rust no_std Guide](https://docs.rust-embedded.org/book/intro/no-std.html)
- Project Issues: https://github.com/mad4j/rustedbytes-nmea/issues
- Project Repository: https://github.com/mad4j/rustedbytes-nmea

## Questions or Clarifications

When in doubt:
1. Check existing code patterns in `src/lib.rs`
2. Refer to `NMEA-183-COMPLIANCE.md` for message specifications
3. Look at existing tests for examples
4. Maintain consistency with the existing codebase
