# Module 2: Rust Fundamentals for Security

## Learning Objectives

By the end of this module, you will:

- Master Rust's ownership, borrowing, and lifetime concepts
- Write safe, concurrent code for detection systems
- Handle errors elegantly in security tooling
- Leverage Rust's pattern matching for detection logic
- Apply security-focused Rust idioms
- Build efficient data structures for log processing

## Table of Contents

1. [Why Rust for Detection Engineering](#1-why-rust-for-detection-engineering)
2. [Ownership and Borrowing](#2-ownership-and-borrowing)
3. [Error Handling](#3-error-handling)
4. [Collections and Iterators](#4-collections-and-iterators)
5. [Pattern Matching](#5-pattern-matching)
6. [Concurrency Basics](#6-concurrency-basics)
7. [Security-Focused Rust](#7-security-focused-rust)
8. [Lab Exercises](#8-lab-exercises)

---

## 1. Why Rust for Detection Engineering

### The Case for Rust

**Performance Benefits:**
- Zero-cost abstractions
- No garbage collection pauses
- Memory efficient (crucial for high-volume log processing)
- Compiled to native code
- Predictable performance characteristics

**Safety Benefits:**
- Memory safety without garbage collection
- Thread safety enforced at compile time
- No null pointer exceptions
- No data races
- Preventing common security vulnerabilities

**Productivity Benefits:**
- Cargo package manager and build system
- Excellent error messages
- Rich ecosystem of crates
- Strong type system catches bugs early
- Great tooling (rustfmt, clippy, rust-analyzer)

### Real-World Performance Comparison

| Task | Python | Go | Rust |
|------|--------|-----|------|
| Parse 1M JSON logs | 12.5s | 2.1s | 0.8s |
| Regex matching 100K events | 8.2s | 1.5s | 0.6s |
| Concurrent processing | 15.3s | 3.2s | 1.2s |
| Memory usage (10M events) | 2.4GB | 850MB | 320MB |

*Benchmarks are illustrative; actual performance varies by implementation*

### Detection Engineering Use Cases

**Ideal for Rust:**
- High-throughput log parsers
- Real-time detection engines
- Custom protocol analyzers
- Performance-critical correlation engines
- CLI tools for security operations
- Agent/sensor software

**Consider alternatives when:**
- Rapid prototyping needed (use Python)
- Simple glue scripts (use Bash/Python)
- Heavy data science/ML (use Python with Rust extensions)
- Team lacks Rust expertise (evaluate learning curve)

---

## 2. Ownership and Borrowing

### The Ownership Model

Rust's ownership system ensures memory safety at compile time without runtime overhead.

**Three Rules of Ownership:**
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

#### Example: Processing Security Events

```rust
#[derive(Debug)]
struct SecurityEvent {
    timestamp: u64,
    source_ip: String,
    event_type: String,
    severity: u8,
}

fn main() {
    // Event is owned by 'event'
    let event = SecurityEvent {
        timestamp: 1704067200,
        source_ip: "192.168.1.100".to_string(),
        event_type: "failed_login".to_string(),
        severity: 3,
    };
    
    // Ownership transferred to 'processed_event'
    let processed_event = process_event(event);
    
    // Error: 'event' no longer valid here
    // println!("{:?}", event); // Would not compile!
    
    println!("{:?}", processed_event); // This works
}

fn process_event(evt: SecurityEvent) -> SecurityEvent {
    // evt is owned by this function
    println!("Processing event: {}", evt.event_type);
    evt // Ownership returned to caller
}
```

### Borrowing: References

Instead of transferring ownership, you can borrow values:

```rust
fn analyze_event(event: &SecurityEvent) {
    // Immutable borrow - can read but not modify
    println!("Analyzing {}: severity {}", 
             event.event_type, event.severity);
}

fn enrich_event(event: &mut SecurityEvent) {
    // Mutable borrow - can modify
    event.severity = calculate_severity(event);
}

fn main() {
    let mut event = SecurityEvent {
        timestamp: 1704067200,
        source_ip: "192.168.1.100".to_string(),
        event_type: "failed_login".to_string(),
        severity: 3,
    };
    
    analyze_event(&event);      // Immutable borrow
    enrich_event(&mut event);   // Mutable borrow
    analyze_event(&event);      // Can borrow again
    
    println!("Final event: {:?}", event); // Still owned by main
}

fn calculate_severity(event: &SecurityEvent) -> u8 {
    // Simple severity calculation
    match event.event_type.as_str() {
        "failed_login" => 3,
        "privilege_escalation" => 9,
        "data_exfiltration" => 10,
        _ => 1,
    }
}
```

### Borrowing Rules

**The Borrow Checker enforces:**
1. You can have either:
   - One mutable reference, OR
   - Any number of immutable references
2. References must always be valid

**Why this matters for security:**
- Prevents data races at compile time
- No use-after-free vulnerabilities
- No iterator invalidation bugs
- Thread-safe by default

#### Practical Example: Event Correlation

```rust
use std::collections::HashMap;

struct CorrelationEngine {
    events: Vec<SecurityEvent>,
    ip_counts: HashMap<String, u32>,
}

impl CorrelationEngine {
    fn new() -> Self {
        CorrelationEngine {
            events: Vec::new(),
            ip_counts: HashMap::new(),
        }
    }
    
    fn add_event(&mut self, event: SecurityEvent) {
        // Count events by IP
        *self.ip_counts
            .entry(event.source_ip.clone())
            .or_insert(0) += 1;
        
        self.events.push(event);
    }
    
    fn get_suspicious_ips(&self) -> Vec<&String> {
        // Immutable borrow to find IPs with >10 events
        self.ip_counts
            .iter()
            .filter(|(_, &count)| count > 10)
            .map(|(ip, _)| ip)
            .collect()
    }
}
```

### Lifetimes

Lifetimes ensure references are valid for as long as they're used:

```rust
// Lifetime annotations tell the compiler how long references live
fn get_highest_severity<'a>(
    event1: &'a SecurityEvent,
    event2: &'a SecurityEvent
) -> &'a SecurityEvent {
    if event1.severity > event2.severity {
        event1
    } else {
        event2
    }
}

// Usage
fn main() {
    let event1 = SecurityEvent {
        timestamp: 1704067200,
        source_ip: "10.0.0.1".to_string(),
        event_type: "malware".to_string(),
        severity: 8,
    };
    
    let event2 = SecurityEvent {
        timestamp: 1704067201,
        source_ip: "10.0.0.2".to_string(),
        event_type: "failed_login".to_string(),
        severity: 3,
    };
    
    let highest = get_highest_severity(&event1, &event2);
    println!("Highest severity: {} ({})", 
             highest.severity, highest.event_type);
}
```

---

## 3. Error Handling

Rust doesn't have exceptions. Instead, it uses `Result<T, E>` and `Option<T>`.

### The Result Type

```rust
use std::fs::File;
use std::io::{self, Read};

// Result type for operations that can fail
fn read_log_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // '?' propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_log_file("/var/log/auth.log") {
        Ok(contents) => println!("Read {} bytes", contents.len()),
        Err(e) => eprintln!("Error reading log: {}", e),
    }
}
```

### Custom Error Types

For detection systems, create domain-specific errors:

```rust
use std::fmt;

#[derive(Debug)]
enum DetectionError {
    ParseError(String),
    InvalidTimestamp,
    MissingField(String),
    CorrelationTimeout,
}

impl fmt::Display for DetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DetectionError::ParseError(msg) => 
                write!(f, "Parse error: {}", msg),
            DetectionError::InvalidTimestamp => 
                write!(f, "Invalid timestamp format"),
            DetectionError::MissingField(field) => 
                write!(f, "Missing required field: {}", field),
            DetectionError::CorrelationTimeout => 
                write!(f, "Correlation window timeout"),
        }
    }
}

impl std::error::Error for DetectionError {}

// Using custom error
fn parse_event(json: &str) -> Result<SecurityEvent, DetectionError> {
    // Parsing logic here
    if json.is_empty() {
        return Err(DetectionError::ParseError(
            "Empty input".to_string()
        ));
    }
    
    // ... more parsing ...
    
    Ok(SecurityEvent {
        timestamp: 1704067200,
        source_ip: "192.168.1.1".to_string(),
        event_type: "login".to_string(),
        severity: 1,
    })
}
```

### The ? Operator and Error Propagation

```rust
use std::io;

fn process_log_pipeline(path: &str) -> Result<Vec<SecurityEvent>, Box<dyn std::error::Error>> {
    // ? automatically propagates errors up the call stack
    let contents = read_log_file(path)?;
    let events = parse_log_contents(&contents)?;
    let filtered = filter_high_severity(events)?;
    Ok(filtered)
}

fn parse_log_contents(contents: &str) -> Result<Vec<SecurityEvent>, DetectionError> {
    let mut events = Vec::new();
    
    for line in contents.lines() {
        let event = parse_event(line)?; // Propagates DetectionError
        events.push(event);
    }
    
    Ok(events)
}

fn filter_high_severity(events: Vec<SecurityEvent>) -> Result<Vec<SecurityEvent>, DetectionError> {
    Ok(events.into_iter()
        .filter(|e| e.severity >= 7)
        .collect())
}
```

### Option Type for Nullable Values

```rust
use std::collections::HashMap;

struct ThreatIntelligence {
    known_bad_ips: HashMap<String, String>,
}

impl ThreatIntelligence {
    fn lookup_ip(&self, ip: &str) -> Option<&String> {
        self.known_bad_ips.get(ip)
    }
    
    fn enrich_event(&self, event: &mut SecurityEvent) {
        if let Some(threat_info) = self.lookup_ip(&event.source_ip) {
            println!("Known threat: {}", threat_info);
            event.severity = 10; // Max severity for known bad
        }
    }
}
```

---

## 4. Collections and Iterators

### Essential Collections for Detection

#### Vec<T> - Dynamic Arrays

```rust
fn collect_suspicious_events() -> Vec<SecurityEvent> {
    let mut events = Vec::new();
    
    // Add events
    events.push(SecurityEvent {
        timestamp: 1704067200,
        source_ip: "192.168.1.100".to_string(),
        event_type: "failed_login".to_string(),
        severity: 3,
    });
    
    // Pre-allocate capacity for performance
    let mut large_batch = Vec::with_capacity(10000);
    
    events
}
```

#### HashMap<K, V> - Key-Value Storage

```rust
use std::collections::HashMap;

fn build_ip_reputation() -> HashMap<String, u8> {
    let mut reputation = HashMap::new();
    
    reputation.insert("192.168.1.100".to_string(), 50);  // Suspicious
    reputation.insert("10.0.0.1".to_string(), 100);      // Trusted
    reputation.insert("203.0.113.42".to_string(), 0);    // Malicious
    
    reputation
}

fn calculate_risk_score(event: &SecurityEvent, reputation: &HashMap<String, u8>) -> u8 {
    let base_score = event.severity;
    let rep_score = reputation.get(&event.source_ip).unwrap_or(&50);
    
    // Lower reputation = higher risk
    base_score.saturating_add(100 - rep_score)
}
```

#### HashSet<T> - Unique Collections

```rust
use std::collections::HashSet;

fn build_whitelist() -> HashSet<String> {
    let mut whitelist = HashSet::new();
    whitelist.insert("10.0.0.1".to_string());
    whitelist.insert("10.0.0.2".to_string());
    whitelist
}

fn is_whitelisted(ip: &str, whitelist: &HashSet<String>) -> bool {
    whitelist.contains(ip)
}
```

### Powerful Iterator Patterns

Iterators are zero-cost abstractions in Rust - they're as fast as hand-written loops!

```rust
fn analyze_events(events: Vec<SecurityEvent>) {
    // Chain multiple operations efficiently
    let high_severity_ips: Vec<String> = events
        .iter()
        .filter(|e| e.severity >= 7)                    // Only high severity
        .filter(|e| e.event_type == "failed_login")     // Specific type
        .map(|e| e.source_ip.clone())                   // Extract IPs
        .collect();                                     // Collect to Vec
    
    println!("High severity login failures: {:?}", high_severity_ips);
}

fn count_events_by_type(events: &[SecurityEvent]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for event in events {
        *counts.entry(event.event_type.clone()).or_insert(0) += 1;
    }
    
    counts
}

// More functional approach
fn count_events_functional(events: &[SecurityEvent]) -> HashMap<String, usize> {
    events.iter()
        .fold(HashMap::new(), |mut acc, event| {
            *acc.entry(event.event_type.clone()).or_insert(0) += 1;
            acc
        })
}
```

### Performance Tips

```rust
fn efficient_filtering(events: Vec<SecurityEvent>) -> Vec<SecurityEvent> {
    // Good: consume iterator, no cloning
    events.into_iter()
        .filter(|e| e.severity >= 5)
        .collect()
}

fn inefficient_filtering(events: &Vec<SecurityEvent>) -> Vec<SecurityEvent> {
    // Less efficient: requires cloning
    events.iter()
        .filter(|e| e.severity >= 5)
        .cloned()
        .collect()
}
```

---

## 5. Pattern Matching

Pattern matching is a powerful feature for detection logic.

### Basic Match Expressions

```rust
fn classify_event(event: &SecurityEvent) -> &str {
    match event.severity {
        0..=2 => "info",
        3..=5 => "warning",
        6..=8 => "high",
        9..=10 => "critical",
        _ => "unknown",
    }
}

fn handle_event_type(event_type: &str) {
    match event_type {
        "login_success" => println!("User logged in"),
        "login_failure" => println!("Login attempt failed"),
        "privilege_escalation" => println!("ALERT: Privilege escalation!"),
        _ => println!("Unknown event type"),
    }
}
```

### Advanced Pattern Matching

```rust
enum DetectionResult {
    NoAlert,
    Alert { severity: u8, message: String },
    CorrelatedAlert { 
        severity: u8, 
        message: String, 
        related_events: Vec<String> 
    },
}

fn process_detection(result: DetectionResult) {
    match result {
        DetectionResult::NoAlert => {
            // Do nothing
        }
        DetectionResult::Alert { severity, message } => {
            println!("ALERT [{}]: {}", severity, message);
        }
        DetectionResult::CorrelatedAlert { 
            severity, 
            message, 
            related_events 
        } => {
            println!("CORRELATED ALERT [{}]: {}", severity, message);
            println!("Related: {:?}", related_events);
        }
    }
}
```

### Pattern Matching with Guards

```rust
fn evaluate_brute_force(ip: &str, failed_count: u32, time_window_mins: u32) -> bool {
    match (failed_count, time_window_mins) {
        (count, window) if count > 10 && window < 5 => {
            println!("Brute force from {}: {} attempts in {} mins", 
                     ip, count, window);
            true
        }
        (count, window) if count > 20 && window < 15 => {
            println!("Potential brute force from {}", ip);
            true
        }
        _ => false,
    }
}
```

### Destructuring Structs

```rust
fn analyze_event_pattern(event: &SecurityEvent) {
    match event {
        SecurityEvent { 
            event_type, 
            severity: 9..=10, 
            source_ip,
            .. 
        } => {
            println!("CRITICAL: {} from {}", event_type, source_ip);
        }
        SecurityEvent { 
            event_type: login_type, 
            severity: 5..=8, 
            .. 
        } if login_type.contains("login") => {
            println!("Suspicious login activity");
        }
        _ => {
            // Other events
        }
    }
}
```

---

## 6. Concurrency Basics

Rust's ownership system prevents data races at compile time!

### Threading for Parallel Processing

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn process_logs_concurrently(log_files: Vec<String>) {
    let mut handles = vec![];
    
    for log_file in log_files {
        let handle = thread::spawn(move || {
            println!("Processing: {}", log_file);
            // Process log file
            let events = parse_log_file(&log_file);
            events.len()
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        match handle.join() {
            Ok(count) => println!("Processed {} events", count),
            Err(e) => eprintln!("Thread panicked: {:?}", e),
        }
    }
}

fn parse_log_file(path: &str) -> Vec<SecurityEvent> {
    // Parsing logic
    vec![]
}
```

### Shared State with Arc and Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn concurrent_event_collection() {
    // Arc = Atomic Reference Counting (thread-safe)
    // Mutex = Mutual exclusion for safe shared access
    let events = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for i in 0..4 {
        let events_clone = Arc::clone(&events);
        
        let handle = thread::spawn(move || {
            let event = SecurityEvent {
                timestamp: 1704067200 + i,
                source_ip: format!("192.168.1.{}", i),
                event_type: "test".to_string(),
                severity: i as u8,
            };
            
            // Lock mutex to modify shared data
            let mut events_vec = events_clone.lock().unwrap();
            events_vec.push(event);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_events = events.lock().unwrap();
    println!("Collected {} events", final_events.len());
}
```

### Channels for Message Passing

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn event_processing_pipeline() {
    let (tx, rx) = mpsc::channel();
    
    // Producer thread
    thread::spawn(move || {
        for i in 0..10 {
            let event = SecurityEvent {
                timestamp: 1704067200 + i,
                source_ip: format!("10.0.0.{}", i),
                event_type: "network_connection".to_string(),
                severity: (i % 5) as u8,
            };
            
            tx.send(event).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Consumer thread (main thread)
    for received in rx {
        println!("Processing event from {}", received.source_ip);
        // Detection logic here
    }
}
```

---

## 7. Security-Focused Rust

### Avoiding Common Vulnerabilities

#### 1. No Buffer Overflows

```rust
fn safe_string_handling() {
    let mut buffer = String::new();
    
    // Safe: String grows automatically
    buffer.push_str("This is safe");
    buffer.push_str(" - no buffer overflow!");
    
    // Safe: bounds checked
    let logs = vec!["log1", "log2", "log3"];
    if let Some(first) = logs.get(0) {
        println!("First log: {}", first);
    }
    
    // This would panic (not compile-time error, but caught at runtime)
    // let invalid = logs[100]; // Index out of bounds
}
```

#### 2. Type Safety for Security

```rust
// NewType pattern for type safety
struct IpAddress(String);
struct Hostname(String);

impl IpAddress {
    fn new(ip: String) -> Result<Self, String> {
        // Validate IP format
        if ip.split('.').count() == 4 {
            Ok(IpAddress(ip))
        } else {
            Err("Invalid IP address".to_string())
        }
    }
}

fn connect_to_ip(ip: IpAddress) {
    println!("Connecting to {}", ip.0);
}

fn main() {
    let ip = IpAddress::new("192.168.1.1".to_string()).unwrap();
    connect_to_ip(ip); // Type-safe!
    
    // Won't compile - type mismatch!
    // let hostname = Hostname("example.com".to_string());
    // connect_to_ip(hostname);
}
```

#### 3. Sanitization and Validation

```rust
use regex::Regex;

fn sanitize_input(input: &str) -> String {
    // Remove potentially dangerous characters
    input.chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-')
        .collect()
}

fn validate_log_source(source: &str) -> bool {
    let pattern = Regex::new(r"^[a-zA-Z0-9\-\.]+$").unwrap();
    pattern.is_match(source)
}
```

#### 4. Secure Random Generation

```rust
use rand::Rng;

fn generate_alert_id() -> String {
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    format!("alert-{:016x}", id)
}
```

### Best Practices

1. **Use `#![forbid(unsafe_code)]` when possible**
```rust
#![forbid(unsafe_code)]

// This crate won't compile with any unsafe blocks
```

2. **Leverage the type system**
```rust
// Instead of stringly-typed code:
fn bad_example(event_type: &str) { }

// Use enums:
enum EventType {
    Login,
    Logout,
    FileAccess,
    NetworkConnection,
}

fn good_example(event_type: EventType) { }
```

3. **Use Clippy for security lints**
```bash
cargo clippy -- -W clippy::all
```

---

## 8. Lab Exercises

### Lab 1: Build a Log Event Parser

**Objective:** Create a safe, efficient log parser using Rust fundamentals.

**Requirements:**
1. Define a `LogEvent` struct
2. Parse JSON logs into `LogEvent` instances
3. Handle parsing errors gracefully
4. Filter events by severity
5. Count events by type

**Starter Code:**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct LogEvent {
    timestamp: u64,
    source: String,
    event_type: String,
    severity: u8,
}

fn parse_log_line(json: &str) -> Result<LogEvent, Box<dyn std::error::Error>> {
    // TODO: Implement JSON parsing
    todo!()
}

fn filter_by_severity(events: Vec<LogEvent>, min_severity: u8) -> Vec<LogEvent> {
    // TODO: Implement filtering
    todo!()
}

fn count_by_type(events: &[LogEvent]) -> HashMap<String, usize> {
    // TODO: Implement counting
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log() {
        let json = r#"{"timestamp": 1704067200, "source": "web-server", "event_type": "http_request", "severity": 1}"#;
        let event = parse_log_line(json).unwrap();
        assert_eq!(event.event_type, "http_request");
    }

    #[test]
    fn test_filter_severity() {
        // TODO: Add test
    }
}
```

### Lab 2: Concurrent Log Processing

**Objective:** Process multiple log files concurrently.

**Tasks:**
1. Read 4 different log files
2. Parse each file in a separate thread
3. Collect all events into a shared vector
4. Print summary statistics

**Hints:**
- Use `Arc<Mutex<Vec<LogEvent>>>`
- Spawn threads with `thread::spawn`
- Use `.join()` to wait for completion

### Lab 3: Detection Rule Engine

**Objective:** Build a simple rule engine with pattern matching.

**Requirements:**
1. Define detection rules as enums
2. Match events against rules
3. Generate alerts for matches
4. Support multiple rule types:
   - Threshold-based (e.g., >5 failed logins)
   - Pattern-based (e.g., specific event type)
   - Correlation-based (e.g., two events within time window)

**Starter:**
```rust
enum DetectionRule {
    Threshold { event_type: String, count: u32, window_secs: u64 },
    Pattern { event_type: String, severity: u8 },
}

struct Alert {
    rule_name: String,
    triggered_at: u64,
    severity: u8,
    message: String,
}

fn evaluate_rule(rule: &DetectionRule, events: &[LogEvent]) -> Option<Alert> {
    // TODO: Implement rule evaluation
    todo!()
}
```

---

## Knowledge Check

### Questions

1. **Explain Rust's ownership rules and why they prevent memory safety issues.**

2. **What's the difference between `Result` and `Option`? When would you use each?**

3. **How does the borrow checker prevent data races?**

4. **Why are iterators in Rust considered "zero-cost abstractions"?**

5. **What are the advantages of pattern matching over if/else chains?**

### Coding Challenges

1. **Challenge 1:** Write a function that takes a vector of events and returns only those from the last 5 minutes.

2. **Challenge 2:** Implement a function that detects brute force attacks (>10 failed logins from same IP in 5 minutes).

3. **Challenge 3:** Create a concurrent pipeline that reads logs, parses them, filters by severity, and writes alerts to a file.

---

## Resources

### Essential Reading
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

### Security-Focused Resources
- [Secure Rust Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Rust Security Database](https://rustsec.org/)

### Practice Projects
- [Advent of Code in Rust](https://adventofcode.com/)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

---

## Next Steps

After mastering Rust fundamentals:

1. ✅ Understand ownership and borrowing
2. ✅ Handle errors properly
3. ✅ Use iterators and collections efficiently
4. ✅ Write concurrent code safely

**Continue to:** [Module 3: Detection Engineering Fundamentals](../module-03-detection-fundamentals/README.md)

---

## Feedback

Questions or suggestions? Open an issue on GitHub!
