use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LogEvent {
    timestamp: u64,
    level: String,
    message: String,
    #[serde(flatten)]
    extra: serde_json::Value,
}

fn parse_json_log(line: &str) -> Result<LogEvent, serde_json::Error> {
    serde_json::from_str(line)
}

fn main() {
    println!("Simple Log Parser Example");
    println!("Enter JSON logs (one per line, Ctrl+D to end):\n");

    let stdin = io::stdin();
    let mut successful = 0;
    let mut failed = 0;

    for line in stdin.lock().lines() {
        match line {
            Ok(log_line) => {
                match parse_json_log(&log_line) {
                    Ok(event) => {
                        println!("✓ Parsed: {} - {}", event.level, event.message);
                        successful += 1;
                    }
                    Err(e) => {
                        eprintln!("✗ Parse error: {}", e);
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("✗ Read error: {}", e);
                break;
            }
        }
    }

    println!("\n--- Summary ---");
    println!("Successful: {}", successful);
    println!("Failed: {}", failed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_json() {
        let json = r#"{"timestamp":1704067200,"level":"INFO","message":"Test message"}"#;
        let result = parse_json_log(json);
        assert!(result.is_ok());
        
        let event = result.unwrap();
        assert_eq!(event.level, "INFO");
        assert_eq!(event.message, "Test message");
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{"timestamp":"invalid","level":"INFO"}"#;
        let result = parse_json_log(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_extra_fields() {
        let json = r#"{"timestamp":1704067200,"level":"ERROR","message":"Failed","user":"admin","ip":"192.168.1.1"}"#;
        let result = parse_json_log(json);
        assert!(result.is_ok());
        
        let event = result.unwrap();
        assert_eq!(event.level, "ERROR");
        assert!(event.extra.get("user").is_some());
    }
}
