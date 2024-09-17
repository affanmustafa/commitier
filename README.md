# Commitier

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg">
    <img alt="The Rust Programming Language: A language empowering everyone to build reliable and efficient software"
         src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"
         width="50%">
  </picture>
</div>

Commitier is a command-line tool designed to help developers create consistent and meaningful commit messages. Built with Rust, it provides a structured way to format commit messages and can even create commits directly.

## Features

- Interactive commit message creation
- Customizable commit message prefixes
- Option to create Git commits directly
- View recent commit history

## Prerequisites

Before you can use Commitier, you need to have Rust installed on your system. If you haven't installed Rust yet, follow these steps:

1. Visit <https://www.rust-lang.org/tools/install>
2. Follow the instructions for your operating system
3. After installation, restart your terminal

You can verify your Rust installation by running:

```bash
rustc --version
```

## Installation

Currently, Commitier is not packaged for distribution. To use it, you'll need to clone the repository and run it locally:

1. Clone the repository:

   ```bash
   git clone https://github.com/affanmustafa/commitier.git
   cd commitier
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

### Initializing Commitier

Before first use, initialize Commitier in your project:

```bash
cargo run init
```

This will prompt you to set up your preferred commit message prefixes.

### Creating a Commit Message

To generate a commit message:

```bash
cargo run commit
```

This will guide you through selecting a prefix and entering a commit description.

### Creating a Git Commit

To generate a commit message and create a Git commit:

```bash
cargo run commit --create-commit
```

### Viewing Recent Commits

To view recent commits:

```bash
cargo run check-commits
```

By default, this shows the last 5 commits. You can specify a different number:

```bash
cargo run check-commits --count 10
```

## Configuration

Commitier uses a custom field in your project's `package.json` file to store commit message prefixes. After initialization, your `package.json` will include:

```json
{
  "commitier-prefixes": [
    "feat:",
    "fix:",
    "docs:",
    "style:",
    "refactor:",
    "test:",
    "chore:"
  ]
}
```

You can manually edit this list to customize your commit prefixes.

## Default Prefixes

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Formatting, missing semi colons, etc; no code change
- `refactor:` - Refactoring production code
- `test:` - Adding missing tests, refactoring tests; no production code change
- `chore:` - Updating grunt tasks etc; no production code change

## Future Plans

In the future, Commitier will be packaged as an npm module for easier installation and use.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Affan Mustafa

## Acknowledgments

- Inspired by [Commitizen](https://commitizen.github.io/cz-cli/), a tool for creating standardized commit messages
- Built with Rust
