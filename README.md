# Rust plotting library using Python (Matplotlib)

[![codecov](https://codecov.io/gh/cpmech/plotpy/branch/main/graph/badge.svg?token=SUBRKUN63U)](https://codecov.io/gh/cpmech/plotpy)

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

## Examples

### Contour

```rust
use plotpy::*;
use std::path::Path;

// directory to save figures
const OUT_DIR: &str = "/tmp/plotpy/doc_tests";

fn main() -> Result<(), &'static str> {
    // generate (x,y,z) matrices
    let n = 21;
    let mut x = vec![vec![0.0; n]; n];
    let mut y = vec![vec![0.0; n]; n];
    let mut z = vec![vec![0.0; n]; n];
    let (min, max) = (-2.0, 2.0);
    let d = (max - min) / ((n - 1) as f64);
    for i in 0..n {
        let v = min + (i as f64) * d;
        for j in 0..n {
            let u = min + (j as f64) * d;
            x[i][j] = u;
            y[i][j] = v;
            z[i][j] = u * u - v * v;
        }
    }

    // configure and draw contour
    let mut contour = Contour::new();
    contour.colorbar_label = "temperature".to_string();
    contour.colormap_name = "terrain".to_string();
    contour.with_selected = true;
    contour.selected_level = 0.0;
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);
    plot.labels("x", "y");

    // save figure
    let path = Path::new(OUT_DIR).join("doc_contour.svg");
    plot.save(&path)?;
    Ok(())
}
```

![doc_contour.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_contour.svg)
