# Module 4: Data Parsing and Processing

## Learning Objectives

- Master Serde for JSON/YAML parsing
- Build custom deserializers for complex log formats
- Implement schema validation
- Optimize parsing performance
- Handle malformed data gracefully

## Key Topics

### 1. Serde Fundamentals

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct SecurityLog {
    #[serde(rename = "eventTime")]
    event_time: String,
    
    #[serde(rename = "eventType")]
    event_type: String,
    
    #[serde(default)]
    severity: u8,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

fn parse_security_log(json: &str) -> Result<SecurityLog, serde_json::Error> {
    serde_json::from_str(json)
}
```

### 2. Custom Deserializers

```rust
use serde::de::{self, Deserializer};

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(de::Error::custom)
}

#[derive(Deserialize)]
struct Event {
    #[serde(deserialize_with = "deserialize_timestamp")]
    timestamp: u64,
}
```

### 3. Performance Optimization

- Use `serde_json::from_slice` for better performance
- Stream processing with `serde_json::Deserializer`
- Avoid unnecessary allocations
- Benchmark with `criterion`

### 4. Error Recovery

```rust
fn parse_log_batch(logs: &[&str]) -> (Vec<SecurityLog>, Vec<String>) {
    let mut successful = Vec::new();
    let mut errors = Vec::new();
    
    for (i, log) in logs.iter().enumerate() {
        match parse_security_log(log) {
            Ok(parsed) => successful.push(parsed),
            Err(e) => errors.push(format!("Line {}: {}", i, e)),
        }
    }
    
    (successful, errors)
}
```

## Lab Exercises

1. **Parse CloudTrail Logs:** Build parser for AWS CloudTrail JSON
2. **Custom CEF Parser:** Implement full CEF format parser
3. **Performance Benchmark:** Optimize parsing 100K events/sec
4. **Streaming Parser:** Process logs without loading entire file

## Resources

- [Serde Documentation](https://serde.rs/)
- [serde_json Performance Guide](https://github.com/serde-rs/json)

**Next:** [Module 5: Pattern Matching and Detection Logic](../module-05-pattern-matching/README.md)
