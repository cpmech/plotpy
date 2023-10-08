# Rust plotting library using Python (Matplotlib)

[![codecov](https://codecov.io/gh/cpmech/plotpy/branch/main/graph/badge.svg?token=SUBRKUN63U)](https://codecov.io/gh/cpmech/plotpy)

## Contents

* [Introduction](#introduction)
* [Installation on Debian/Ubuntu/Linux](#installation)
* [Installation on macOS](#macos)
* [Setting Cargo.toml](#cargo)
* [Examples](#examples)

## <a name="introduction"></a> Introduction

This crate implements high-level functions to generate plots and drawings.  Although we use Python/Matplotlib, the goal is to provide a convenient Rust library that is **different** than Matplotlib. The difference happens because we want **convenience** for the Rust developer while getting the **fantastic quality of Matplotlib** 😀.

Internally, we use [Matplotlib](https://matplotlib.org/) via a Python 3 script.  First, we generate a python code in a directory of your choice (e.g., `/tmp/plotpy`), and then we call **python3** using Rust's `std::process::Command`.

For more information (and examples), check out the [plotpy documentation on docs.rs](https://docs.rs/plotpy).

See also the [examples directory](https://github.com/cpmech/plotpy/tree/main/examples) with the output of the [integration tests](https://github.com/cpmech/plotpy/tree/main/tests).

## <a name="installation"></a> Installation on Debian/Ubuntu/Linux

In addition to Matplotlib (version >= 3.3.0), this crate depends on `russell_lab`, which, in turn, depends on an efficient BLAS library such as [OpenBLAS](https://github.com/OpenMathLib/OpenBLAS) and [Intel MKL](https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2023-2/overview.html). Thus, we have two options:

1. Use the standard Debian packages based on OpenBLAS (default)
2. **(XOR)** Install Intel MKL, which includes LAPACK

Option 2 requires the following environment variable:

```bash
export RUSSELL_LAB_USE_INTEL_MKL=1
```

For convenience, you may use the scripts in the [zscripts](https://github.com/cpmech/plotpy/tree/main/zscripts) directory.

**1.** Install Matplotlib and use the standard Debian packages based on OpenBLAS:

```bash
bash zscripts/01-ubuntu-matplotlib-and-openblas.bash
```

**2.** Install Matplotlib and install Intel MKL:

```bash
bash zscripts/02-ubuntu-matplotlib-and-intel-mkl.bash
```

### <a name="macos"></a> Installation on macOS

In macOS, you may use [Homebrew](https://brew.sh/) (and pip) to install the dependencies:

```bash
brew install openblas lapack
pip3 install matplotlib
```

**Note** In macOS, we have to set the `LIBRARY_PATH` all the time when using `plotpy`:

```bash
export LIBRARY_PATH=$LIBRARY_PATH:$(brew --prefix)/opt/openblas/lib:$(brew --prefix)/opt/lapack/lib
```

## <a name="cargo"></a> Setting Cargo.toml

[![Crates.io](https://img.shields.io/crates/v/plotpy.svg)](https://crates.io/crates/plotpy)

👆 Check the crate version and update your Cargo.toml accordingly:

```toml
[dependencies]
plotpy = "*"
```

## <a name="examples"></a> Examples

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
