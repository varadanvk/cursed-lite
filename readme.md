# cursed-lite

An early-version command-line tool to randomize and restore filenames in a directory. Part of the cursed family of randomizers.

## Description

`cursed-lite` is a Rust-based command-line tool designed to:

- **Randomize filenames:** Recursively rename all `.py` files within a specified directory to random, meaningless names. This is useful for obfuscating filenames, creating temporary anonymized copies of code, or for testing scenarios.
- **Restore filenames:** Rename the randomized files back to their original names using a saved mapping file. This allows for easy reversal of the randomization process.

This tool is an early, "lite" version and focuses on core filename randomization and restoration functionality. It is a precursor to the full `cursed` package, which will be a comprehensive Python-based randomizer with extended features. `cursed-lite` is specifically built for handling Python (`.py`) files but can be adapted for other file types. It maintains the original directory structure, only altering the filenames.

## Installation

To install `cursed-lite` globally, you need to have Rust and Cargo installed on your system. If you haven't already, get them from [https://rustup.rs/](https://rustup.rs/).

Once Rust and Cargo are set up, follow these steps:

1.  **Clone the repository** (if you haven't downloaded the project code yet).
2.  **Navigate to the project directory** in your terminal (where `Cargo.toml` is located).
3.  **Run the installation command:**

    ```bash
    cargo install --path .
    ```

    This builds the project and installs the `cursed-lite` binary to Cargo's bin directory (typically `~/.cargo/bin`). Ensure this directory is in your system's `PATH` environment variable to execute the tool from anywhere.

## Usage

To use `cursed-lite`, follow these steps:

1.  **Navigate to the directory containing the files you want to randomize:**

    ```bash
    cursed-lite randomize <path_to_directory>

    ```

2.  **Run the command to randomize the filenames:**

    ```bash
    cursed-lite randomize <path_to_directory>
    ```

Mappings are stored locally in a `mappings.json` file within the target directory
