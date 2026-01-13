# Contributing to Detection-as-Code Curriculum

Thank you for your interest in contributing to the Detection-as-Code curriculum! This document provides guidelines for contributing.

## Ways to Contribute

### 1. Content Contributions
- New modules or lessons
- Additional examples and exercises
- Improved explanations
- Real-world case studies
- Sample detection rules

### 2. Code Contributions
- Example implementations
- Bug fixes in code samples
- Performance improvements
- Additional test cases

### 3. Documentation
- Fixing typos and errors
- Improving clarity
- Adding diagrams and visualizations
- Translating content

### 4. Community
- Answering questions in Discussions
- Sharing your learning journey
- Providing feedback on modules

## Contribution Process

### For Content Changes

1. **Fork the repository**
   ```bash
   gh repo fork 404foundme-lg/Detection-as-Code
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b content/your-contribution-name
   ```

3. **Make your changes**
   - Follow the existing structure and style
   - Test all code examples
   - Check spelling and grammar

4. **Commit with clear messages**
   ```bash
   git commit -m "Add: Exercise for Module 3 on correlation"
   ```

5. **Push and create a Pull Request**
   ```bash
   git push origin content/your-contribution-name
   ```

### For Code Examples

1. **Ensure code compiles**
   ```bash
   cd examples/your-example
   cargo build
   cargo test
   cargo clippy
   ```

2. **Include tests**
   - Unit tests for functionality
   - Integration tests where appropriate
   - Example usage in documentation

3. **Format code**
   ```bash
   cargo fmt
   ```

4. **Add a README**
   - Purpose of the example
   - How to run it
   - Expected output
   - Learning objectives

## Style Guidelines

### Markdown
- Use ATX-style headers (`#` not underlines)
- Code blocks should specify language
- Keep line length reasonable (~80-100 chars)
- Use relative links for internal references

### Rust Code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Include comments for complex logic
- Use `cargo fmt` for formatting
- Run `cargo clippy` and address warnings

### Detection Rules
- Follow Sigma specification for Sigma rules
- Include MITRE ATT&CK mappings
- Document false positives
- Provide test cases

## Content Standards

### Module Content Should Include
- Clear learning objectives
- Structured progression
- Practical examples
- Hands-on exercises
- Knowledge checks
- Additional resources

### Code Examples Should
- Be complete and runnable
- Include error handling
- Have meaningful variable names
- Include comments where helpful
- Demonstrate best practices

### Exercises Should
- Have clear instructions
- Include deliverables checklist
- Provide validation steps
- Offer bonus challenges
- Match module difficulty

## Reporting Issues

When reporting issues:

1. **Search existing issues** first
2. **Use issue templates** when available
3. **Include context:**
   - Module/section affected
   - What you expected
   - What actually happened
   - Steps to reproduce

## Pull Request Checklist

Before submitting a PR:

- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy warnings addressed
- [ ] Documentation is updated
- [ ] Commit messages are clear
- [ ] PR description explains changes
- [ ] Related issue is referenced

## Module Template

When creating new modules, use this structure:

```markdown
# Module X: [Title]

## Learning Objectives
- Objective 1
- Objective 2

## Table of Contents
1. [Section 1](#section-1)
2. [Section 2](#section-2)

## Section 1
Content with code examples...

## Section 2
More content...

## Lab Exercises
Hands-on practice...

## Knowledge Check
Questions and challenges...

## Resources
- Links to relevant materials

## Next Steps
Link to next module...
```

## Questions?

- **General questions:** Use [GitHub Discussions](https://github.com/404foundme-lg/Detection-as-Code/discussions)
- **Bug reports:** Open an [Issue](https://github.com/404foundme-lg/Detection-as-Code/issues)
- **Private concerns:** Email maintainers (see profile)

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Provide constructive feedback
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment or discriminatory language
- Trolling or insulting comments
- Publishing private information
- Other unprofessional conduct

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Credited in relevant sections
- Thanked in release notes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Thank You!

Your contributions help make detection engineering more accessible to everyone. We appreciate your time and effort!

---

**Questions?** Open a [Discussion](https://github.com/404foundme-lg/Detection-as-Code/discussions)
