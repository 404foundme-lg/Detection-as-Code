# Simple Log Parser

A basic JSON log parser demonstrating parsing and error handling in Rust.

## Features

- Parse JSON logs into structured format
- Handle parsing errors gracefully
- Support arbitrary extra fields
- Command-line interface

## Building

```bash
cargo build --release
```

## Running

```bash
# Interactive mode
cargo run

# From file
cat data/sample.log | cargo run

# From stdin
echo '{"timestamp":1704067200,"level":"INFO","message":"Hello"}' | cargo run
```

## Testing

```bash
cargo test
```

## Example Usage

```bash
$ cargo run
Simple Log Parser Example
Enter JSON logs (one per line, Ctrl+D to end):

{"timestamp":1704067200,"level":"INFO","message":"Application started"}
✓ Parsed: INFO - Application started

{"timestamp":1704067201,"level":"ERROR","message":"Connection failed","error":"timeout"}
✓ Parsed: ERROR - Connection failed

invalid json
✗ Parse error: expected value at line 1 column 1

--- Summary ---
Successful: 2
Failed: 1
```

## Learning Objectives

This example demonstrates:
- Serde deserialization
- Error handling with Result
- Pattern matching
- Command-line I/O
- Unit testing

## Next Steps

Try enhancing this parser:
1. Add support for CEF format
2. Implement filtering by log level
3. Add performance benchmarks
4. Stream processing from files
