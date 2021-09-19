use plotpy::{Curve, Plot};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

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
    plot.set_subplot(2, 2, 1)
        .set_horizontal_gap(0.1)
        .set_vertical_gap(0.2)
        .set_gaps(0.3, 0.4)
        .set_equal_axes()
        .set_hide_axes(false)
        .set_range(-1.0, 1.0, -1.0, 1.0)
        .set_range_from_vec(&[0.0, 1.0, 0.0, 1.0])
        .set_xmin(0.0)
        .set_xmax(1.0)
        .set_ymin(0.0)
        .set_ymax(1.0)
        .set_xrange(0.0, 1.0)
        .set_yrange(0.0, 1.0)
        .set_num_ticks_x(0)
        .set_num_ticks_x(8)
        .set_num_ticks_y(0)
        .set_num_ticks_y(5)
        .set_label_x("x-label")
        .set_label_y("y-label")
        .set_labels("x", "y");
    plot.clear_current_figure();
    plot.set_title("my plot")
        .set_frame_borders(false)
        .set_frame_borders(true)
        .set_frame_borders(false);
    plot.grid_and_labels("x", "y");

    // add curve to plot
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot.svg");
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
    let path = Path::new(OUT_DIR).join("integ_plot_error.xyz");
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
    plot.set_super_title("all subplots");
    plot.set_horizontal_gap(0.5);
    plot.set_vertical_gap(0.5);
    plot.set_gaps(0.3, 0.2);

    // add curve to subplots
    plot.set_subplot(2, 2, 1);
    plot.add(&curve);
    plot.set_subplot(2, 2, 2);
    plot.add(&curve);
    plot.set_subplot(2, 2, 3);
    plot.add(&curve);
    plot.set_subplot(2, 2, 4);
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_subplots.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 980);
    Ok(())
}
