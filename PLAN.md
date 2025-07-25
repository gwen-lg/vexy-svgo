# Issue #201: SVGO Default Plugin Parity - Implementation of Missing Plugins

## Project Overview
This plan focuses on implementing the remaining 3 SVGO plugins needed to achieve full default plugin parity:
1. **cleanupAttrs** - Cleans up attributes from newlines, trailing and repeating spaces
2. **cleanupNumericValues** - Rounds numeric values to fixed precision, removes default px units
3. **cleanupEnableBackground** - Removes or cleans up enable-background attribute when possible

## Technical Architecture Decisions
- Follow existing plugin architecture patterns in vexy_svgo
- Use visitor pattern for AST traversal as established in other plugins
- Maintain API compatibility with SVGO plugin parameters
- Implement comprehensive testing for each plugin

## Phase 1: Plugin Implementation Details

### 1.1 cleanupAttrs Plugin
- **Objective:** Clean up attribute values by removing unnecessary whitespace
- **Implementation Steps:**
  1. Create `crates/plugin-sdk/src/plugins/cleanup_attrs.rs`
  2. Implement attribute value normalization:
     - Remove newlines from attribute values
     - Trim leading and trailing whitespace
     - Collapse multiple spaces into single space
  3. Support plugin parameters:
     - `newlines`: boolean (default: true) - remove newlines
     - `trim`: boolean (default: true) - trim whitespace
     - `spaces`: boolean (default: true) - normalize spaces
  4. Add visitor pattern implementation to traverse all elements

### 1.2 cleanupNumericValues Plugin
- **Objective:** Optimize numeric values in attributes and styles
- **Implementation Steps:**
  1. Create `crates/plugin-sdk/src/plugins/cleanup_numeric_values.rs`
  2. Implement numeric value optimization:
     - Round floating point values to specified precision
     - Remove leading zeros (0.5 → .5)
     - Remove trailing zeros (1.0 → 1)
     - Remove default "px" units where applicable
     - Convert between units when beneficial
  3. Support plugin parameters:
     - `floatPrecision`: u8 (default: 3) - decimal places
     - `leadingZero`: boolean (default: true) - keep leading zeros
     - `defaultPx`: boolean (default: true) - remove default px
     - `convertToPx`: boolean (default: true) - convert units to px
  4. Handle both attribute values and CSS property values

### 1.3 cleanupEnableBackground Plugin
- **Objective:** Remove or optimize enable-background attribute
- **Implementation Steps:**
  1. Create `crates/plugin-sdk/src/plugins/cleanup_enable_background.rs`
  2. Implement enable-background cleanup logic:
     - Remove enable-background="new" (the default value)
     - Remove enable-background when it has no effect
     - Optimize coordinate values in enable-background
  3. Check for filter effects that might use BackgroundImage
  4. Preserve enable-background only when necessary

## Phase 2: Implementation Strategy

### 2.1 Code Structure and Patterns
- **Objective:** Follow established patterns for consistency
- **Implementation Steps:**
  1. Use existing plugin structure as template (e.g., `remove_empty_attrs.rs`)
  2. Implement Plugin trait with metadata and optimize methods
  3. Use visitor pattern for element traversal
  4. Follow error handling patterns with Result types
  5. Use parameter structs with serde for configuration

### 2.2 Plugin Registration
- **Objective:** Integrate plugins into the system
- **Implementation Steps:**
  1. Add plugin imports to `crates/plugin-sdk/src/plugins/mod.rs`
  2. Uncomment plugin registration in `registry.rs`
  3. Ensure plugins are included in default preset
  4. Verify plugin ordering matches SVGO

### 2.3 Testing Strategy
- **Objective:** Comprehensive testing for reliability
- **Implementation Steps:**
  1. Unit tests for each plugin function
  2. Integration tests with sample SVG files
  3. Edge case testing (empty values, malformed input)
  4. Compatibility tests comparing with SVGO output
  5. Performance benchmarks for optimization

## Phase 3: Specific Implementation Tasks

### 3.1 cleanupAttrs Implementation Details
```rust
// Key functions to implement:
- clean_attribute_value(value: &str, params: &CleanupAttrsParams) -> String
- should_cleanup_attribute(attr_name: &str) -> bool
- normalize_whitespace(s: &str) -> String
```

### 3.2 cleanupNumericValues Implementation Details
```rust
// Key functions to implement:
- round_numeric_value(value: f64, precision: u8) -> String
- parse_numeric_with_unit(value: &str) -> Option<(f64, &str)>
- optimize_numeric_string(value: &str, params: &CleanupNumericValuesParams) -> String
- is_default_unit(value: &str, unit: &str, context: &str) -> bool
```

### 3.3 cleanupEnableBackground Implementation Details
```rust
// Key functions to implement:
- parse_enable_background(value: &str) -> Option<EnableBackground>
- is_enable_background_used(doc: &Document) -> bool
- optimize_enable_background(value: &str) -> Option<String>
```

## Phase 4: Integration and Verification

### 4.1 Build and Test
- **Objective:** Ensure all implementations work correctly
- **Implementation Steps:**
  1. Run `./build.sh` to compile and test
  2. Fix any compilation errors
  3. Ensure all existing tests still pass
  4. Verify new plugin tests pass

### 4.2 Plugin Integration Testing
- **Objective:** Test plugins work with real SVG files
- **Implementation Steps:**
  1. Test with simple SVG files
  2. Test with complex SVG files from testdata/
  3. Compare output with expected results
  4. Verify no SVG corruption occurs

## Phase 5: Final Steps

### 5.1 Documentation Updates
- **Objective:** Update all relevant documentation
- **Implementation Steps:**
  1. Update CHANGELOG.md with new plugins
  2. Update plugin reference documentation
  3. Update TODO.md to mark completion
  4. Update WORK.md with implementation notes

### 5.2 Validation
- **Objective:** Final validation of implementation
- **Implementation Steps:**
  1. Run full test suite
  2. Verify 100% plugin parity achieved
  3. Test CLI with --show-plugins
  4. Confirm optimization improvements

## Success Criteria
1. **Plugin Parity:** All default plugins from SVGO are implemented and enabled by default in Vexy SVGO
2. **Configuration Parity:** Default plugin configurations match SVGO exactly
3. **Optimization Parity:** File size optimization results differ by no more than 1% on average
4. **Performance:** Vexy SVGO performs comparably or better than SVGO
5. **Quality:** Optimized SVGs maintain visual and functional equivalence
6. **Testing:** Automated tests prevent future parity regressions

## Risk Assessment and Mitigation
- **Risk:** Complex SVGO plugins may be difficult to port
  - **Mitigation:** Prioritize most impactful plugins first, document any temporary limitations
- **Risk:** Performance optimization may compromise parity
  - **Mitigation:** Comprehensive testing at each optimization step
- **Risk:** SVGO updates may break parity
  - **Mitigation:** Regular synchronization with SVGO updates, automated parity monitoring

## Future Considerations
- Establish process for maintaining parity with future SVGO updates
- Consider contributing improvements back to SVGO where appropriate
- Plan for extending parity to non-default plugin configurations