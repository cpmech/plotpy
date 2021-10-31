use plotpy::{Curve, Plot};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_curve() -> Result<(), &'static str> {
    // curve object and options
    let mut curve1 = Curve::new();
    curve1
        .set_line_alpha(0.7)
        .set_line_color("#cd0000")
        .set_line_style("--")
        .set_line_width(2.0)
        .set_marker_color("#1862ab")
        .set_marker_every(2)
        .set_marker_void(false)
        .set_marker_line_color("#cda500")
        .set_marker_line_width(3.0)
        .set_marker_size(8.0)
        .set_marker_style("p");

    // another curve
    let mut curve2 = Curve::new();
    curve2
        .set_line_style("None")
        .set_marker_line_color("#1862ab")
        .set_marker_style("s")
        .set_marker_void(true);

    // draw curves
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y = &[
        1.0, 1.41421356, 1.73205081, 2.0, 2.23606798, 2.44948974, 2.64575131, 2.82842712, 3.0, 3.16227766,
    ];
    let y2 = &[1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 3.5, 3.5, 3.5, 3.5];
    curve1.draw(x, y);
    curve2.draw(x, y2);

    // add curves to plot
    let mut plot = Plot::new();
    plot.add(&curve1);
    plot.add(&curve2);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_curve.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 490);
    Ok(())
}

#[test]
fn test_curve_3d() -> Result<(), &'static str> {
    // curve object and options
    let mut curve = Curve::new();
    curve
        .set_line_alpha(0.7)
        .set_line_color("#cd0000")
        .set_line_style("--")
        .set_line_width(2.0)
        .set_marker_color("#1862ab")
        .set_marker_every(2)
        .set_marker_void(false)
        .set_marker_line_color("#cda500")
        .set_marker_line_width(3.0)
        .set_marker_size(8.0)
        .set_marker_style("p");

    // draw curves
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
    let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
    let z = &[0.0, 0.0, 0.0, 1.0, 1.0];
    curve.draw_3d(x, y, z);

    // add curves to plot
    let mut plot = Plot::new();
    plot.set_range_3d(-0.5, 6.0, -0.5, 30.0, -0.5, 1.5);
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_curve_3d.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 700);
    Ok(())
}
