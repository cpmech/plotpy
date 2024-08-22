# Rust plotting library using Python (Matplotlib) <!-- omit from toc --> 

[![documentation](https://img.shields.io/badge/plotpy-documentation-blue)](https://docs.rs/plotpy)
[![Track Awesome List](https://www.trackawesomelist.com/badge.svg)](https://www.trackawesomelist.com/rust-unofficial/awesome-rust/)

[![Test & Coverage](https://github.com/cpmech/plotpy/actions/workflows/test_and_coverage.yml/badge.svg)](https://github.com/cpmech/plotpy/actions/workflows/test_and_coverage.yml)
[![codecov](https://codecov.io/gh/cpmech/plotpy/branch/main/graph/badge.svg?token=SUBRKUN63U)](https://codecov.io/gh/cpmech/plotpy)


## Contents <!-- omit from toc --> 

- [Introduction](#introduction)
- [Installation](#installation)
- [Setting Cargo.toml](#setting-cargotoml)
- [Use of Jupyter via evcxr](#use-of-jupyter-via-evcxr)
- [Examples](#examples)
  - [Contour](#contour)
  - [Superquadric](#superquadric)


## Introduction

This crate implements high-level functions to generate plots and drawings. Although we use Python/Matplotlib, the goal is to provide a convenient Rust library that is **different** than Matplotlib. The difference happens because we want **convenience** for the Rust developer while getting the **fantastic quality of Matplotlib** 😀.

Plotpy is more verbose than Matplotlib because we aim to minimize the need to memorize the functionality by taking advantage of the intelligence of the IDE (e.g., VS Code) on auto-completing the code.

Internally, we use [Matplotlib](https://matplotlib.org/) via a Python 3 script. First, we generate a python code in a directory of your choice (e.g., `/tmp/plotpy`), and then we call **python3** using Rust's `std::process::Command`.

For more information (and examples), check out the [plotpy documentation on docs.rs](https://docs.rs/plotpy).

See also the [examples directory](https://github.com/cpmech/plotpy/tree/main/examples) with the output of the [integration tests](https://github.com/cpmech/plotpy/tree/main/tests).



## Installation

*This code is mainly tested on Debian/Ubuntu/Linux.*

This crate needs Python3 and Matplotlib, of course.

On Debian/Ubuntu/Linux, run:

```bash
sudo apt install python3-matplotlib
```

**Important:** The Rust code will call `python3` via `std::process::Command`. However, there is an option to call a different python executable; for instance (the code below is no tested):

```text
let mut plot = Plot::new();
plot.set_python_exe("C:\Windows11\WhereIs\python.exe")
    .add(...)
    .save(...)?;
```



## Setting Cargo.toml

[![Crates.io](https://img.shields.io/crates/v/plotpy.svg)](https://crates.io/crates/plotpy)

👆 Check the crate version and update your Cargo.toml accordingly:

```toml
[dependencies]
plotpy = "*"
```



## Use of Jupyter via evcxr

Plotpy can be used with Jupyter via [evcxr](https://github.com/evcxr/evcxr). Thus, it can interactively display the plots in a Jupyter Notebook. This feature requires the installation of `evcxr`. See the [Jupyter/evcxr article](https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/).

The following code shows a minimal example (not tested)

```text
// set the python path
let python = "where-is-my/python";

// set the figure path and name to be saved
let path = "my-figure.svg";

// plot and show in a Jupyter notebook
let mut plot = Plot::new();
plot.set_python_exe(python)
    .set_label_x("x")
    .set_label_y("y")
    .show_in_jupyter(path)?;
```



## Examples

### Contour

```rust
use plotpy::{generate3d, Contour, Plot, StrError};

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
    cube.set_surf_color("#ee29f2")
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
