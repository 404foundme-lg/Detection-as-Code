# Module 5: Pattern Matching and Detection Logic

## Learning Objectives

- Master regex for pattern matching
- Build state machines for complex detections
- Implement correlation engines
- Handle time-based detection windows
- Apply anomaly detection basics

## Key Topics

### 1. Regular Expressions with regex Crate

```rust
use regex::Regex;

struct PatternDetector {
    suspicious_patterns: Vec<Regex>,
}

impl PatternDetector {
    fn new() -> Self {
        let patterns = vec![
            Regex::new(r"Invoke-Mimikatz").unwrap(),
            Regex::new(r"DownloadString\(.*http").unwrap(),
            Regex::new(r"IEX\s*\(").unwrap(),
            Regex::new(r"powershell.*-enc\s+[A-Za-z0-9+/=]+").unwrap(),
        ];
        
        PatternDetector {
            suspicious_patterns: patterns,
        }
    }
    
    fn detect(&self, command: &str) -> Vec<String> {
        let mut matches = Vec::new();
        
        for pattern in &self.suspicious_patterns {
            if pattern.is_match(command) {
                matches.push(pattern.as_str().to_string());
            }
        }
        
        matches
    }
}
```

### 2. State Machines for Complex Patterns

```rust
#[derive(Debug, PartialEq)]
enum AttackState {
    Initial,
    Reconnaissance,
    InitialAccess,
    Persistence,
    Detected,
}

struct AttackStateMachine {
    state: AttackState,
    events: Vec<String>,
}

impl AttackStateMachine {
    fn new() -> Self {
        AttackStateMachine {
            state: AttackState::Initial,
            events: Vec::new(),
        }
    }
    
    fn process_event(&mut self, event_type: &str) {
        self.events.push(event_type.to_string());
        
        match (&self.state, event_type) {
            (AttackState::Initial, "port_scan") => {
                self.state = AttackState::Reconnaissance;
            }
            (AttackState::Reconnaissance, "failed_login") => {
                // Still in recon
            }
            (AttackState::Reconnaissance, "successful_login") => {
                self.state = AttackState::InitialAccess;
            }
            (AttackState::InitialAccess, "scheduled_task") => {
                self.state = AttackState::Persistence;
            }
            (AttackState::Persistence, _) => {
                self.state = AttackState::Detected;
            }
            _ => {}
        }
    }
    
    fn is_attack_detected(&self) -> bool {
        self.state == AttackState::Detected
    }
}
```

### 3. Correlation Engine

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CorrelationRule {
    name: String,
    events: Vec<String>,
    time_window: Duration,
}

struct Correlator {
    rules: Vec<CorrelationRule>,
    event_buffer: Vec<(String, Instant)>,
}

impl Correlator {
    fn add_event(&mut self, event_type: String) {
        let now = Instant::now();
        self.event_buffer.push((event_type, now));
        
        // Clean old events
        self.event_buffer.retain(|(_, time)| {
            now.duration_since(*time) < Duration::from_secs(3600)
        });
    }
    
    fn check_correlations(&self) -> Vec<String> {
        let mut alerts = Vec::new();
        
        for rule in &self.rules {
            if self.matches_rule(rule) {
                alerts.push(rule.name.clone());
            }
        }
        
        alerts
    }
    
    fn matches_rule(&self, rule: &CorrelationRule) -> bool {
        let now = Instant::now();
        let cutoff = now - rule.time_window;
        
        let recent_events: Vec<&String> = self.event_buffer
            .iter()
            .filter(|(_, time)| *time > cutoff)
            .map(|(event, _)| event)
            .collect();
        
        // Check if all required events are present
        rule.events.iter().all(|required| {
            recent_events.contains(&required)
        })
    }
}
```

### 4. Time Window Detection

```rust
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

struct TimeWindowDetector {
    events: VecDeque<SystemTime>,
    threshold: usize,
    window: Duration,
}

impl TimeWindowDetector {
    fn new(threshold: usize, window_secs: u64) -> Self {
        TimeWindowDetector {
            events: VecDeque::new(),
            threshold,
            window: Duration::from_secs(window_secs),
        }
    }
    
    fn add_event(&mut self) -> bool {
        let now = SystemTime::now();
        let cutoff = now - self.window;
        
        // Remove old events
        while let Some(&front) = self.events.front() {
            if front < cutoff {
                self.events.pop_front();
            } else {
                break;
            }
        }
        
        // Add new event
        self.events.push_back(now);
        
        // Check threshold
        self.events.len() >= self.threshold
    }
}
```

### 5. Anomaly Detection Basics

```rust
struct AnomalyDetector {
    baseline_mean: f64,
    baseline_stddev: f64,
    threshold_sigmas: f64,
}

impl AnomalyDetector {
    fn is_anomalous(&self, value: f64) -> bool {
        let z_score = (value - self.baseline_mean).abs() / self.baseline_stddev;
        z_score > self.threshold_sigmas
    }
}

// Example: Detect unusual login counts
fn detect_unusual_login_activity(user: &str, login_count: u32, detector: &AnomalyDetector) -> bool {
    detector.is_anomalous(login_count as f64)
}
```

## Lab Exercises

1. **Regex Pattern Library:** Build a library of common malicious patterns
2. **Multi-Stage Attack Detector:** Implement state machine for kill chain
3. **Correlation Engine:** Detect lateral movement through correlated events
4. **Anomaly Detector:** Build baseline and detect deviations

## Resources

- [regex crate documentation](https://docs.rs/regex/)
- [State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)

**Next:** [Module 6: Sigma Rule Integration](../module-06-sigma-integration/README.md)
