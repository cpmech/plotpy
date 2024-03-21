use plotpy::{generate3d, Plot, StrError, Surface};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_surface() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface
        .set_row_stride(1)
        .set_col_stride(1)
        .set_with_wireframe(true)
        .set_colormap_name("Pastel1")
        .set_with_colorbar(true)
        .set_colorbar_label("temperature")
        .set_number_format_cb("%.1f")
        .set_wire_line_color("#1862ab")
        .set_wire_line_style(":")
        .set_wire_line_width(0.75);

    // draw surface
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
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
    let n_lines = lines_iter.count();
    assert!(n_lines > 1600 && n_lines < 1700);
    Ok(())
}

#[test]
fn test_surface_color() -> Result<(), StrError> {
    let mut surface = Surface::new();

    // draw surface
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface.set_surf_color("gold").draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_color.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n_lines = lines_iter.count();
    assert!(n_lines > 1100 && n_lines < 1200);
    Ok(())
}

#[test]
fn test_surface_wireframe() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_surface(false).set_with_wireframe(true);

    // draw wireframe
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface.draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_wireframe.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 890);
    Ok(())
}

#[test]
fn test_surface_points() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_surface(false).set_with_points(true);

    // draw points
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface.set_point_size(200.0).set_point_style("*").draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_points.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n_lines = lines_iter.count();
    assert!(n_lines > 800 && n_lines < 900);
    Ok(())
}

#[test]
fn test_surface_points_color() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_surface(false).set_with_points(true);

    // draw points
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface
        .set_point_color("blue")
        .set_point_line_color("red")
        .set_point_size(100.0)
        .set_point_style("s")
        .draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_points_color.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n_lines = lines_iter.count();
    assert!(n_lines > 800 && n_lines < 900);
    Ok(())
}

#[test]
fn test_surface_points_void() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_surface(false).set_with_points(true);

    // draw points
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface
        .set_point_void(true)
        .set_point_line_color("black")
        .set_point_line_width(1.5)
        .set_point_size(100.0)
        .set_point_style("o")
        .draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_points_void.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n_lines = lines_iter.count();
    assert!(n_lines > 800 && n_lines < 900);
    Ok(())
}
