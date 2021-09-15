use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
fn test_plot() -> Result<(), &'static str> {
    // curve object and options
    let mut curve = Curve::new();

    // draw curve
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y = &[1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 3.5, 3.5, 3.5, 3.5];
    curve.draw(x, y);

    // configure plot
    let mut plot = Plot::new();
    plot.subplot(2, 2, 1);
    plot.subplot_horizontal_gap(0.1);
    plot.subplot_vertical_gap(0.2);
    plot.subplot_gap(0.3, 0.4);
    plot.equal();
    plot.hide_axes();
    plot.range(-1.0, 1.0, -1.0, 1.0);
    plot.range_vec(&[0.0, 1.0, 0.0, 1.0]);
    plot.xmin(0.0);
    plot.xmax(1.0);
    plot.ymin(0.0);
    plot.ymax(1.0);
    plot.xrange(0.0, 1.0);
    plot.yrange(0.0, 1.0);
    plot.xnticks(0);
    plot.xnticks(8);
    plot.ynticks(0);
    plot.ynticks(5);
    plot.xlabel("x-label");
    plot.ylabel("y-label");
    plot.labels("x", "y");
    plot.clear_current_figure();
    plot.title("my plot");
    plot.grid_and_labels("x", "y");

    // add curve to plot
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("plot.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 670);
    Ok(())
}

#[test]
fn test_plot_error() {
    let plot = Plot::new();
    let path = Path::new(OUT_DIR).join("plot_error.xyz");
    assert_eq!(plot.save(&path).err(), Some("python3 failed; please see the log file"));
}

#[test]
fn test_plot_subplots() -> Result<(), &'static str> {
    // curve object and options
    let mut curve = Curve::new();

    // draw curve
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let y = &[1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
    curve.draw(x, y);

    // configure plot
    let mut plot = Plot::new();
    plot.title_all_subplots("all subplots");
    plot.subplot_horizontal_gap(0.5);
    plot.subplot_vertical_gap(0.5);
    plot.subplot_gap(0.3, 0.2);

    // add curve to subplots
    plot.subplot(2, 2, 1);
    plot.add(&curve);
    plot.subplot(2, 2, 2);
    plot.add(&curve);
    plot.subplot(2, 2, 3);
    plot.add(&curve);
    plot.subplot(2, 2, 4);
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("plot_subplots.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 980);
    Ok(())
}
