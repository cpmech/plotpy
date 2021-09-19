use plotpy::*;
use russell_lab::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

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
fn test_contour() -> Result<(), &'static str> {
    // contour object and options
    let mut contour = Contour::new();
    contour
        .set_colors(&vec!["#fcaeae", "#da98d1", "#c45178", "#5594d2", "#e6af69", "#e6d969"])
        .set_levels(&vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        .set_colorbar_label("temperature")
        .set_line_color("yellow")
        .set_line_style(":")
        .set_line_width(2.5)
        .set_selected_line_color("#69e699")
        .set_selected_line_width(5.0)
        .set_selected_level(1.0, true);

    // draw contour
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("contour.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1500);
    Ok(())
}

#[test]
fn test_contour_colors() -> Result<(), &'static str> {
    // contour object and options
    let mut contour = Contour::new();
    contour
        .set_colors(&vec!["red", "green", "blue"])
        .set_levels(&vec![1.0, 3.0, 5.0, 7.0])
        .set_no_lines(true)
        .set_no_labels(true)
        .set_no_inline_labels(true)
        .set_no_colorbar(true);

    // draw contour
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("contour_colors.svg");
    match plot.save(&path) {
        Err(_) => plot.print_log_file(&path)?,
        Ok(_) => (),
    }

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 580);
    Ok(())
}

#[test]
fn test_contour_colormap_index() -> Result<(), &'static str> {
    for index in 0..10 {
        // contour object and options
        let mut contour = Contour::new();
        contour
            .set_colormap_index(index)
            .set_no_lines(true)
            .set_no_labels(true)
            .set_no_inline_labels(true)
            .set_no_colorbar(true);

        // draw contour
        let n = 9;
        let (x, y, z) = gen_xyz(n);
        contour.draw(&x, &y, &z);

        // add contour to plot
        let mut plot = Plot::new();
        plot.add(&contour);

        // save figure
        let filename = format!("contour_colormap_{}.svg", index);
        let path = Path::new(OUT_DIR).join(&filename);
        plot.save(&path)?;

        // check number of lines
        let file = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 770);
    }
    Ok(())
}

#[test]
fn test_contour_colormap_name() -> Result<(), &'static str> {
    for name in ["Pastel1", "tab20c", "gnuplot2"] {
        // contour object and options
        let mut contour = Contour::new();
        contour
            .set_colormap_name(name)
            .set_no_lines(true)
            .set_no_labels(true)
            .set_no_inline_labels(true)
            .set_no_colorbar(true);

        // draw contour
        let n = 9;
        let (x, y, z) = gen_xyz(n);
        contour.draw(&x, &y, &z);

        // add contour to plot
        let mut plot = Plot::new();
        plot.add(&contour);

        // save figure
        let filename = format!("contour_colormap_{}.svg", name);
        let path = Path::new(OUT_DIR).join(&filename);
        plot.save(&path)?;

        // check number of lines
        let file = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 770);
    }
    Ok(())
}
