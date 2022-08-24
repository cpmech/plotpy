# Rust plotting library using Python (Matplotlib)

[![codecov](https://codecov.io/gh/cpmech/plotpy/branch/main/graph/badge.svg?token=SUBRKUN63U)](https://codecov.io/gh/cpmech/plotpy)

This library implements high-level functions to generate plots and drawings.
Although we use Python/Matplotlib, the goal is to provide a convenient Rust library
that is **different** than Matplotlib. The difference happens because we want **convenience**
for the Rust developer while getting the **fantastic quality of Matplotlib** 😀.

Internally, we use [Matplotlib](https://matplotlib.org/) via a Python 3 script.
First, we generate a python code in a directory of your choice (e.g., `/tmp/plotpy`),
and then we call **python3** using Rust's `std::process::Command`.

For more information (and examples), check out the [plotpy documentation on docs.rs](https://docs.rs/plotpy)

See also the [examples directory](https://github.com/cpmech/plotpy/tree/main/examples) with the output
of the [integration tests](https://github.com/cpmech/plotpy/tree/main/tests).

## Documentation

[Plotpy documentation on docs.rs](https://docs.rs/plotpy)

## Installation

For convenience, we use [Russell Lab](https://github.com/cpmech/russell).

Thus, we need the following some libraries

### Latest Ubuntu dependencies

```bash
sudo apt-get install liblapacke-dev libopenblas-dev python3-pip
pip install matplotlib
```

### Earlier Ubuntu dependencies

```bash
sudo apt-get install liblapacke-dev libopenblas-dev python3-pip3
pip3 install matplotlib
```

**Note** We use pip3 because the version of Matplotlib needs to be at least 3.3.0 and the earlier Ubuntu comes with 3.1.2.

### MacOS

```bash
brew install openblas lapack
export LIBRARY_PATH=$LIBRARY_PATH:$(brew --prefix)/opt/openblas/lib:$(brew --prefix)/opt/lapack/lib
pip3 install matplotlib
```

**Note** MacOS do not include the lib installed by homebrew, so, everytime you using `plotpy`, you should set the `LIBRARY_PATH`, or you can set it in your shell configure file. Here is an example for `zsh` users:

```bash
echo 'export LIBRARY_PATH=$LIBRARY_PATH:$(brew --prefix)/opt/openblas/lib:$(brew --prefix)/opt/lapack/lib' >> ~/.zshrc
```

### Cargo.toml

[![Crates.io](https://img.shields.io/crates/v/plotpy.svg)](https://crates.io/crates/plotpy)

👆 Check the crate version and update your Cargo.toml accordingly:

```toml
[dependencies]
plotpy = "*"
```

## Examples

### Contour

```rust
use plotpy::{Contour, Plot, StrError};
use russell_lab::generate3d;

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
    plot.save("/tmp/plotpy/readme_contour.svg")?;
    Ok(())
}
```

![readme_contour.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/readme_contour.svg)

### Superquadric

```rust
use plotpy::{Plot, StrError, Surface};

fn main() -> Result<(), StrError> {
    // star
    let r = &[1.0, 1.0, 1.0];
    let c = &[-1.0, -1.0, -1.0];
    let k = &[0.5, 0.5, 0.5];
    let mut star = Surface::new();
    star.set_colormap_name("jet")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // pyramids
    let c = &[1.0, -1.0, -1.0];
    let k = &[1.0, 1.0, 1.0];
    let mut pyramids = Surface::new();
    pyramids
        .set_colormap_name("inferno")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // rounded cube
    let c = &[-1.0, 1.0, 1.0];
    let k = &[4.0, 4.0, 4.0];
    let mut cube = Surface::new();
    cube.set_solid_color("#ee29f2")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // sphere
    let c = &[0.0, 0.0, 0.0];
    let k = &[2.0, 2.0, 2.0];
    let mut sphere = Surface::new();
    sphere
        .set_colormap_name("rainbow")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // sphere (direct)
    let mut sphere_direct = Surface::new();
    sphere_direct.draw_sphere(&[1.0, 1.0, 1.0], 1.0, 40, 20)?;

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&star)
        .add(&pyramids)
        .add(&cube)
        .add(&sphere)
        .add(&sphere_direct);

    // save figure
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/plotpy/readme_superquadric.svg")?;
    Ok(())
}
```

![readme_superquadric.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/readme_superquadric.svg)
