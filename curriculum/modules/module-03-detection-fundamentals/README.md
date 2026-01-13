# Module 3: Detection Engineering Fundamentals

## Learning Objectives

- Understand core detection engineering concepts
- Map detections to MITRE ATT&CK framework
- Work with common log formats and schemas
- Design effective detection logic
- Manage false positives and alert fatigue
- Implement alert enrichment strategies

## Table of Contents

1. [Detection Engineering Overview](#1-detection-engineering-overview)
2. [MITRE ATT&CK Framework](#2-mitre-attck-framework)
3. [Log Formats and Schemas](#3-log-formats-and-schemas)
4. [Detection Logic Patterns](#4-detection-logic-patterns)
5. [False Positive Management](#5-false-positive-management)
6. [Alert Enrichment](#6-alert-enrichment)
7. [Lab Exercises](#7-lab-exercises)

---

## 1. Detection Engineering Overview

### The Detection Pyramid

```
         /\
        /  \     Advanced Persistent Threats
       /    \    (Behavioral Analytics)
      /------\
     /  TTPs  \  Tactics, Techniques, Procedures
    /----------\ (Detection Rules)
   /  Atomic    \
  / Indicators   \ Indicators of Compromise
 /----------------\ (Hashes, IPs, Domains)
```

**Detection Layers:**

1. **Atomic Indicators:** Fast, specific, high false negative rate
2. **Detection Rules:** Behavioral patterns, moderate speed
3. **Analytics:** Statistical/ML models, slower but higher fidelity

### Detection Development Workflow

```rust
// Example: Detection development in Rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Detection {
    id: String,
    name: String,
    description: String,
    severity: Severity,
    mitre_techniques: Vec<String>,
    data_sources: Vec<String>,
    logic: DetectionLogic,
}

#[derive(Debug, Deserialize, Serialize)]
enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Deserialize, Serialize)]
enum DetectionLogic {
    SimpleMatch { field: String, value: String },
    Threshold { field: String, count: u32, window_secs: u64 },
    Correlation { rules: Vec<String>, within_secs: u64 },
}
```

---

## 2. MITRE ATT&CK Framework

### Understanding ATT&CK

The MITRE ATT&CK framework is a knowledge base of adversary tactics and techniques.

**14 Tactics (Enterprise):**
1. Reconnaissance
2. Resource Development
3. Initial Access
4. Execution
5. Persistence
6. Privilege Escalation
7. Defense Evasion
8. Credential Access
9. Discovery
10. Lateral Movement
11. Collection
12. Command and Control
13. Exfiltration
14. Impact

### Mapping Detections to ATT&CK

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct AttackTechnique {
    id: String,
    name: String,
    tactic: String,
    description: String,
}

struct AttackMapping {
    techniques: HashMap<String, AttackTechnique>,
}

impl AttackMapping {
    fn new() -> Self {
        let mut techniques = HashMap::new();
        
        techniques.insert(
            "T1110.001".to_string(),
            AttackTechnique {
                id: "T1110.001".to_string(),
                name: "Brute Force: Password Guessing".to_string(),
                tactic: "Credential Access".to_string(),
                description: "Adversaries may use brute force techniques...".to_string(),
            },
        );
        
        techniques.insert(
            "T1078".to_string(),
            AttackTechnique {
                id: "T1078".to_string(),
                name: "Valid Accounts".to_string(),
                tactic: "Initial Access".to_string(),
                description: "Adversaries may obtain credentials...".to_string(),
            },
        );
        
        AttackMapping { techniques }
    }
    
    fn get_technique(&self, id: &str) -> Option<&AttackTechnique> {
        self.techniques.get(id)
    }
}

// Example usage in detection
fn create_brute_force_detection() -> Detection {
    Detection {
        id: "DET-001".to_string(),
        name: "SSH Brute Force Detection".to_string(),
        description: "Detects multiple failed SSH authentication attempts".to_string(),
        severity: Severity::High,
        mitre_techniques: vec!["T1110.001".to_string()],
        data_sources: vec!["Linux Auth Logs".to_string()],
        logic: DetectionLogic::Threshold {
            field: "failed_ssh_login".to_string(),
            count: 5,
            window_secs: 300,
        },
    }
}
```

### Coverage Analysis

Track which ATT&CK techniques you can detect:

```rust
use std::collections::HashSet;

struct CoverageAnalyzer {
    all_techniques: HashSet<String>,
    covered_techniques: HashSet<String>,
}

impl CoverageAnalyzer {
    fn calculate_coverage(&self) -> f64 {
        if self.all_techniques.is_empty() {
            return 0.0;
        }
        
        (self.covered_techniques.len() as f64 / self.all_techniques.len() as f64) * 100.0
    }
    
    fn get_gaps(&self) -> Vec<String> {
        self.all_techniques
            .difference(&self.covered_techniques)
            .cloned()
            .collect()
    }
}
```

---

## 3. Log Formats and Schemas

### Common Log Formats

#### 1. JSON Logs

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct JsonLog {
    timestamp: String,
    level: String,
    message: String,
    #[serde(flatten)]
    extra: serde_json::Value,
}

fn parse_json_log(log_line: &str) -> Result<JsonLog, serde_json::Error> {
    serde_json::from_str(log_line)
}

// Example JSON log:
// {"timestamp":"2024-01-01T12:00:00Z","level":"ERROR","message":"Login failed","user":"admin","ip":"192.168.1.100"}
```

#### 2. CEF (Common Event Format)

```rust
#[derive(Debug)]
struct CefLog {
    version: u8,
    device_vendor: String,
    device_product: String,
    device_version: String,
    signature_id: String,
    name: String,
    severity: u8,
    extensions: std::collections::HashMap<String, String>,
}

fn parse_cef(log: &str) -> Result<CefLog, String> {
    // CEF Format: CEF:Version|Device Vendor|Device Product|Device Version|Signature ID|Name|Severity|Extension
    let parts: Vec<&str> = log.split('|').collect();
    
    if parts.len() < 8 {
        return Err("Invalid CEF format".to_string());
    }
    
    Ok(CefLog {
        version: parts[0].trim_start_matches("CEF:").parse().unwrap_or(0),
        device_vendor: parts[1].to_string(),
        device_product: parts[2].to_string(),
        device_version: parts[3].to_string(),
        signature_id: parts[4].to_string(),
        name: parts[5].to_string(),
        severity: parts[6].parse().unwrap_or(0),
        extensions: parse_cef_extensions(parts[7]),
    })
}

fn parse_cef_extensions(ext: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    
    for pair in ext.split_whitespace() {
        if let Some((key, value)) = pair.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    
    map
}
```

#### 3. Syslog

```rust
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct SyslogMessage {
    priority: u8,
    timestamp: Option<DateTime<Utc>>,
    hostname: String,
    app_name: String,
    message: String,
}

fn parse_syslog(log: &str) -> Result<SyslogMessage, String> {
    // Basic syslog parsing (RFC 3164)
    // Format: <priority>timestamp hostname app_name: message
    
    // This is simplified - use a proper syslog parser in production
    Ok(SyslogMessage {
        priority: 13,
        timestamp: None,
        hostname: "host".to_string(),
        app_name: "app".to_string(),
        message: log.to_string(),
    })
}
```

#### 4. Windows Event Logs (EVTX)

```rust
#[derive(Debug, Deserialize)]
struct WindowsEvent {
    #[serde(rename = "EventID")]
    event_id: u32,
    
    #[serde(rename = "Computer")]
    computer: String,
    
    #[serde(rename = "TimeCreated")]
    time_created: String,
    
    #[serde(rename = "EventData")]
    event_data: std::collections::HashMap<String, String>,
}

// Key Windows Event IDs for Security
const EVENT_SUCCESSFUL_LOGON: u32 = 4624;
const EVENT_FAILED_LOGON: u32 = 4625;
const EVENT_ACCOUNT_LOCKOUT: u32 = 4740;
const EVENT_PROCESS_CREATION: u32 = 4688;
const EVENT_SERVICE_INSTALL: u32 = 7045;
```

### Schema Validation

```rust
use serde_json::Value;

fn validate_log_schema(log: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    // Check required fields
    if !log.get("timestamp").is_some() {
        errors.push("Missing 'timestamp' field".to_string());
    }
    
    if !log.get("event_type").is_some() {
        errors.push("Missing 'event_type' field".to_string());
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## 4. Detection Logic Patterns

### Pattern 1: Simple Matching

```rust
fn simple_match_detection(event: &JsonLog) -> bool {
    // Detect specific PowerShell commands
    if let Some(command) = event.extra.get("CommandLine") {
        if let Some(cmd_str) = command.as_str() {
            return cmd_str.contains("Invoke-Mimikatz") 
                || cmd_str.contains("DownloadString");
        }
    }
    false
}
```

### Pattern 2: Threshold Detection

```rust
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

struct ThresholdDetector {
    event_counts: HashMap<String, Vec<SystemTime>>,
    threshold: usize,
    window: Duration,
}

impl ThresholdDetector {
    fn new(threshold: usize, window_secs: u64) -> Self {
        ThresholdDetector {
            event_counts: HashMap::new(),
            threshold,
            window: Duration::from_secs(window_secs),
        }
    }
    
    fn add_event(&mut self, key: String) -> bool {
        let now = SystemTime::now();
        let cutoff = now - self.window;
        
        // Get or create entry for this key
        let timestamps = self.event_counts.entry(key).or_insert_with(Vec::new);
        
        // Remove old events outside window
        timestamps.retain(|&t| t > cutoff);
        
        // Add new event
        timestamps.push(now);
        
        // Check if threshold exceeded
        timestamps.len() >= self.threshold
    }
}

// Usage
fn detect_brute_force(ip: String, detector: &mut ThresholdDetector) -> bool {
    // Returns true if IP has >5 failed logins in 5 minutes
    detector.add_event(ip)
}
```

### Pattern 3: Correlation Detection

```rust
use std::collections::HashMap;

struct CorrelationEngine {
    event_buffer: Vec<JsonLog>,
    max_buffer_size: usize,
}

impl CorrelationEngine {
    fn new(max_buffer_size: usize) -> Self {
        CorrelationEngine {
            event_buffer: Vec::new(),
            max_buffer_size,
        }
    }
    
    fn add_event(&mut self, event: JsonLog) {
        self.event_buffer.push(event);
        
        // Maintain buffer size
        if self.event_buffer.len() > self.max_buffer_size {
            self.event_buffer.remove(0);
        }
    }
    
    fn detect_lateral_movement(&self) -> Vec<String> {
        let mut alerts = Vec::new();
        
        // Look for: Failed login -> Successful login -> Process creation
        // This is a simplified example
        
        for window in self.event_buffer.windows(3) {
            if window.len() == 3 {
                let event1_type = window[0].extra.get("event_type").and_then(|v| v.as_str());
                let event2_type = window[1].extra.get("event_type").and_then(|v| v.as_str());
                let event3_type = window[2].extra.get("event_type").and_then(|v| v.as_str());
                
                if event1_type == Some("failed_login")
                    && event2_type == Some("successful_login")
                    && event3_type == Some("process_creation")
                {
                    alerts.push("Potential lateral movement detected".to_string());
                }
            }
        }
        
        alerts
    }
}
```

### Pattern 4: Anomaly Detection

```rust
struct BaselineProfile {
    normal_login_hours: Vec<u8>, // Hours 0-23
    normal_process_count: u32,
    known_user_agents: std::collections::HashSet<String>,
}

impl BaselineProfile {
    fn is_anomalous_time(&self, hour: u8) -> bool {
        !self.normal_login_hours.contains(&hour)
    }
    
    fn is_unusual_user_agent(&self, user_agent: &str) -> bool {
        !self.known_user_agents.contains(user_agent)
    }
}

fn detect_anomaly(event: &JsonLog, baseline: &BaselineProfile) -> bool {
    // Check for after-hours activity
    if let Some(hour_str) = event.extra.get("hour").and_then(|v| v.as_u64()) {
        let hour = hour_str as u8;
        if baseline.is_anomalous_time(hour) {
            return true;
        }
    }
    
    // Check for unusual user agent
    if let Some(ua) = event.extra.get("user_agent").and_then(|v| v.as_str()) {
        if baseline.is_unusual_user_agent(ua) {
            return true;
        }
    }
    
    false
}
```

---

## 5. False Positive Management

### Strategies for Reducing False Positives

#### 1. Whitelisting

```rust
use std::collections::HashSet;

struct Whitelist {
    trusted_ips: HashSet<String>,
    trusted_users: HashSet<String>,
    trusted_processes: HashSet<String>,
}

impl Whitelist {
    fn is_whitelisted(&self, event: &JsonLog) -> bool {
        // Check IP whitelist
        if let Some(ip) = event.extra.get("source_ip").and_then(|v| v.as_str()) {
            if self.trusted_ips.contains(ip) {
                return true;
            }
        }
        
        // Check user whitelist
        if let Some(user) = event.extra.get("user").and_then(|v| v.as_str()) {
            if self.trusted_users.contains(user) {
                return true;
            }
        }
        
        false
    }
}
```

#### 2. Confidence Scoring

```rust
struct ConfidenceScorer {
    weights: HashMap<String, f64>,
}

impl ConfidenceScorer {
    fn calculate_confidence(&self, indicators: &HashMap<String, bool>) -> f64 {
        let mut score = 0.0;
        let mut max_score = 0.0;
        
        for (indicator, &present) in indicators {
            if let Some(&weight) = self.weights.get(indicator) {
                max_score += weight;
                if present {
                    score += weight;
                }
            }
        }
        
        if max_score > 0.0 {
            (score / max_score) * 100.0
        } else {
            0.0
        }
    }
}

// Usage
fn score_brute_force_attempt() -> f64 {
    let mut scorer = ConfidenceScorer {
        weights: HashMap::new(),
    };
    
    scorer.weights.insert("multiple_failures".to_string(), 30.0);
    scorer.weights.insert("short_time_window".to_string(), 25.0);
    scorer.weights.insert("known_bad_ip".to_string(), 45.0);
    
    let mut indicators = HashMap::new();
    indicators.insert("multiple_failures".to_string(), true);
    indicators.insert("short_time_window".to_string(), true);
    indicators.insert("known_bad_ip".to_string(), false);
    
    scorer.calculate_confidence(&indicators)
}
```

#### 3. Context-Aware Detection

```rust
struct DetectionContext {
    business_hours: (u8, u8), // (start_hour, end_hour)
    maintenance_windows: Vec<(SystemTime, SystemTime)>,
}

impl DetectionContext {
    fn should_suppress(&self, event_time: SystemTime) -> bool {
        // Suppress during maintenance
        for (start, end) in &self.maintenance_windows {
            if event_time >= *start && event_time <= *end {
                return true;
            }
        }
        false
    }
}
```

---

## 6. Alert Enrichment

### Enrichment Sources

```rust
use std::collections::HashMap;

struct EnrichmentEngine {
    threat_intel: ThreatIntelligence,
    asset_inventory: AssetInventory,
    user_directory: UserDirectory,
}

struct ThreatIntelligence {
    known_bad_ips: HashMap<String, ThreatInfo>,
}

struct ThreatInfo {
    reputation_score: u8,
    categories: Vec<String>,
    last_seen: String,
}

struct AssetInventory {
    assets: HashMap<String, AssetInfo>,
}

struct AssetInfo {
    hostname: String,
    owner: String,
    criticality: String,
}

struct UserDirectory {
    users: HashMap<String, UserInfo>,
}

struct UserInfo {
    name: String,
    department: String,
    manager: String,
}

impl EnrichmentEngine {
    fn enrich_alert(&self, alert: &mut Alert) {
        // Add threat intelligence
        if let Some(ip) = &alert.source_ip {
            if let Some(threat_info) = self.threat_intel.known_bad_ips.get(ip) {
                alert.enrichment.insert(
                    "threat_reputation".to_string(),
                    threat_info.reputation_score.to_string(),
                );
            }
        }
        
        // Add asset context
        if let Some(hostname) = &alert.hostname {
            if let Some(asset) = self.asset_inventory.assets.get(hostname) {
                alert.enrichment.insert(
                    "asset_owner".to_string(),
                    asset.owner.clone(),
                );
                alert.enrichment.insert(
                    "asset_criticality".to_string(),
                    asset.criticality.clone(),
                );
            }
        }
        
        // Add user context
        if let Some(user) = &alert.user {
            if let Some(user_info) = self.user_directory.users.get(user) {
                alert.enrichment.insert(
                    "user_department".to_string(),
                    user_info.department.clone(),
                );
            }
        }
    }
}

#[derive(Debug)]
struct Alert {
    id: String,
    severity: Severity,
    source_ip: Option<String>,
    hostname: Option<String>,
    user: Option<String>,
    enrichment: HashMap<String, String>,
}
```

---

## 7. Lab Exercises

### Lab 1: Build a Multi-Format Log Parser

**Objective:** Parse JSON, CEF, and Syslog formats into a unified structure.

**Tasks:**
1. Define a unified `LogEvent` struct
2. Implement parsers for each format
3. Handle parsing errors
4. Test with sample logs

### Lab 2: Implement Threshold Detection

**Objective:** Detect brute force attacks using threshold logic.

**Requirements:**
- Track failed login attempts per IP
- Alert when >10 attempts in 5 minutes
- Clean up old events from memory
- Test with simulated event stream

### Lab 3: Build a Correlation Engine

**Objective:** Detect lateral movement through event correlation.

**Pattern to Detect:**
1. Failed RDP login
2. Successful RDP login (within 5 minutes)
3. Process creation (within 1 minute)

**Deliverables:**
- Correlation engine implementation
- Test cases with positive/negative examples
- Performance metrics

---

## Knowledge Check

1. **What are the three layers of the detection pyramid?**
2. **How does MITRE ATT&CK help in detection engineering?**
3. **Name three common log formats and their use cases.**
4. **What's the difference between threshold and correlation detection?**
5. **List three strategies for reducing false positives.**

---

## Resources

- [MITRE ATT&CK](https://attack.mitre.org/)
- [Sigma Rule Format](https://github.com/SigmaHQ/sigma-specification)
- [CEF Format Guide](https://www.microfocus.com/documentation/arcsight/arcsight-smartconnectors/)
- [Detection Engineering Guide](https://www.splunk.com/en_us/blog/security/detection-engineering-maturity-matrix.html)

---

## Next Steps

**Continue to:** [Module 4: Data Parsing and Processing](../module-04-data-parsing/README.md)
