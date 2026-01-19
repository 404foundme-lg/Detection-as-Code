# Detection-as-Code Glossary

## A

**Alert**
A notification generated when a detection rule matches observed activity.

**Anomaly Detection**
Detection method that identifies deviations from established baselines.

**ATT&CK (Adversarial Tactics, Techniques, and Common Knowledge)**
MITRE's knowledge base of adversary behaviors. See [attack.mitre.org](https://attack.mitre.org)

**Atomic Indicator**
Simple, specific indicators like file hashes, IP addresses, or domain names.

## B

**Baseline**
Normal behavior patterns established through observation over time.

**Borrowing**
Rust's mechanism for referencing data without taking ownership.

**Brute Force**
Attack method involving systematic trial of many passwords or credentials.

## C

**CEF (Common Event Format)**
Standard log format: `CEF:Version|Vendor|Product|Version|SignatureID|Name|Severity|Extension`

**Clippy**
Rust's linting tool that catches common mistakes and suggests improvements.

**Correlation**
Combining multiple events to detect complex attack patterns.

**Coverage**
Percentage of ATT&CK techniques your detections can identify.

## D

**Detection**
Logic that identifies potentially malicious activity from log data.

**Detection-as-Code**
Methodology of treating security detections as software: versioned, tested, and automated.

**DSL (Domain-Specific Language)**
Specialized language designed for a particular problem domain.

## E

**Enrichment**
Adding context to alerts (threat intel, asset info, user details).

**EVTX**
Windows Event Log file format.

## F

**False Positive (FP)**
Alert generated on benign activity, not a real threat.

**False Negative (FN)**
Failure to detect actual malicious activity.

## I

**Indicator of Compromise (IOC)**
Evidence that a system has been compromised (e.g., malware hash, C2 domain).

## K

**Kill Chain**
Sequence of stages in a cyber attack (Recon → Weaponization → Delivery → Exploitation → Installation → C2 → Actions).

**KQL (Kusto Query Language)**
Query language used by Azure Sentinel and Microsoft 365 Defender.

## L

**Lateral Movement**
Attacker movement from one compromised system to others in the network.

**Lifetime**
Rust concept defining how long references remain valid.

**Log Source**
System or application generating security-relevant logs.

## M

**MITRE ATT&CK**
See ATT&CK.

**Mutex**
Mutual exclusion lock for safe concurrent access to shared data.

## O

**Ownership**
Rust's system for managing memory through compile-time rules.

## P

**Pattern Matching**
Detecting specific sequences or characteristics in data.

**Precision**
Ratio of true alerts to all alerts: TP / (TP + FP)

**Persistence**
Techniques attackers use to maintain access across reboots.

## R

**Recall**
Ratio of detected attacks to all attacks: TP / (TP + FN)

**Rust**
Systems programming language focused on safety, speed, and concurrency.

## S

**Sigma**
Generic signature format for SIEM systems. YAML-based, platform-agnostic.

**SIEM (Security Information and Event Management)**
Platform for collecting, analyzing, and alerting on security logs.

**Serde**
Rust's serialization/deserialization framework.

**State Machine**
Model tracking system states and transitions, useful for multi-stage detections.

**Syslog**
Standard logging protocol and format for Unix-like systems.

## T

**Tactic**
High-level adversary goal (e.g., Credential Access, Lateral Movement).

**Technique**
Specific method adversaries use to accomplish tactics.

**Threshold Detection**
Alerting when events exceed a count within a time window.

**Threat Intelligence**
Information about threats, TTPs, and indicators.

**TTP (Tactics, Techniques, and Procedures)**
Behavior patterns that characterize threat actors.

## W

**Whitelist**
List of known-good entities that should not generate alerts.

## Y

**YAML (YAML Ain't Markup Language)**
Human-readable data serialization format, used by Sigma.

**YARA**
Pattern matching tool for malware research (not covered extensively in this curriculum).

## Z

**Zero-Day**
Previously unknown vulnerability with no available patch.

---

## Rust-Specific Terms

**Arc (Atomic Reference Counting)**
Thread-safe reference-counted pointer.

**Cargo**
Rust's package manager and build system.

**Crate**
Rust package or library.

**Enum**
Rust type that can be one of several variants.

**impl**
Keyword for implementing methods or traits.

**match**
Rust's pattern matching construct.

**Option<T>**
Type representing an optional value: `Some(T)` or `None`.

**Result<T, E>**
Type for operations that can succeed (`Ok(T)`) or fail (`Err(E)`).

**Trait**
Rust's interface-like mechanism for shared behavior.

**Vec<T>**
Dynamically-sized array/vector.

---

## Detection Engineering Terms

**Blue Team**
Defensive security team focused on detection and response.

**EDR (Endpoint Detection and Response)**
Security tool monitoring endpoint activity.

**Hunt Hypothesis**
Testable assumption about adversary behavior to guide threat hunting.

**Indicator Pyramid**
Model showing relationship between atomic indicators, TTPs, and behaviors.

**MSSP (Managed Security Service Provider)**
Company providing security monitoring services.

**Playbook**
Documented procedure for investigating specific alert types.

**Red Team**
Offensive security team simulating attacks.

**Runbook**
Step-by-step guide for handling security events.

**SOC (Security Operations Center)**
Team and facility for security monitoring.

**TTP Matrix**
Grid mapping techniques to tactics (ATT&CK format).

**Tuning**
Adjusting detection rules to reduce false positives while maintaining coverage.

---

*Don't see a term? Open an issue to request it!*
