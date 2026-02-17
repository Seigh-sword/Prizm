# Contributing to Prizm

Created by: **Seigh-sword**  
Repository: https://github.com/Seigh-sword/Prizm

Thank you for your interest in contributing to Prizm! We welcome contributions from everyone, whether it's code, documentation, bug reports, or feature suggestions.

## How to Contribute

### Reporting Bugs
1. Check the [Issues](https://github.com/Seigh-sword/Prizm/issues) page to ensure it hasn't been reported
2. Create a new issue with:
   - Clear description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - Your platform (Linux/macOS/Windows) and Prizm version
   - Minimal code example if applicable

### Suggesting Features
1. Open an issue with the label "enhancement"
2. Describe the feature and why it would be useful
3. Provide examples of how the feature would be used

### Submitting Code

#### Setup Development Environment
```bash
# Clone the repository
git clone https://github.com/Seigh-sword/Prizm
cd needs-a-name

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cd compiler
cargo build
```

#### Making Changes
1. Create a new branch for your feature: `git checkout -b feature/my-feature`
2. Make your changes and test thoroughly
3. Follow the code style in existing files
4. Add comments for complex logic
5. Update documentation if needed

#### Testing
```bash
# Run tests
cargo test

# Build release
cargo build --release

# Test with example file
./target/release/prizm_compiler run ../source/example.pzm
```

#### Submitting a Pull Request
1. Push your branch: `git push origin feature/my-feature`
2. Create a Pull Request on GitHub
3. Provide:
   - Description of changes
   - Reference any related issues
   - Explanation of why the change is needed

## Code Guidelines

### Rust Code Style
- Use `cargo fmt` to format code
- Use `cargo clippy` for linting: `cargo clippy -- -D warnings`
- Follow standard Rust naming conventions (snake_case for variables/functions)

### Prizm Language Syntax
When adding new features to Prizm:
- Use comma (`,`) for statement terminators
- Follow the header/attribute pattern
- Document new attributes with ID numbers
- Add examples to the README

### Documentation
- Update README.md when adding features
- Add comments to complex code sections
- Document new attributes with clear descriptions
- Include usage examples

## Header System: Adding New Functionality

When adding a new header to Prizm:

1. **Define ID Range**: Assign unique IDs (e.g., header 1201-1206)
2. **Update attributes.rs**: Add module and attribute definitions
3. **Update lexer.rs**: Add keywords if needed
4. **Update stdlib.rs**: Implement the functionality
5. **Update README.md**: Document the new header with examples
6. **Update version.json**: Increment version number

### Example: Adding a "network" Header
```rust
// In attributes.rs
pub mod network {
    pub const HEADER_ID: u16 = 1201;
    pub const CONNECT: u16 = 1201;
    pub const SEND: u16 = 1202;
    pub const RECEIVE: u16 = 1203;
}

// In lexer.rs - add to keywords
"network" => TokenType::Header(Header::Network),

// In README.md - document usage
#### Network Operations Header (`network`)
- `network.connect(host, port)` - Connect to server
- `network.send(data)` - Send data
- `network.receive()` - Receive data
```

## Development Standards

### Commit Messages
- Be clear and descriptive
- Start with a verb (Add, Fix, Update, etc.)
- Example: "Add network header with socket operations"

### Pull Request Titles
- Follow pattern: `[Feature/Fix/Docs] Brief description`
- Example: `[Feature] Add network socket operations`

### Versions
- Version format: MAJOR.MINOR.PATCH (e.g., 0.1.0)
- Update version.json and Cargo.toml when releasing
- Document changes in CHANGELOG

## Areas for Contribution

### High Priority
- [ ] Implement actual binary compilation for all platforms
- [ ] Create auto-update mechanism (version checking and downloader)
- [ ] Build GUI setup window using ui header
- [ ] Add more standard library functions
- [ ] Complete parser implementation

### Medium Priority
- [ ] Package managers/module system
- [ ] Debugging support
- [ ] Performance optimizations
- [ ] Language Server Protocol (LSP) support
- [ ] VS Code extension

### Low Priority
- [ ] Additional UI themes
- [ ] Additional headers
- [ ] IDE plugins for other editors
- [ ] Community examples and tutorials

## Questions?

- Ask in Issues with label "question"
- Check documentation in README.md
- Look at examples in source/example.pzm

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please read and follow our Code of Conduct:

- Be respectful and inclusive
- Welcome feedback and criticism
- Focus on the work, not the person
- Respect confidentiality

Thank you for contributing to Prizm!
