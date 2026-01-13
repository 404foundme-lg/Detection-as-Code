# Module 6: Sigma Rule Integration

## Learning Objectives

- Understand Sigma rule specification
- Parse Sigma YAML rules in Rust
- Build rule translation engines
- Implement backend compilers
- Create custom field mappings

## Key Topics

### 1. Sigma Rule Structure

```yaml
title: Suspicious PowerShell Command
id: 1a2b3c4d-5e6f-7g8h-9i0j-k1l2m3n4o5p6
status: experimental
description: Detects suspicious PowerShell commands
references:
    - https://attack.mitre.org/techniques/T1059/001/
author: Detection Engineer
date: 2024/01/01
tags:
    - attack.execution
    - attack.t1059.001
logsource:
    product: windows
    service: powershell
detection:
    selection:
        EventID: 4104
        ScriptBlockText|contains:
            - 'DownloadString'
            - 'Invoke-Expression'
    condition: selection
falsepositives:
    - Legitimate admin scripts
level: high
```

### 2. Parsing Sigma Rules in Rust

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct SigmaRule {
    title: String,
    id: String,
    status: String,
    description: String,
    author: String,
    date: String,
    tags: Vec<String>,
    logsource: LogSource,
    detection: Detection,
    falsepositives: Vec<String>,
    level: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LogSource {
    product: String,
    service: Option<String>,
    category: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Detection {
    #[serde(flatten)]
    selections: HashMap<String, serde_yaml::Value>,
    condition: String,
}

fn load_sigma_rule(path: &str) -> Result<SigmaRule, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let rule: SigmaRule = serde_yaml::from_str(&contents)?;
    Ok(rule)
}
```

### 3. Rule Evaluation Engine

```rust
use serde_json::Value;

struct SigmaEngine {
    rules: Vec<SigmaRule>,
}

impl SigmaEngine {
    fn evaluate(&self, event: &Value) -> Vec<String> {
        let mut matches = Vec::new();
        
        for rule in &self.rules {
            if self.rule_matches(rule, event) {
                matches.push(rule.title.clone());
            }
        }
        
        matches
    }
    
    fn rule_matches(&self, rule: &SigmaRule, event: &Value) -> bool {
        // Simplified evaluation
        // Real implementation would parse condition syntax
        true
    }
}
```

### 4. Backend Translation

```rust
trait SigmaBackend {
    fn translate(&self, rule: &SigmaRule) -> String;
}

struct ElasticsearchBackend;

impl SigmaBackend for ElasticsearchBackend {
    fn translate(&self, rule: &SigmaRule) -> String {
        // Convert Sigma to Elasticsearch Query DSL
        format!(r#"
        {{
            "query": {{
                "bool": {{
                    "must": [
                        {{"match": {{"EventID": 4104}}}},
                        {{"match": {{"ScriptBlockText": "DownloadString"}}}}
                    ]
                }}
            }}
        }}
        "#)
    }
}

struct SplunkBackend;

impl SigmaBackend for SplunkBackend {
    fn translate(&self, rule: &SigmaRule) -> String {
        // Convert Sigma to Splunk SPL
        format!("EventID=4104 ScriptBlockText=*DownloadString* OR ScriptBlockText=*Invoke-Expression*")
    }
}
```

## Lab Exercises

1. **Parse Sigma Rules:** Load and validate Sigma YAML files
2. **Build Evaluator:** Implement simple condition evaluator
3. **Backend Compiler:** Translate to Splunk SPL or Elastic DSL
4. **Rule Validator:** Check Sigma rules for correctness

## Resources

- [Sigma Specification](https://github.com/SigmaHQ/sigma-specification)
- [SigmaHQ Rules](https://github.com/SigmaHQ/sigma)

**Next:** [Module 7: Performance and Scalability](../module-07-performance/README.md)
