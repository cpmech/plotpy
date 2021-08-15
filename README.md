# Rust plotting library using Python (Matplotlib)

Work in progress...

This package implements a plotting library, with its own functions. However, internally, this package generates a Python script with Matplotlib commands. Then, this package runs the script using `process::Command`.

Documentation:

- [API reference (docs.rs)](https://docs.rs/plotpy)

## Installation

Add the following lines to Cargo.toml:

```toml
[dependencies]
plotpy = "*"
```
