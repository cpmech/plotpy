# Rust plotting library using Python (Matplotlib)

[![codecov](https://codecov.io/gh/cpmech/plotpy/branch/main/graph/badge.svg?token=SUBRKUN63U)](https://codecov.io/gh/cpmech/plotpy)

This package implements a plotting library, with its own functions. However, internally, this package generates a Python script with Matplotlib commands. Then, this package runs the script using `process::Command`.

For convenience, we use `Vector` and `Matrix` from [Russell Lab](https://github.com/cpmech/russell).

Documentation:

- [API reference (docs.rs)](https://docs.rs/plotpy)

## Installation

Install some libraries:

```bash
sudo apt-get install \
    liblapacke-dev \
    libopenblas-dev \
    python3-matplotlib
```

Add this to your Cargo.toml (choose the right version):

```toml
[dependencies]
plotpy = "*"
```

## Examples

### Contour

```rust
use plotpy::{Contour, Plot, StrError};
use russell_lab::generate3d;
use std::path::Path;

fn main() -> Result<(), StrError> {
    // generate (x,y,z) matrices
    let n = 21;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x - y * y);

    // configure contour
    let mut contour = Contour::new();
    contour
        .set_colorbar_label("temperature")
        .set_colormap_name("terrain")
        .set_selected_level(0.0, true);

    // draw contour
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);
    plot.set_labels("x", "y");

    // save figure
    plot.save(Path::new("/tmp/plotpy/readme_contour.svg"))?;
    Ok(())
}
```

![readme_contour.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/readme_contour.svg)
