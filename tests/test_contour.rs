use plotpy::{generate3d, Contour, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_contour() -> Result<(), StrError> {
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
        .set_selected_level(1.0, true)
        .set_extra_filled("alpha=0.8")
        .set_extra_line("alpha=0.3");

    // draw contour
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_contour.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 1400 && n < 1700);
    Ok(())
}

#[test]
fn test_contour_colors() -> Result<(), StrError> {
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
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_contour_colors.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 580);
    Ok(())
}

#[test]
fn test_contour_colormap_index() -> Result<(), StrError> {
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
        let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
        contour.draw(&x, &y, &z);

        // add contour to plot
        let mut plot = Plot::new();
        plot.add(&contour);

        // save figure
        let filename = format!("integ_contour_colormap_{}.svg", index);
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
fn test_contour_colormap_name() -> Result<(), StrError> {
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
        let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
        contour.draw(&x, &y, &z);

        // add contour to plot
        let mut plot = Plot::new();
        plot.add(&contour);

        // save figure
        let filename = format!("integ_contour_colormap_{}.svg", name);
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
fn test_contour_draw_tri() -> Result<(), StrError> {
    // point coordinates (two triangles in a square)

    let x = vec![0.0, 1.0, 2.0, 3.0, 0.5, 1.5, 2.5, 1.0, 2.0, 1.5];
    let y = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 3.0];
    let connectivity = vec![
        vec![0, 1, 4],
        vec![1, 5, 4],
        vec![2, 6, 5],
        vec![4, 5, 7],
        vec![5, 8, 7],
        vec![7, 8, 9],
        vec![1, 2, 5],
        vec![2, 3, 6],
    ];

    // elevations
    let mut z = vec![0.0; x.len()];
    for i in 0..x.len() {
        z[i] = x[i] + y[i];
    }

    // canvas
    let mut contour = Contour::new();
    contour
        .set_no_fill(false)
        .set_colors(&vec!["#fcaeae", "#da98d1", "#c45178", "#5594d2", "#e6af69", "#e6d969"])
        .set_colorbar_label("x + y")
        .set_line_color("black")
        .set_line_style("-")
        .set_line_width(1.5)
        .set_tri_show_edges(true)
        .set_tri_edges_color("green")
        .set_tri_edges_line_width(1.0)
        .set_tri_edges_line_style("--");

    // draw contour
    contour.draw_tri(&x, &y, &z, &connectivity);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_contour_draw_tri.svg");
    plot.set_equal_axes(true).set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 1030 && n < 1090);
    Ok(())
}
