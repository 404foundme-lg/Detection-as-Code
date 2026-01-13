# Detection-as-Code with Rust - Course Syllabus

## Course Information

**Course Title:** Detection-as-Code with Rust  
**Duration:** 12-20 weeks (varies by track)  
**Credits:** Self-paced, non-credit  
**Prerequisites:** Basic programming, security awareness  
**Level:** Intermediate to Advanced

## Course Description

This comprehensive curriculum teaches modern detection engineering using Rust. Students learn to build high-performance, production-ready detection systems by applying software engineering principles to security detections. The course combines theoretical foundations with extensive hands-on practice.

## Learning Outcomes

Upon successful completion, students will be able to:

1. **Design** detection rules using industry-standard formats (Sigma)
2. **Implement** high-performance log parsers and detection engines in Rust
3. **Deploy** production detection systems with CI/CD pipelines
4. **Optimize** detection performance for high-throughput environments
5. **Evaluate** detection effectiveness using MITRE ATT&CK framework
6. **Contribute** to open-source detection engineering projects

## Course Structure

### Module 1: Foundations of Detection-as-Code (1 week)
- Detection-as-Code philosophy and principles
- Version control for security detections
- Testing and validation strategies
- **Lab:** Git workflow, Sigma rule creation

### Module 2: Rust Fundamentals for Security (2 weeks)
- Ownership, borrowing, and lifetimes
- Error handling patterns
- Collections and iterators
- Concurrency basics
- **Lab:** Log event parser, concurrent processor

### Module 3: Detection Engineering Fundamentals (1 week)
- MITRE ATT&CK framework
- Log formats (JSON, CEF, Syslog, EVTX)
- Detection logic patterns
- False positive management
- **Lab:** Multi-format parser, threshold detector

### Module 4: Data Parsing and Processing (2 weeks)
- Serde for JSON/YAML parsing
- Custom deserializers
- Schema validation
- Performance optimization
- **Lab:** CloudTrail parser, streaming processor

### Module 5: Pattern Matching and Detection Logic (2 weeks)
- Regular expressions
- State machines
- Correlation engines
- Time-based detection
- **Lab:** Pattern library, correlation engine

### Module 6: Sigma Rule Integration (1 week)
- Sigma rule specification
- Rule parsing and validation
- Backend translation
- Field mapping
- **Lab:** Sigma parser, backend compiler

### Module 7: Performance and Scalability (2 weeks)
- Profiling and benchmarking
- Memory optimization
- Concurrent processing
- Streaming pipelines
- **Lab:** Performance optimization, parallel processing

### Module 8: Deployment and Operations (1 week)
- Containerization (Docker)
- CI/CD pipelines
- Monitoring and observability
- Configuration management
- **Lab:** Docker deployment, GitHub Actions

### Module 9: Advanced Topics (2 weeks)
- Machine learning integration
- Graph-based detection
- Threat intelligence feeds
- Custom DSLs
- **Lab:** ML detector, graph analyzer

## Assessment Methods

### Knowledge Checks (20%)
- End-of-module quizzes
- Conceptual understanding questions
- Best practices scenarios

### Lab Assignments (30%)
- Hands-on coding exercises
- Implementation of detection concepts
- Code quality and testing

### Projects (40%)
1. Simple Log Parser (5%)
2. Detection Rule Engine (10%)
3. Sigma Rule Compiler (10%)
4. High-Performance Pipeline (15%)

### Capstone Project (10%)
- Complete detection system
- Documentation and presentation
- Code review and deployment

## Grading Scale

- **A:** 90-100% - Exceptional understanding and implementation
- **B:** 80-89% - Strong grasp of concepts with good execution
- **C:** 70-79% - Adequate understanding with room for improvement
- **D:** 60-69% - Basic concepts understood, significant gaps
- **F:** Below 60% - Insufficient mastery

## Required Materials

### Software (Free)
- Rust toolchain (rustc, cargo)
- Git version control
- Code editor (VS Code recommended)
- Docker (for deployment modules)

### Readings
- The Rust Programming Language (free online)
- Sigma specification (free)
- MITRE ATT&CK framework (free)

### Optional
- Programming Rust (O'Reilly)
- Practical Threat Detection Engineering

## Time Commitment

### Fast Track (6-8 weeks)
- **Time:** 15-20 hours/week
- **For:** Experienced Rust developers
- **Focus:** Security concepts, detection patterns

### Standard Track (12-14 weeks)
- **Time:** 10-15 hours/week
- **For:** Most students
- **Focus:** Balanced learning, all modules

### Extended Track (16-20 weeks)
- **Time:** 6-10 hours/week
- **For:** New to Rust or security
- **Focus:** Fundamentals, extensive practice

## Weekly Schedule (Standard Track)

### Weeks 1-2: Foundations & Rust Basics
- Module 1: Detection-as-Code foundations
- Module 2: Start Rust fundamentals
- Lab work and exercises

### Weeks 3-4: Rust Deep Dive
- Module 2: Complete Rust fundamentals
- Project 1: Begin log parser
- Practice ownership and borrowing

### Weeks 5-6: Detection Engineering
- Module 3: Detection fundamentals
- Module 4: Data parsing
- Project 1: Complete log parser

### Weeks 7-8: Pattern Matching
- Module 5: Pattern matching
- Project 2: Detection rule engine
- Advanced Rust patterns

### Weeks 9-10: Sigma Integration
- Module 6: Sigma rules
- Project 3: Sigma compiler
- Rule translation

### Weeks 11-12: Performance & Deployment
- Module 7: Performance optimization
- Module 8: Deployment
- Project 4: High-performance pipeline

### Weeks 13-14: Advanced Topics & Capstone
- Module 9: Advanced topics
- Project 5: Capstone project
- Final presentation

## Office Hours & Support

### Community Support
- GitHub Discussions: Q&A and general help
- Discord: Real-time chat (Rust community)
- Weekly office hours: TBD

### Response Times
- Discussion posts: Within 48 hours
- Code reviews: Within 1 week
- Project feedback: Within 2 weeks

## Academic Integrity

### Allowed
- Collaborating on concepts
- Discussing approaches
- Using online resources
- Getting help from community

### Not Allowed
- Copying code from other students
- Using complete solutions from internet
- Submitting others' work as your own
- Sharing solutions before deadlines

### Citation Required
- External code snippets
- Adapted algorithms
- Third-party libraries

## Accessibility

We're committed to accessibility:
- Text-based content (screen reader friendly)
- Code examples with descriptions
- Alternative assessment formats available
- Accommodations upon request

## Course Policies

### Attendance
- Self-paced, no mandatory sessions
- Weekly office hours optional
- Community participation encouraged

### Late Work
- Projects accepted up to 1 week late (-10%)
- Extensions available upon request
- No penalty for life circumstances

### Revisions
- Resubmit projects for improved grade
- One revision per project
- Must be within 2 weeks of feedback

## Technical Requirements

### Minimum System Requirements
- **OS:** Linux, macOS, or Windows 10+
- **RAM:** 4GB minimum, 8GB recommended
- **Storage:** 10GB free space
- **Network:** Stable internet connection

### Software Versions
- Rust: 1.70 or later
- Docker: 20.10 or later
- Git: 2.30 or later

## Resources

### Primary
- Course modules and documentation
- Example code repository
- Lab exercises

### Supplementary
- Rust Book and documentation
- Sigma rule repository
- MITRE ATT&CK website
- Detection engineering blogs

### Community
- Rust Users Forum
- Detection Engineering Discord
- Security Twitter/X community

## Course Updates

This syllabus may be updated during the course. Students will be notified of significant changes via:
- GitHub notifications
- Discussion announcements
- Email (if applicable)

---

## Contact Information

- **Issues:** GitHub Issues for bugs/problems
- **Discussions:** GitHub Discussions for questions
- **Contributions:** See CONTRIBUTING.md

---

**Version:** 1.0  
**Last Updated:** January 2024  
**Next Review:** June 2024
