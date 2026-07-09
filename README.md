# git-ignore

**The maintenance toolkit for `.gitignore` files.**

A lightweight CLI for generating, analyzing, linting, organizing, and maintaining `.gitignore` files. It combines template generation, repository analysis, duplicate detection, intelligent recommendations, and project health checks into a single tool, so your `.gitignore` can be as clean and maintainable as the rest of your codebase. I built this tool to help me manage huge git workspaces containing multiple stacks without having to think too much about sensitive files or gitignore rules.

![](https://github.com/irfanghat/git-ignore/blob/main/docs/git_ignore_screenshot_demo.png?raw=true)

[![License](https://img.shields.io/badge/license-Apache-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/irfanghat/git-ignore/build.yaml?branch=main)](https://github.com/irfanghat/git-ignore/actions)

---

## Why git-ignore?

Most `.gitignore` files are written once, copy-pasted from Stack Overflow, and never touched again, accumulating stale rules, duplicates, and gaps as a project evolves. `git-ignore` treats your `.gitignore` as a living file that deserves the same care as the rest of your repository:

- **Generate** from well-maintained language/framework templates
- **Audit** for anomalies, redundant rules, and missing common patterns
- **Deduplicate** and clean up entries automatically
- **Inspect** your ignore rules without leaving the terminal

## Installation

### Via Cargo

```bash
cargo install git-ignore
```

### From source

```bash
git clone https://github.com/irfanghat/git-ignore.git
cd git-ignore
cargo install --path .
```

### Prebuilt binaries

Prebuilt binaries for Linux, macOS, and Windows are available on the [Releases](https://github.com/irfanghat/git-ignore/releases) page.

## Quick Start

```bash
# -----------------------------------------------------------
# Initialize a new .gitignore in the current directory
# -----------------------------------------------------------
git-ignore init

# -----------------------------------------------------------
# Generate one from a template
# -----------------------------------------------------------
git-ignore template Rust

# -----------------------------------------------------------
# See all available templates
# -----------------------------------------------------------
git-ignore template --list

# -----------------------------------------------------------
# Add a pattern
# -----------------------------------------------------------
git-ignore add "*.log"

# -----------------------------------------------------------
# Check if a pattern is already covered
# -----------------------------------------------------------
git-ignore check "target/"

# -----------------------------------------------------------
# Find and remove duplicate rules
# -----------------------------------------------------------
git-ignore dedupe

# -----------------------------------------------------------
# List everything currently ignored
# -----------------------------------------------------------
git-ignore list

# -----------------------------------------------------------
# Audit your .gitignore for issues and recommendations
# -----------------------------------------------------------
git-ignore audit

# -----------------------------------------------------------
# Automatically apply suggested fixes
# -----------------------------------------------------------
git-ignore audit fix
```

## Usage

```
The maintenance toolkit for .gitignore files.

Usage: git-ignore <COMMAND>

Commands:
  init      Create or initialize a .gitignore
  add       Add a pattern
  template  Create from template e.g. Node, Rust, C++, Dotnet etc.
  remove    Remove a pattern
  list      List ignored patterns
  dedupe    Remove duplicate rules
  check     Check if a pattern exists
  audit     Check for any anomalies, get recommendations and or apply them
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Command Reference

| Command | Description |
|---|---|
| `init` | Create a new `.gitignore` in the current directory if one doesn't exist |
| `add <pattern>` | Append a pattern to `.gitignore` |
| `remove <pattern>` | Remove a pattern from `.gitignore` |
| `template [name]` | Populate `.gitignore` from a known template (e.g. `Rust`, `Node`, `C++`, `Dotnet`) |
| `template --list` | List all available templates |
| `list` | Print all patterns currently defined in `.gitignore` |
| `check <pattern>` | Check whether a given pattern is already covered |
| `dedupe` | Detect and remove duplicate or redundant rules |
| `audit` | Scan `.gitignore` for anomalies and suggest improvements |
| `audit fix` | Apply the recommended fixes automatically |

## Templates

`git-ignore` ships with a curated set of language and framework templates, so you can scaffold a correct, idiomatic `.gitignore` in one command:

```bash
git-ignore template --list
```

```bash
git-ignore template Node
git-ignore template Rust
git-ignore template C++
git-ignore template Dotnet
```

> Don't see a template you need? [Open an issue](https://github.com/irfanghat/git-ignore/issues) or contribute one, see [Contributing](#contributing).

## How Auditing Works

`git-ignore audit` inspects your `.gitignore` for common problems:

- Duplicate or overlapping patterns
- Overly broad or overly narrow rules
- Missing entries commonly expected for your detected project type
- Dead rules that no longer match anything in the repository

Run `git-ignore audit` to see a report, or `git-ignore audit fix` to apply the suggested changes directly.

## Contributing

Contributions are welcome! Whether it's a bug fix, a new template, or an improvement to the audit heuristics:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Open a pull request

Please make sure `cargo test` and `cargo clippy` pass before submitting.

## License

Licensed under the [Apache 2.0 License](LICENSE).

## Acknowledgments

Built with [`clap`](https://github.com/clap-rs/clap) for the CLI interface and [`anyhow`](https://github.com/dtolnay/anyhow) for ergonomic error handling.
