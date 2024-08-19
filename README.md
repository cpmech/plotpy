# Rust plotting library using Python (Matplotlib)

[![codecov](https://codecov.io/gh/cpmech/plotpy/branch/main/graph/badge.svg?token=SUBRKUN63U)](https://codecov.io/gh/cpmech/plotpy)

## Contents

* [Introduction](#introduction)
* [Installation](#installation)
* [Setting Cargo.toml](#cargo)
* [Trying with Jupyter evcxr Kernel Interactively](#interactive)
* [Examples](#examples)

## <a name="introduction"></a> Introduction

This crate implements high-level functions to generate plots and drawings.  Although we use Python/Matplotlib, the goal is to provide a convenient Rust library that is **different** than Matplotlib. The difference happens because we want **convenience** for the Rust developer while getting the **fantastic quality of Matplotlib** 😀.

Internally, we use [Matplotlib](https://matplotlib.org/) via a Python 3 script.  First, we generate a python code in a directory of your choice (e.g., `/tmp/plotpy`), and then we call **python3** using Rust's `std::process::Command`.

For more information (and examples), check out the [plotpy documentation on docs.rs](https://docs.rs/plotpy).

See also the [examples directory](https://github.com/cpmech/plotpy/tree/main/examples) with the output of the [integration tests](https://github.com/cpmech/plotpy/tree/main/tests).

## <a name="installation"></a> Installation on Debian/Ubuntu/Linux

This crate needs Python3 and Matplotlib, of course.

On Debian/Ubuntu/Linux, run:

```bash
sudo apt install python3-matplotlib
```

**Important:** The Rust code will call `python3` via `std::process::Command`. However, there is an option to call a different python executable; for instance:

```text
let mut plot = Plot::new();
plot.set_python_exe("python")
    .add(...)
    .save(...)?;
```

## <a name="cargo"></a> Setting Cargo.toml

[![Crates.io](https://img.shields.io/crates/v/plotpy.svg)](https://crates.io/crates/plotpy)

👆 Check the crate version and update your Cargo.toml accordingly:

```toml
[dependencies]
plotpy = "*"
```

## <a name="interactive"></a> Trying with Jupyter evcxr Kernel Interactively
Plotpy now supports integration with `evcxr` and is able to interactively display the saved plots in Jupyter Notebook. This feature needs [evcxr](https://github.com/evcxr/evcxr) kernel, please install it following the [tutorial](https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/).

The following code shows a minimal example of this.
```rust
// Set python path. There's not currently any integration with evcxr for python, so
// we must set python path in order to use the suitable python environment.
let python = "/home/test/miniconda3/envs/py311/bin/python";
// Set the figure path and name to be saved.
let path = "figure.svg";
// Plot and save and then show in jupyter notebook
let mut plot = Plot::new();
plot.set_python_exe(python);
plot.set_label_x("x");
plot.set_label_y("y");
plot.save(path)?;
plot.show_in_evcxr(path)?;
```
Run the above code in jupyter notebook and then an empty figure will be displayed.

## <a name="examples"></a> Examples

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
