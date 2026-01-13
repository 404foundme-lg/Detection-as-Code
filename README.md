# Detection-as-Code

A comprehensive, modular curriculum for learning Detection Engineering using Rust.

## 🎯 Overview

This curriculum provides a professional learning path for implementing Detection-as-Code using Rust. Designed for engineers with security or software development experience, it combines theoretical foundations with hands-on practical applications.

## 📚 Curriculum Structure

The curriculum consists of 9 progressive modules:

1. **[Foundations of Detection-as-Code](curriculum/modules/module-01-foundations/)** - Core concepts and philosophy
2. **[Rust Fundamentals for Security](curriculum/modules/module-02-rust-basics/)** - Essential Rust for detection engineering  
3. **[Detection Engineering Fundamentals](curriculum/modules/module-03-detection-fundamentals/)** - Security detection principles
4. **[Data Parsing and Processing](curriculum/modules/module-04-data-parsing/)** - Building robust log parsers
5. **[Pattern Matching and Detection Logic](curriculum/modules/module-05-pattern-matching/)** - Implementing detection algorithms
6. **[Sigma Rule Integration](curriculum/modules/module-06-sigma-integration/)** - Industry-standard detection formats
7. **[Performance and Scalability](curriculum/modules/module-07-performance/)** - Production-grade optimization
8. **[Deployment and Operations](curriculum/modules/module-08-deployment/)** - Production deployment
9. **[Advanced Topics](curriculum/modules/module-09-advanced-topics/)** - ML, graphs, and cutting-edge techniques

## 🚀 Getting Started

### Prerequisites

- Basic programming experience (any language)
- Understanding of security concepts (logs, alerts, threats)
- Familiarity with command-line tools
- Git version control basics

### Setup

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   rustup component add clippy rustfmt
   ```

2. **Clone this repository:**
   ```bash
   git clone https://github.com/404foundme-lg/Detection-as-Code
   cd Detection-as-Code
   ```

3. **Start learning:**
   ```bash
   cd curriculum
   cat README.md
   ```

## 📖 Learning Paths

### 🏃 Fast Track (6-8 weeks)
For experienced Rust developers:
- Modules: 1, 3, 4, 5, 6, 8
- Projects: 2, 3, 5

### 🚶 Standard Track (12-14 weeks)
Recommended for most students:
- All modules in sequence
- All projects
- All labs

### 🧗 Extended Track (16-20 weeks)
For those new to Rust or security:
- All modules with additional practice
- Supplementary exercises  
- Multiple iterations on projects

## 🛠️ What You'll Build

Throughout this curriculum, you'll build:

- **Project 1:** Multi-format log parser (JSON, CEF, Syslog)
- **Project 2:** Detection rule engine with AND/OR/NOT logic
- **Project 3:** Sigma-to-native rule compiler
- **Project 4:** High-performance detection pipeline (100K+ events/sec)
- **Project 5:** Complete production detection system

## 📂 Repository Structure

```
Detection-as-Code/
├── curriculum/
│   ├── modules/           # 9 learning modules
│   ├── examples/          # Code examples
│   ├── exercises/         # Practice exercises
│   ├── projects/          # Hands-on projects
│   └── resources/         # Additional materials
├── README.md
└── CONTRIBUTING.md
```

## 🎓 Learning Outcomes

After completing this curriculum, you will be able to:

- ✅ Design and implement detection rules as code
- ✅ Build high-performance log parsers and detection engines
- ✅ Work with industry-standard formats (Sigma, YARA-L)
- ✅ Deploy detection systems in production environments
- ✅ Optimize detection performance and reduce false positives
- ✅ Contribute to open-source detection engineering projects

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas we'd love help with:
- Additional modules and exercises
- Example implementations
- Bug fixes and improvements
- Documentation enhancements

## 📜 License

This curriculum is released under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

Built with inspiration from:
- [Sigma Rule Project](https://github.com/SigmaHQ/sigma)
- [Detection Engineering Community](https://detectionengineering.io/)
- [Rust Security Working Group](https://www.rust-lang.org/governance/wgs/wg-security)
- [MITRE ATT&CK Framework](https://attack.mitre.org/)

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/404foundme-lg/Detection-as-Code/issues)
- **Discussions:** [GitHub Discussions](https://github.com/404foundme-lg/Detection-as-Code/discussions)

## 🌟 Star History

If you find this curriculum useful, please consider giving it a star! ⭐

---

**Ready to start?** Begin with the [Curriculum Overview](curriculum/README.md)