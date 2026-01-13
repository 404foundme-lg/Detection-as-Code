# Module 9: Advanced Topics

## Learning Objectives

- Integrate machine learning models
- Implement graph-based detection
- Consume threat intelligence feeds
- Build custom detection DSLs
- Contribute to open source projects

## Key Topics

### 1. ML Integration with tract

```rust
use tract_onnx::prelude::*;

struct MLDetector {
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl MLDetector {
    fn load(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;
        
        Ok(MLDetector { model })
    }
    
    fn predict(&self, features: Vec<f32>) -> Result<f32, Box<dyn std::error::Error>> {
        let input = tract_ndarray::Array::from_vec(features);
        let result = self.model.run(tvec!(input.into()))?;
        
        // Extract prediction score
        Ok(0.85)
    }
}
```

### 2. Graph-Based Detection

```rust
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;

struct ThreatGraph {
    graph: Graph<Entity, Relationship>,
}

#[derive(Debug)]
enum Entity {
    User(String),
    Host(String),
    Process(String),
}

#[derive(Debug)]
enum Relationship {
    LoggedInto,
    Executed,
    ConnectedTo,
}

impl ThreatGraph {
    fn add_event(&mut self, event: &Event) {
        // Build graph from events
    }
    
    fn detect_lateral_movement(&self) -> Vec<String> {
        // Analyze graph for suspicious patterns
        vec![]
    }
}
```

### 3. Threat Intelligence Integration

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ThreatFeed {
    indicators: Vec<Indicator>,
}

#[derive(Debug, Deserialize)]
struct Indicator {
    value: String,
    indicator_type: String,
    confidence: u8,
}

async fn fetch_threat_intel(api_url: &str) -> Result<ThreatFeed, Box<dyn std::error::Error>> {
    let response = reqwest::get(api_url).await?;
    let feed = response.json::<ThreatFeed>().await?;
    Ok(feed)
}

fn enrich_with_threat_intel(event: &Event, feed: &ThreatFeed) -> bool {
    for indicator in &feed.indicators {
        if event.contains_indicator(&indicator.value) {
            return true;
        }
    }
    false
}
```

### 4. Custom Detection DSL

```rust
// Define a simple DSL for detection rules
macro_rules! detection_rule {
    ($name:expr, $condition:expr) => {
        Rule {
            name: $name.to_string(),
            condition: Box::new($condition),
        }
    };
}

struct Rule {
    name: String,
    condition: Box<dyn Fn(&Event) -> bool>,
}

// Usage
fn create_rules() -> Vec<Rule> {
    vec![
        detection_rule!("High Severity Events", |e: &Event| e.severity > 7),
        detection_rule!("Failed Logins", |e: &Event| e.event_type == "failed_login"),
    ]
}
```

### 5. Contributing to Open Source

**Best Practices:**
- Fork detection rule repositories
- Add well-documented rules
- Include test cases
- Follow project guidelines
- Engage with community

**Example Contribution:**
```bash
# Fork sigma repository
git clone https://github.com/yourusername/sigma
cd sigma

# Create detection branch
git checkout -b detection/aws-credential-access

# Add rule
vim rules/cloud/aws/aws_iam_credential_access.yml

# Test
python tools/sigma/sigma.py validate rules/cloud/aws/

# Commit and push
git add rules/cloud/aws/aws_iam_credential_access.yml
git commit -m "Add AWS IAM credential access detection"
git push origin detection/aws-credential-access

# Create pull request on GitHub
```

## Capstone Project

Build a complete detection system with:
- Multi-format log ingestion
- Sigma rule support
- ML-based anomaly detection
- Threat intelligence integration
- Real-time alerting
- Performance monitoring
- Docker deployment

## Resources

- [tract ML Framework](https://github.com/sonos/tract)
- [petgraph for Graphs](https://docs.rs/petgraph/)
- [MISP Threat Intelligence](https://www.misp-project.org/)

## Course Completion

Congratulations on completing the Detection-as-Code curriculum!

**Next steps:**
- Build detection systems for your organization
- Contribute to open source projects
- Share knowledge with the community
- Continue learning and improving

---

**Have feedback?** Open an issue or contribute to the curriculum!
