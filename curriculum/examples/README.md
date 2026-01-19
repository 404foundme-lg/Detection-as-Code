# Detection-as-Code Examples

This directory contains practical examples demonstrating Detection-as-Code concepts in Rust.

## Examples

### 1. Simple Log Parser (`simple-parser/`)
Basic JSON and syslog parser implementation.

**Key concepts:**
- Serde deserialization
- Error handling
- Pattern matching

### 2. Threshold Detector (`threshold-detector/`)
Detects events exceeding thresholds within time windows.

**Key concepts:**
- Time-based windowing
- HashMap for tracking
- Event correlation

### 3. Sigma Rule Engine (`sigma-engine/`)
Mini Sigma rule parser and evaluator.

**Key concepts:**
- YAML parsing
- Rule evaluation logic
- Backend translation

### 4. Concurrent Log Processor (`concurrent-processor/`)
Process multiple log sources in parallel.

**Key concepts:**
- Multi-threading with Rayon
- Shared state with Arc/Mutex
- Channel-based communication

### 5. Detection Pipeline (`detection-pipeline/`)
Complete end-to-end detection pipeline.

**Key concepts:**
- Log ingestion
- Rule evaluation
- Alert generation
- Output formatting

## Running Examples

Each example is a standalone Rust project:

```bash
cd examples/simple-parser
cargo build
cargo run
```

## Testing Examples

```bash
cd examples/simple-parser
cargo test
```

## Example Structure

Each example includes:
- `src/main.rs` - Main implementation
- `Cargo.toml` - Dependencies
- `README.md` - Specific documentation
- `tests/` - Unit and integration tests
- `data/` - Sample input data
