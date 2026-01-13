# Learning Roadmap

Visual guide to navigating the Detection-as-Code curriculum.

## 🗺️ Your Journey

```
START HERE
    ↓
┌─────────────────────────────────────┐
│  Week 1: Foundations                │
│  • What is Detection-as-Code?       │
│  • Git for security detections      │
│  • Write your first Sigma rule      │
└───────────────┬─────────────────────┘
                ↓
┌─────────────────────────────────────┐
│  Weeks 2-4: Rust Fundamentals       │
│  • Ownership & borrowing            │
│  • Error handling                   │
│  • Collections & iterators          │
│  • Concurrency                      │
└───────────────┬─────────────────────┘
                ↓
        ┌───────┴────────┐
        ↓                ↓
┌──────────────┐  ┌──────────────┐
│  Project 1   │  │ Checkpoint   │
│  Log Parser  │  │ Can you...   │
│              │  │ ✓ Parse JSON │
│ 1-2 weeks    │  │ ✓ Use Result │
└──────┬───────┘  └──────────────┘
       ↓
┌─────────────────────────────────────┐
│  Weeks 5-6: Detection Engineering   │
│  • MITRE ATT&CK                     │
│  • Log formats                      │
│  • Detection patterns               │
│  • False positive management        │
└───────────────┬─────────────────────┘
                ↓
┌─────────────────────────────────────┐
│  Weeks 7-8: Pattern Matching        │
│  • Regular expressions              │
│  • State machines                   │
│  • Correlation engines              │
│  • Time windows                     │
└───────────────┬─────────────────────┘
                ↓
        ┌───────┴────────┐
        ↓                ↓
┌──────────────┐  ┌──────────────┐
│  Project 2   │  │ Checkpoint   │
│  Rule Engine │  │ Can you...   │
│              │  │ ✓ Detect     │
│ 2 weeks      │  │ ✓ Correlate  │
└──────┬───────┘  └──────────────┘
       ↓
┌─────────────────────────────────────┐
│  Weeks 9-10: Sigma Integration      │
│  • Parse Sigma rules                │
│  • Translate to backends            │
│  • Field mappings                   │
└───────────────┬─────────────────────┘
                ↓
┌─────────────────────────────────────┐
│  Project 3: Sigma Compiler          │
│  • YAML parsing                     │
│  • Backend translation              │
│  • CLI tool                         │
└───────────────┬─────────────────────┘
                ↓
┌─────────────────────────────────────┐
│  Weeks 11-12: Performance           │
│  • Profiling                        │
│  • Optimization                     │
│  • Concurrent processing            │
│  • Deployment                       │
└───────────────┬─────────────────────┘
                ↓
┌─────────────────────────────────────┐
│  Weeks 13-14: Capstone              │
│  • Complete detection system        │
│  • Production deployment            │
│  • Documentation                    │
└───────────────┬─────────────────────┘
                ↓
        🎓 COMPLETE! 🎉
```

## 🎯 Skill Progression

### Level 1: Foundation (Weeks 1-4)
**Skills Acquired:**
- ✅ Understand Detection-as-Code concepts
- ✅ Write basic Rust programs
- ✅ Parse JSON logs
- ✅ Use git for version control

**Can Build:**
- Simple log parsers
- Basic detection rules

### Level 2: Intermediate (Weeks 5-8)
**Skills Acquired:**
- ✅ Map detections to MITRE ATT&CK
- ✅ Work with multiple log formats
- ✅ Implement pattern matching
- ✅ Build correlation logic

**Can Build:**
- Multi-format parsers
- Threshold detectors
- Simple correlation engines

### Level 3: Advanced (Weeks 9-12)
**Skills Acquired:**
- ✅ Parse and translate Sigma rules
- ✅ Optimize for performance
- ✅ Deploy to production
- ✅ Monitor and tune

**Can Build:**
- Sigma rule engines
- High-performance pipelines
- Production detection systems

### Level 4: Expert (Weeks 13-14+)
**Skills Acquired:**
- ✅ Integrate ML models
- ✅ Graph-based detection
- ✅ Custom DSLs
- ✅ Open source contributions

**Can Build:**
- Complete detection platforms
- Advanced correlation systems
- Custom detection frameworks

## 📊 Skill Tree

```
                    Expert
                      ↑
        ┌─────────────┼─────────────┐
        │             │             │
    ML/Graph    Custom DSLs    Open Source
        ↑             ↑             ↑
        └─────────────┴─────────────┘
                  Advanced
                      ↑
        ┌─────────────┼─────────────┐
        │             │             │
    Sigma       Performance    Deployment
        ↑             ↑             ↑
        └─────────────┴─────────────┘
                Intermediate
                      ↑
        ┌─────────────┼─────────────┐
        │             │             │
   Detection    Patterns      Correlation
        ↑             ↑             ↑
        └─────────────┴─────────────┘
                  Foundation
                      ↑
        ┌─────────────┼─────────────┐
        │             │             │
    Rust Basics   Git/Version   Parsing
```

## 🏁 Milestones

### Milestone 1: First Detection (Week 2)
- ✓ Created first Sigma rule
- ✓ Tested with sample data
- ✓ Version controlled in git

### Milestone 2: First Rust Program (Week 4)
- ✓ Built working log parser
- ✓ All tests passing
- ✓ Handles errors gracefully

### Milestone 3: Detection Engine (Week 8)
- ✓ Implements multiple detection patterns
- ✓ Processes real log data
- ✓ Generates actionable alerts

### Milestone 4: Production Ready (Week 12)
- ✓ Handles 100K+ events/sec
- ✓ Deployed in container
- ✓ Monitored and observable

### Milestone 5: Expert (Week 14+)
- ✓ Complete detection platform
- ✓ Contributing to open source
- ✓ Teaching others

## 🔄 Iteration Cycles

Each module follows this cycle:

```
Learn → Practice → Build → Test → Review → Improve
  ↑                                            ↓
  └────────────────────────────────────────────┘
```

1. **Learn:** Read module content
2. **Practice:** Complete exercises
3. **Build:** Work on projects
4. **Test:** Validate your code
5. **Review:** Get feedback
6. **Improve:** Refine and optimize

## 📈 Progress Tracking

Use this template to track progress:

```markdown
# My Progress

## Current Status
- Module: X
- Project: Y
- Confidence: 7/10

## Completed
- [x] Module 1
- [x] Module 2
- [ ] Module 3
...

## Next Steps
1. Complete Module 3 exercises
2. Start Project 2
3. Review Rust ownership

## Challenges
- Understanding lifetimes
- Performance optimization
- Need more practice with...

## Wins
- First detection deployed!
- 100% test coverage
- Contributed to Sigma repo
```

## 🎓 Graduation Requirements

To complete the curriculum:

- ✓ All 9 modules completed
- ✓ At least 3 of 5 projects finished
- ✓ Capstone project deployed
- ✓ Documentation written
- ✓ Presentation delivered (optional)

## 🚀 Beyond the Curriculum

After completion:

1. **Build Real Systems**
   - Apply to your organization
   - Contribute to detection platforms
   
2. **Share Knowledge**
   - Write blog posts
   - Give talks
   - Mentor others

3. **Specialize**
   - Cloud detection (AWS, Azure, GCP)
   - Endpoint detection (EDR)
   - Network detection (NDR)
   - Application security

4. **Stay Current**
   - Follow new Rust features
   - Track emerging threats
   - Update detection rules
   - Engage with community

---

**Start your journey:** [Begin Module 1](modules/module-01-foundations/README.md)
