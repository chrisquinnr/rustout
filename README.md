# rustout
[<img alt="github" src="https://img.shields.io/badge/github-chrisquinnr/rustout-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/chrisquinnr/rustout)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rustout.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rustout)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rustout-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/syn)


`rustout` is a minimal logging crate for Rust that supports printing colors to the terminal output. It has few dependencies and provides an easy-to-use interface for logging messages at different levels.

## Usage

```
cargo add rustout
```

Or, add `rustout` as a dependency in your `Cargo.toml` file:
```
rustout = "0.4.1"
```

Initialise the logger:
```rust
use rustout::init_logger;

fn main() {
    init_logger()

    // Your code here
}
```
Then start logging messages in your code using the log macros like this:
```rust
use log::{error, warn, info, debug, trace};

error!("This is an error message");
warn!("This is a warning message");
info!("This is an info message");
debug!("This is a debug message");
trace!("This is a trace message");

```
Build and run your project. The log messages will be printed to the terminal with colorized output.

## Customization
You can customize the log level filter and colors used, just  modify the code in the main function where the logger is initialised.

## Dependencies

- `log`: A flexible logging framework for Rust.
- `ansi_term`: A crate for coloring and formatting text output in the terminal.