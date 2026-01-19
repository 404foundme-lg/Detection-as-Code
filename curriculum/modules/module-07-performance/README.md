# Module 7: Performance and Scalability

## Learning Objectives

- Profile Rust detection code
- Optimize memory usage
- Implement concurrent processing
- Build streaming pipelines
- Benchmark detection performance

## Key Topics

### 1. Profiling with flamegraph

```bash
cargo install flamegraph
cargo flamegraph --bin detection-engine
```

### 2. Memory Optimization

```rust
// Bad: Creates many temporary strings
fn inefficient_processing(logs: &[String]) -> Vec<String> {
    logs.iter()
        .map(|log| log.clone())
        .filter(|log| log.contains("error"))
        .collect()
}

// Good: Uses references
fn efficient_processing(logs: &[String]) -> Vec<&String> {
    logs.iter()
        .filter(|log| log.contains("error"))
        .collect()
}
```

### 3. Concurrent Processing with Rayon

```rust
use rayon::prelude::*;

fn process_logs_parallel(logs: Vec<String>) -> Vec<Alert> {
    logs.par_iter()
        .filter_map(|log| detect_threat(log))
        .collect()
}

fn detect_threat(log: &str) -> Option<Alert> {
    // Detection logic
    None
}

#[derive(Debug)]
struct Alert {
    severity: u8,
    message: String,
}
```

### 4. Streaming with Tokio

```rust
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;

async fn stream_log_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {
        process_log_line(&line).await;
    }
    
    Ok(())
}

async fn process_log_line(line: &str) {
    // Process each line asynchronously
}
```

### 5. Benchmarking with Criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse_benchmark(c: &mut Criterion) {
    let sample_log = r#"{"timestamp":1704067200,"event":"login"}"#;
    
    c.bench_function("parse_json_log", |b| {
        b.iter(|| parse_log(black_box(sample_log)))
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
```

### 6. Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Throughput | >100K events/sec | Single core |
| Latency | <10ms | P99 processing time |
| Memory | <500MB | Per 1M events buffered |
| CPU | <50% | Normal operation |

## Lab Exercises

1. **Profile Detection Engine:** Identify bottlenecks
2. **Optimize Parser:** Achieve 2x performance improvement
3. **Parallel Processing:** Scale to all CPU cores
4. **Stream Processing:** Handle GB-sized log files

## Resources

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rayon Documentation](https://docs.rs/rayon/)
- [Tokio Documentation](https://tokio.rs/)

**Next:** [Module 8: Deployment and Operations](../module-08-deployment/README.md)
