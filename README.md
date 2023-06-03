# rustout

`rustout` is a minimal logging crate for Rust that supports printing colors to the terminal output. It has few dependencies and provides an easy-to-use interface for logging messages at different levels.

## Usage

```
cargo add rustout
```

Or, add `rustout` as a dependency in your `Cargo.toml` file:
```
rustout = "0.2.0"
```

Initialise the logger:
```rust
use rustout::init_logger;
use log::{error, warn, info, debug, trace};

fn main() {
    log::set_logger(&rustout).unwrap();
    log::set_max_level(LevelFilter::Trace);  // Adjust log level filter 

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