use plotpy::*;
use russell_lab::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

fn gen_xyz(n: usize) -> (Matrix, Matrix, Matrix) {
    assert!(n > 1);
    let mut x = Matrix::new(n, n);
    let mut y = Matrix::new(n, n);
    let mut z = Matrix::new(n, n);
    let (min, max) = (-2.0, 2.0);
    let d = (max - min) / ((n - 1) as f64);
    for i in 0..n {
        let v = min + (i as f64) * d;
        for j in 0..n {
            let u = min + (j as f64) * d;
            x[i][j] = u;
            y[i][j] = v;
            z[i][j] = u * u + v * v;
        }
    }
    (x, y, z)
}

#[test]
fn test_surface() -> Result<(), &'static str> {
    let mut surface = Surface::new();
    surface
        .set_row_stride(1)
        .set_col_stride(1)
        .set_with_wireframe(true)
        .set_colormap_name("Pastel1")
        .set_with_colorbar(true)
        .set_colorbar_label("temperature")
        .set_number_format_cb("%.1f")
        .set_line_color("#1862ab")
        .set_line_style(":")
        .set_line_width(0.75);

    // draw surface
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    surface.draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1650);
    Ok(())
}

#[test]
fn test_wireframe() -> Result<(), &'static str> {
    let mut surface = Surface::new();
    surface.set_with_surface(false).set_with_wireframe(true);

    // draw wireframe
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    surface.draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_wireframe.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 890);
    Ok(())
}
