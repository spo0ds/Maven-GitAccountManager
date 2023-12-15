# GitHub Account Manager

### Overview

The GitHub Account Manager is a command-line tool designed to simplify the management of GitHub accounts for Git configurations. This tool helps you switch between different GitHub accounts seamlessly, set and display Git configurations, and manage a list of GitHub accounts.

### Problem Statement

Working with multiple GitHub accounts on a single machine can be challenging due to potential conflicts in Git configurations. This tool addresses this issue by allowing users to switch between GitHub accounts easily, set configurations for specific accounts, and manage a list of accounts associated with their Git environment.

### Installation and Usage

Follow these steps to install and use the functionality of the GitHub Account Manager:

Prerequisites

- Rust Installed:
    Ensure that you have Rust installed on your machine. If not, you can [install Rust](https://www.rust-lang.org/learn/get-started).

- Git Installed:
    Make sure Git is installed on your system, as this tool relies on Git for configuration operations.

### Installation

- Clone the Repository:
    Clone the repository containing your Rust GitHub Account Manager.

```bash
git clone <repository_url>
cd <repository_directory>
```

- Build the Project:
    Build the Rust project using Cargo.

```bash
cargo build --release
```

- Set the manager path variable.

```bash
echo 'export PATH=$PATH:$(pwd)/target/release' >> ~/.bashrc
```

- Run the Manager:
Execute the compiled binary to run the GitHub Account Manager.

```bash
manager <subcommand> [options]
```

### Commands and Usage

- Add a new GitHub account:

```bash
manager set <username> <email>
```

- Display all accounts:

```bash
manager list
```

- Switch to a different GitHub account:

```bash
manager switch <username>
```

- Reset Git configuration for an account or all accounts:

```bash
manager reset [<username>]
```

- Set Git configuration for a specific account:

```bash
manager config --username <username>
```

- Show current Git configuration:

```bash
manager show
```

### License

This project is licensed under 
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

