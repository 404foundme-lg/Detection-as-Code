# Hands-On Projects

Comprehensive projects to build during the curriculum.

## Project 1: Simple Log Parser
**Duration:** 1 week  
**Modules:** 2, 4

### Objective
Build a robust parser supporting JSON, CEF, and Syslog formats.

### Requirements
1. Parse all three log formats
2. Normalize to common schema
3. Handle malformed inputs gracefully
4. Achieve >50K events/sec throughput
5. Include comprehensive tests

### Deliverables
- [ ] Working parser binary
- [ ] Unit tests with >80% coverage
- [ ] Documentation
- [ ] Performance benchmarks

---

## Project 2: Detection Rule Engine
**Duration:** 2 weeks  
**Modules:** 3, 5

### Objective
Create a detection engine with AND/OR/NOT logic.

### Requirements
1. Support simple pattern matching
2. Implement threshold detection
3. Handle multiple rules simultaneously
4. Generate structured alerts
5. Track false positive rates

### Deliverables
- [ ] Rule engine library
- [ ] Sample detection rules
- [ ] Test suite
- [ ] Performance report

---

## Project 3: Sigma Rule Compiler
**Duration:** 2 weeks  
**Modules:** 6

### Objective
Build a Sigma-to-native rule translator.

### Requirements
1. Parse Sigma YAML format
2. Validate rule syntax
3. Translate to 2+ backends (Splunk, Elastic)
4. Support field mappings
5. CLI tool for conversion

### Deliverables
- [ ] Sigma parser
- [ ] Backend translators
- [ ] CLI tool
- [ ] Documentation

---

## Project 4: High-Performance Detection Pipeline
**Duration:** 2-3 weeks  
**Modules:** 7, 8

### Objective
Build a production-ready pipeline processing 100K+ events/sec.

### Requirements
1. Concurrent log processing
2. Real-time detection
3. Multiple output formats
4. Monitoring and metrics
5. Docker deployment

### Deliverables
- [ ] Detection pipeline
- [ ] Dockerfile and compose file
- [ ] Grafana dashboards
- [ ] Load test results

---

## Project 5: Production Detection System (Capstone)
**Duration:** 3-4 weeks  
**Modules:** All

### Objective
Deploy a complete detection system with all features.

### Requirements
1. Multi-source log ingestion
2. Sigma rule support
3. Threat intel integration
4. Real-time alerting
5. Web dashboard
6. CI/CD pipeline
7. Documentation

### Architecture
```
┌─────────────┐
│ Log Sources │
└──────┬──────┘
       │
┌──────▼──────┐
│  Ingestion  │
│   Layer     │
└──────┬──────┘
       │
┌──────▼──────┐
│  Detection  │
│   Engine    │
└──────┬──────┘
       │
┌──────▼──────┐
│   Alert     │
│  Manager    │
└──────┬──────┘
       │
┌──────▼──────┐
│   Outputs   │
│ (SIEM, etc) │
└─────────────┘
```

### Deliverables
- [ ] Complete system implementation
- [ ] Deployment scripts
- [ ] User documentation
- [ ] Operator guide
- [ ] Presentation/demo

## Evaluation Criteria

Projects are evaluated on:
- **Functionality** (40%): Does it work as specified?
- **Code Quality** (20%): Clean, maintainable, idiomatic Rust?
- **Testing** (15%): Comprehensive test coverage?
- **Performance** (10%): Meets performance targets?
- **Documentation** (15%): Clear and complete?

## Submission

For each project:
1. Create GitHub repository
2. Include README with setup instructions
3. Add CI/CD pipeline
4. Tag release version
5. Submit repository URL
