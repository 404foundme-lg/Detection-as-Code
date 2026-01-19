# Quick Start Guide

Get started with the Detection-as-Code curriculum in minutes!

## 5-Minute Setup

### 1. Install Rust

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then restart your terminal

# Verify installation
rustc --version
cargo --version
```

### 2. Install Additional Tools

```bash
# Essential development tools
rustup component add clippy rustfmt rust-analyzer

# Optional: Sigma CLI for rule validation
pip install sigma-cli

# Optional: YAML linter
pip install yamllint
```

### 3. Clone Repository

```bash
git clone https://github.com/404foundme-lg/Detection-as-Code
cd Detection-as-Code
```

### 4. Try the Example

```bash
cd curriculum/examples/simple-parser

# Build the example
cargo build

# Run tests
cargo test

# Try it out
echo '{"timestamp":1704067200,"level":"INFO","message":"Hello, Detection-as-Code!"}' | cargo run
```

## Choose Your Path

### For Security Engineers New to Rust

**Start here:** [Module 1: Foundations](curriculum/modules/module-01-foundations/README.md)

**Timeline:** 16-20 weeks (Extended Track)

**Focus areas:**
- Take time with Module 2 (Rust Fundamentals)
- Complete all exercises and labs
- Build all 5 projects
- Join Rust community for support

### For Rust Developers New to Security

**Start here:** [Module 1: Foundations](curriculum/modules/module-01-foundations/README.md)

**Timeline:** 6-8 weeks (Fast Track)

**You can skip:**
- Most of Module 2 (review as needed)
- Basic Rust exercises

**Focus on:**
- Security concepts in Module 3
- Detection patterns in Module 5
- MITRE ATT&CK framework
- Projects 3-5

### For Experienced Detection Engineers

**Start here:** [Module 3: Detection Engineering Fundamentals](curriculum/modules/module-03-detection-fundamentals/README.md)

**Timeline:** 12-14 weeks (Standard Track)

**You can skim:**
- Module 1 (review Detection-as-Code philosophy)
- Module 3 fundamentals (if already familiar)

**Deep dive:**
- Module 6: Sigma Integration
- Module 7: Performance
- Module 9: Advanced Topics

## First Steps

### Week 1: Foundations

1. **Read:** [Module 1 README](curriculum/modules/module-01-foundations/README.md)
2. **Complete:** [Lab 1 - Git Workflow](curriculum/modules/module-01-foundations/labs/lab1-git-workflow.md)
3. **Complete:** [Lab 2 - Sigma Rule](curriculum/modules/module-01-foundations/labs/lab2-sigma-rule.md)
4. **Setup:** Your development environment

### Week 2: Rust Basics

1. **Read:** [Module 2 README](curriculum/modules/module-02-rust-basics/README.md)
2. **Practice:** Ownership and borrowing exercises
3. **Build:** Simple log event structure
4. **Test:** Write your first Rust tests

### Week 3: First Project

1. **Start:** [Project 1 - Simple Log Parser](curriculum/projects/README.md)
2. **Implement:** JSON parsing
3. **Add:** Error handling
4. **Test:** Unit tests

## Development Environment

### Recommended Editor Setup

**VS Code:**
```bash
# Install VS Code extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension tamasfe.even-better-toml
code --install-extension serayuzgur.crates
```

**IntelliJ IDEA / CLion:**
- Install Rust plugin from JetBrains Marketplace

**Vim/Neovim:**
- Install rust-analyzer LSP
- Configure with coc.nvim or native LSP

### Useful Cargo Commands

```bash
# Create new project
cargo new my-detector

# Build project
cargo build

# Run tests
cargo test

# Run with release optimizations
cargo run --release

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open
```

## Essential Reading

Before diving deep, read:

1. **[The Rust Book](https://doc.rust-lang.org/book/)** - Chapters 1-10
2. **[Sigma Specification](https://github.com/SigmaHQ/sigma-specification)** - Overview
3. **[MITRE ATT&CK](https://attack.mitre.org/)** - Framework basics

## Learning Tips

### 1. Code Every Day
- Even 30 minutes makes a difference
- Practice Rust syntax with small examples
- Build muscle memory

### 2. Join the Community
- **Rust Discord:** Get help with Rust questions
- **Detection Engineering Slack:** Discuss security topics
- **GitHub Discussions:** Course-specific questions

### 3. Build Real Projects
- Apply concepts to your work
- Detect threats in your environment
- Share what you build

### 4. Learn in Public
- Blog about your progress
- Share code on GitHub
- Help others learning

### 5. Iterate and Improve
- Start simple, then optimize
- Test continuously
- Refactor as you learn

## Common Pitfalls

### 1. Trying to Learn Everything at Once
- **Solution:** Focus on one module at a time
- Master basics before moving to advanced topics

### 2. Skipping the Fundamentals
- **Solution:** Don't skip Module 2 if new to Rust
- Ownership is crucial - take time to understand it

### 3. Not Running Code
- **Solution:** Type every example, don't just read
- Experiment and break things

### 4. Ignoring Errors
- **Solution:** Read compiler messages carefully
- Rust's error messages are helpful - learn from them

### 5. Working in Isolation
- **Solution:** Ask questions early
- Use community resources

## Progress Tracking

Track your progress:

```markdown
# My Detection-as-Code Journey

## Modules Completed
- [x] Module 1: Foundations
- [ ] Module 2: Rust Basics
- [ ] Module 3: Detection Fundamentals
...

## Projects Built
- [ ] Project 1: Log Parser
- [ ] Project 2: Rule Engine
...

## Key Learnings
- 2024-01-15: Understood ownership and borrowing
- 2024-01-20: Built first detection rule
...
```

## Getting Help

**Stuck? Try this order:**

1. **Read the docs:** Check module README
2. **Search issues:** Someone may have asked before
3. **Compiler messages:** Read Rust error messages carefully
4. **Community:** Ask in Discussions
5. **Examples:** Look at example code
6. **Debug:** Add print statements and tests

## Setting Goals

### Week 1 Goal
Complete Module 1 and write your first Sigma rule.

### Month 1 Goal
Finish Modules 1-3 and Project 1.

### 3-Month Goal
Complete standard track, build Projects 1-3.

### 6-Month Goal
Finish all modules, deploy Project 5 to production.

## Next Steps

Ready to start?

1. Complete setup above
2. Choose your learning path
3. Set your goals
4. Start with [Module 1](curriculum/modules/module-01-foundations/README.md)

**Happy learning!**

---

**Need help?** Open a [Discussion](https://github.com/404foundme-lg/Detection-as-Code/discussions)
