# Rust plotting library using Python (Matplotlib)

This package implements a plotting library, with its own functions. However, internally, this package generates a Python script with Matplotlib commands. Then, this package runs the script using `process::Command`.

Documentation:

- [API reference (docs.rs)](https://docs.rs/plotpy)

## Installation

### 1 Install Matplotlib

```bash
sudo apt-get install python3-matplotlib
```

### 2 Configure Cargo.toml

Add the following lines to Cargo.toml:

```toml
[dependencies]
plotpy = "*"
```
