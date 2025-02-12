# Project Plan: Building `cursed`

## Learning Path

### Phase 1: cursed-lite (File Randomizer)

A simpler project to learn Rust basics by building a file name randomizer.

#### Core Functionality

- Take a directory path as input
- Read all files in directory
- Generate random strings for new names
- Rename files while maintaining a mapping
- Ability to reverse changes

#### Learning Goals

1. **Rust Basics**

   - Basic syntax and types
   - Ownership model
   - Error handling with Result
   - String manipulation
   - File I/O operations
   - CLI argument parsing
   - Project structure

2. **Implementation Steps**
   - Set up project structure with Cargo
   - Parse command line arguments
   - Implement file reading/writing
   - Create random name generator
   - Build mapping system
   - Add reverse functionality
   - Basic error handling

### Phase 2: cursed (Python Obfuscator)

The main chaos-inducing project that will build on Phase 1's learnings.

#### Core Functionality

- Parse Python files
- Extract class/function definitions
- Distribute code across random files
- Maintain working imports
- Generate meaningless but valid code
- Create reversible mappings

#### Learning Goals

1. **Advanced Rust**

   - Custom error types
   - Advanced file manipulation
   - Abstract Syntax Tree parsing
   - Module system
   - Testing
   - Documentation

2. **Python Integration**
   - Understanding Python's import system
   - AST manipulation
   - Code generation
   - Maintaining dependencies

## Project Timeline

### Week 1: cursed-lite

1. **Day 1-2**

   - Set up Rust environment
   - Learn basic syntax
   - Create project structure
   - Implement basic file reading

2. **Day 3-4**

   - Implement random name generation
   - Add file renaming logic
   - Create mapping system

3. **Day 5-7**
   - Add reverse functionality
   - Implement error handling
   - Add basic CLI interface
   - Testing and documentation

### Week 2-3: cursed (Basic Version)

1. **Week 2**

   - Learn Python AST basics
   - Implement basic file parsing
   - Create class/function extraction
   - Build basic redistribution logic

2. **Week 3**
   - Implement import handling
   - Add mapping system
   - Create basic CLI
   - Testing and documentation

### Week 4+: cursed (Advanced Features)

- Random import generation
- Fake inheritance chains
- Misleading docstrings
- Multiple redistribution strategies
- Configuration options

## Technical Requirements

### cursed-lite

```toml
[dependencies]
clap = "4.4"              # CLI argument parsing
rand = "0.8"              # Random string generation
serde_json = "1.0"        # Mapping file format
anyhow = "1.0"            # Error handling
```

### cursed

```toml
[dependencies]
# Basic utilities
clap = "4.4"              # CLI argument parsing
rand = "0.8"              # Random string generation
serde_json = "1.0"        # Mapping file format
anyhow = "1.0"            # Error handling

# Python parsing
rustpython-parser = "0.2" # Python parsing
syn = "2.0"              # Rust syntax parsing
quote = "1.0"            # Code generation
```

## Project Structure

### cursed-lite

```
cursed-lite/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs       # Command line interface
│   ├── randomizer.rs # File randomization logic
│   └── mapper.rs    # Mapping management
└── tests/
    └── integration_tests.rs
```

### cursed

```
cursed/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs           # Command line interface
│   ├── parser/          # Python code parsing
│   │   ├── mod.rs
│   │   ├── ast.rs
│   │   └── imports.rs
│   ├── obfuscator/      # Code obfuscation logic
│   │   ├── mod.rs
│   │   ├── namer.rs     # Name generation
│   │   ├── distributor.rs # Code distribution
│   │   └── imports.rs    # Import handling
│   └── mapper.rs        # Mapping management
└── tests/
    ├── parser_tests.rs
    ├── obfuscator_tests.rs
    └── integration_tests.rs
```

## CLI Interface Design

### cursed-lite

```bash
# Basic usage
cursed-lite randomize <directory>
cursed-lite restore <mapping-file>

# Options
--dry-run    # Show what would happen without making changes
--verbose    # Detailed output
--exclude    # Patterns to exclude
```

### cursed

```bash
# Basic usage
cursed obfuscate <python-file-or-directory>
cursed restore <mapping-file>

# Options
--chaos-level <1-5>    # How aggressively to obfuscate
--strategy <scatter|consolidate|random>
--keep-docs           # Preserve original docstrings
--add-fake-imports   # Add meaningless imports
--dry-run           # Show what would happen
--verbose          # Detailed output
```

## Getting Started Steps

1. **Environment Setup**

   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Create new project
   cargo new cursed-lite
   cd cursed-lite

   # Add dependencies to Cargo.toml
   # [dependencies] section from above
   ```

2. **Basic Implementation**

   ```rust
   // Initial main.rs structure
   use std::path::PathBuf;
   use clap::Parser;

   #[derive(Parser)]
   struct Cli {
       #[clap(subcommand)]
       command: Commands,
   }

   #[derive(Subcommand)]
   enum Commands {
       Randomize {
           #[clap(parse(from_os_str))]
           path: PathBuf,
       },
       Restore {
           #[clap(parse(from_os_str))]
           mapping_file: PathBuf,
       },
   }

   fn main() {
       let cli = Cli::parse();
       // Implementation steps will go here
   }
   ```

3. **Next Steps**
   - Implement file reading logic
   - Add random name generation
   - Create mapping system
   - Build restore functionality
   - Add error handling
   - Implement CLI options

Remember: Start small, test often, and gradually increase complexity. The goal is to learn Rust while building something fun and chaotic!
