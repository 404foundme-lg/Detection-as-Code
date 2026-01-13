# Detection-as-Code Curriculum

## Overview

This comprehensive curriculum provides a professional, modular learning path for implementing Detection-as-Code using Rust. Designed for engineers with prior security or software development experience, it combines theoretical foundations with hands-on practical applications.

## Target Audience

- Security engineers looking to automate detection engineering
- Software developers transitioning to security roles
- DevSecOps engineers building detection pipelines
- SOC engineers seeking to improve detection capabilities

## Prerequisites

- Basic programming experience (any language)
- Understanding of security concepts (logs, alerts, threats)
- Familiarity with command-line tools
- Git version control basics

## Learning Objectives

By completing this curriculum, you will:

1. Master Rust fundamentals for security tooling
2. Design and implement detection rules as code
3. Parse and analyze security logs efficiently
4. Build high-performance detection engines
5. Integrate with industry-standard formats (Sigma, YARA-L)
6. Deploy detection systems in production environments
7. Optimize detection performance and reduce false positives
8. Contribute to open-source detection engineering

## Curriculum Structure

### Module 1: Foundations of Detection-as-Code
**Duration:** 1 week  
**Focus:** Core concepts, philosophy, and ecosystem

- What is Detection-as-Code?
- Benefits over traditional SIEM rules
- Industry landscape and tools
- Version control for detections
- Testing and validation strategies

### Module 2: Rust Fundamentals for Security
**Duration:** 2 weeks  
**Focus:** Essential Rust concepts for detection engineering

- Ownership, borrowing, and lifetimes
- Error handling patterns
- Collections and iterators
- Pattern matching
- Concurrency basics
- Security-focused Rust idioms

### Module 3: Detection Engineering Fundamentals
**Duration:** 1 week  
**Focus:** Security detection principles

- Log formats and schemas
- Attack frameworks (MITRE ATT&CK)
- Detection logic patterns
- False positive management
- Alert enrichment strategies

### Module 4: Data Parsing and Processing
**Duration:** 2 weeks  
**Focus:** Building robust log parsers

- Serde for JSON/YAML parsing
- Custom deserializers
- Schema validation
- Performance optimization
- Error recovery patterns

### Module 5: Pattern Matching and Detection Logic
**Duration:** 2 weeks  
**Focus:** Implementing detection algorithms

- Regular expressions with regex crate
- State machines for complex patterns
- Correlation engines
- Time-based detection windows
- Anomaly detection basics

### Module 6: Sigma Rule Integration
**Duration:** 1 week  
**Focus:** Industry-standard detection formats

- Sigma rule specification
- Parsing Sigma YAML
- Rule translation engines
- Backend compilation
- Custom modifiers

### Module 7: Performance and Scalability
**Duration:** 2 weeks  
**Focus:** Production-grade optimization

- Profiling detection code
- Memory management
- Concurrent processing
- Stream processing patterns
- Benchmarking strategies

### Module 8: Deployment and Operations
**Duration:** 1 week  
**Focus:** Production deployment

- Containerization (Docker)
- CI/CD pipelines
- Monitoring and observability
- Configuration management
- Incident response integration

### Module 9: Advanced Topics
**Duration:** 2 weeks  
**Focus:** Cutting-edge techniques

- Machine learning integration
- Graph-based detection
- Threat intelligence feeds
- Custom DSLs for detections
- Contributing to open source

## Hands-On Projects

### Project 1: Simple Log Parser
Build a parser for common log formats (JSON, CEF, Syslog)

### Project 2: Detection Rule Engine
Create a basic detection engine supporting AND/OR/NOT logic

### Project 3: Sigma Rule Compiler
Implement a Sigma-to-native rule translator

### Project 4: High-Performance Detection Pipeline
Build a concurrent detection pipeline processing 100K+ events/sec

### Project 5: Production Detection System
Deploy a complete detection system with monitoring

## Assessment Strategy

- **Module Quizzes:** Knowledge checks after each module
- **Coding Exercises:** Hands-on Rust implementations
- **Lab Assignments:** Practical detection scenarios
- **Capstone Project:** Complete detection system implementation
- **Code Reviews:** Peer and mentor feedback

## Recommended Study Path

### Fast Track (6-8 weeks)
For experienced Rust developers:
- Modules 1, 3, 4, 5, 6, 8
- Projects 2, 3, 5

### Standard Track (12-14 weeks)
Recommended for most students:
- All modules in sequence
- All projects
- All labs

### Extended Track (16-20 weeks)
For those new to Rust or security:
- All modules with additional practice
- Supplementary exercises
- Multiple iterations on projects

## Resources

### Official Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Sigma Specification](https://github.com/SigmaHQ/sigma-specification)
- [MITRE ATT&CK](https://attack.mitre.org/)

### Recommended Crates
- `serde`, `serde_json`, `serde_yaml` - Serialization
- `regex` - Pattern matching
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `tracing` - Logging and instrumentation
- `criterion` - Benchmarking

### Community
- Rust Security Discord
- Detection-as-Code GitHub Discussions
- InfoSec Rust Community

## Getting Started

1. **Setup Development Environment**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   rustup component add clippy rustfmt
   ```

2. **Clone Curriculum Repository**
   ```bash
   git clone https://github.com/404foundme-lg/Detection-as-Code
   cd Detection-as-Code/curriculum
   ```

3. **Start with Module 1**
   ```bash
   cd modules/module-01-foundations
   cat README.md
   ```

## Support and Contribution

- **Issues:** Report bugs or suggest improvements via GitHub Issues
- **Discussions:** Ask questions in GitHub Discussions
- **Contributing:** See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines

## License

This curriculum is released under the MIT License. See [LICENSE](../LICENSE) for details.

## Acknowledgments

Built with inspiration from the detection engineering and Rust communities. Special thanks to:
- Sigma Rule Project
- Detection Engineering Weekly
- Rust Security Working Group
- MITRE ATT&CK Framework team

---

**Ready to begin?** Start with [Module 1: Foundations of Detection-as-Code](modules/module-01-foundations/README.md)
