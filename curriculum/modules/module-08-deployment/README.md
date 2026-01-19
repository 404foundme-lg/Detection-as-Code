# Module 8: Deployment and Operations

## Learning Objectives

- Containerize detection systems
- Build CI/CD pipelines
- Implement monitoring and observability
- Manage configurations
- Integrate with incident response

## Key Topics

### 1. Dockerfile for Detection Engine

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/detection-engine /usr/local/bin/

ENTRYPOINT ["detection-engine"]
CMD ["--config", "/etc/detection/config.yaml"]
```

### 2. CI/CD Pipeline (GitHub Actions)

```yaml
name: Detection CI/CD

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test
      
      - name: Validate rules
        run: cargo run --bin validate-rules
  
  deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Build and push Docker image
        run: |
          docker build -t detection-engine:latest .
          docker push detection-engine:latest
```

### 3. Observability with Tracing

```rust
use tracing::{info, warn, error, instrument};
use tracing_subscriber;

#[instrument]
fn process_event(event: &Event) -> Result<Alert, Error> {
    info!("Processing event: {:?}", event.event_type);
    
    let result = run_detection(event);
    
    match &result {
        Ok(alert) => info!("Alert generated: {}", alert.severity),
        Err(e) => error!("Detection failed: {}", e),
    }
    
    result
}

fn main() {
    tracing_subscriber::fmt::init();
    // Application code
}
```

### 4. Configuration Management

```rust
use serde::{Deserialize, Serialize};
use config::Config;

#[derive(Debug, Deserialize, Serialize)]
struct DetectionConfig {
    log_sources: Vec<LogSource>,
    rules_path: String,
    alert_outputs: Vec<AlertOutput>,
    performance: PerformanceConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct PerformanceConfig {
    max_concurrent: usize,
    buffer_size: usize,
    batch_size: usize,
}

fn load_config(path: &str) -> Result<DetectionConfig, config::ConfigError> {
    Config::builder()
        .add_source(config::File::with_name(path))
        .build()?
        .try_deserialize()
}
```

### 5. Health Checks

```rust
use axum::{Router, routing::get};

async fn health_check() -> &'static str {
    "OK"
}

async fn metrics() -> String {
    format!("events_processed: {}\nalerts_generated: {}", 1000, 42)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics));
    
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Lab Exercises

1. **Containerize:** Build Docker image for detection engine
2. **CI/CD:** Set up GitHub Actions pipeline
3. **Monitoring:** Add Prometheus metrics
4. **Deploy:** Deploy to Kubernetes

## Resources

- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Tracing Documentation](https://docs.rs/tracing/)

**Next:** [Module 9: Advanced Topics](../module-09-advanced-topics/README.md)
