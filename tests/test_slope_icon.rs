use plotpy::{Curve, Plot, SlopeIcon};
use russell_lab::Vector;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_slope_icon_below() -> Result<(), &'static str> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let slope = 1.0 / 3.0;
    let x = [-10.0, 10.0];
    let dx = x[1] - x[0];
    let y1 = [5.0, 5.0 + slope * dx];
    let y2 = [5.0 + slope * dx, 5.0];
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);

    // icon object and options
    let mut icon1 = SlopeIcon::new();
    let mut icon2 = SlopeIcon::new();
    let mut icon3 = SlopeIcon::new();
    let mut icon4 = SlopeIcon::new();

    // configure icons
    icon1.set_precision(3).set_offset_v(1.0);
    icon2.set_precision(3).set_offset_v(1.0);
    icon3.set_precision(3).set_offset_v(1.0);
    icon4.set_precision(3).set_offset_v(1.0);

    // draw icon
    let xc = x[0] + dx / 4.0;
    let yc = y1[0] + slope * dx / 4.0;
    icon1.draw(slope, xc, yc);
    let xc = x[0] + 3.0 * dx / 4.0;
    let yc = y1[0] + slope * 3.0 * dx / 4.0;
    icon2.draw(slope, xc, yc);
    let xc = x[0] + dx / 4.0;
    let yc = y2[0] - slope * dx / 4.0;
    icon3.draw(-slope, xc, yc);
    let xc = x[0] + 3.0 * dx / 4.0;
    let yc = y2[0] - slope * 3.0 * dx / 4.0;
    icon4.draw(-slope, xc, yc);

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&curve1)
        .add(&curve2)
        .add(&icon1)
        .add(&icon2)
        .add(&icon3)
        .add(&icon4);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_below.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 680);
    Ok(())
}

#[test]
fn test_slope_icon_above() -> Result<(), &'static str> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let slope = 1.0 / 3.0;
    let x = [-10.0, 10.0];
    let dx = x[1] - x[0];
    let y1 = [5.0, 5.0 + slope * dx];
    let y2 = [5.0 + slope * dx, 5.0];
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);

    // icon object and options
    let mut icon1 = SlopeIcon::new();
    let mut icon2 = SlopeIcon::new();
    let mut icon3 = SlopeIcon::new();
    let mut icon4 = SlopeIcon::new();

    // configure icons
    icon1.set_precision(3).set_offset_v(1.0).set_flipped(true);
    icon2.set_precision(3).set_offset_v(1.0).set_flipped(true);
    icon3.set_precision(3).set_offset_v(1.0).set_flipped(true);
    icon4.set_precision(3).set_offset_v(1.0).set_flipped(true);

    // draw icon
    let xc = x[0] + dx / 4.0;
    let yc = y1[0] + slope * dx / 4.0;
    icon1.draw(slope, xc, yc);
    let xc = x[0] + 3.0 * dx / 4.0;
    let yc = y1[0] + slope * 3.0 * dx / 4.0;
    icon2.draw(slope, xc, yc);
    let xc = x[0] + dx / 4.0;
    let yc = y2[0] - slope * dx / 4.0;
    icon3.draw(-slope, xc, yc);
    let xc = x[0] + 3.0 * dx / 4.0;
    let yc = y2[0] - slope * 3.0 * dx / 4.0;
    icon4.draw(-slope, xc, yc);

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&curve1)
        .add(&curve2)
        .add(&icon1)
        .add(&icon2)
        .add(&icon3)
        .add(&icon4);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_above.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 680);
    Ok(())
}

#[test]
fn test_slope_icon_linx_liny() -> Result<(), &'static str> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let slope = 1.0 / 3.0;
    let x = [-10.0, 10.0];
    let dx = x[1] - x[0];
    let y1 = [5.0, 5.0 + slope * dx];
    let y2 = [5.0 + slope * dx, 5.0];
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);

    // icon object and options
    let mut icon1 = SlopeIcon::new();
    let mut icon2 = SlopeIcon::new();
    let mut icon3 = SlopeIcon::new();
    let mut icon4 = SlopeIcon::new();
    let mut icon5 = SlopeIcon::new();
    let mut icon6 = SlopeIcon::new();
    let mut icon7 = SlopeIcon::new();
    let mut icon8 = SlopeIcon::new();

    // configure icons
    icon1.set_precision(3).set_offset_v(1.0);
    icon2.set_precision(3).set_offset_v(1.0).set_flipped(true);
    icon3.set_precision(3).set_offset_v(1.0);
    icon4.set_precision(3).set_offset_v(1.0).set_flipped(true);
    icon5.set_precision(3).set_offset_v(1.0);
    icon6.set_precision(3).set_offset_v(1.0).set_flipped(true);
    icon7.set_precision(3).set_offset_v(1.0);
    icon8.set_precision(3).set_offset_v(1.0).set_flipped(true);

    // draw icon
    let xc = x[0] + dx / 4.0;
    let yc = y1[0] + slope * dx / 4.0;
    icon1.draw(slope, xc, yc);
    icon2.draw(slope, xc, yc);
    let xc = x[0] + 3.0 * dx / 4.0;
    let yc = y1[0] + slope * 3.0 * dx / 4.0;
    icon3.draw(slope, xc, yc);
    icon4.draw(slope, xc, yc);
    let xc = x[0] + dx / 4.0;
    let yc = y2[0] - slope * dx / 4.0;
    icon5.draw(-slope, xc, yc);
    icon6.draw(-slope, xc, yc);
    let xc = x[0] + 3.0 * dx / 4.0;
    let yc = y2[0] - slope * 3.0 * dx / 4.0;
    icon7.draw(-slope, xc, yc);
    icon8.draw(-slope, xc, yc);

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&curve1)
        .add(&curve2)
        .add(&icon1)
        .add(&icon2)
        .add(&icon3)
        .add(&icon4)
        .add(&icon5)
        .add(&icon6)
        .add(&icon7)
        .add(&icon8);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_linx_liny.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 770);
    Ok(())
}

#[test]
fn test_slope_icon_logx_liny() -> Result<(), &'static str> {
    // linear models on logx-y
    let (p, slope) = (5.0, 0.5);
    let (x0, y0) = (10.0, 0.0);
    let lx0 = f64::log10(x0);
    let f1 = |x: f64| y0 + slope * (f64::log10(x) - lx0);
    let xmax = x0 + f64::powf(10.0, p);
    let ymax = f1(xmax);
    let f2 = |x: f64| ymax - slope * (f64::log10(x) - lx0);

    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.set_marker_style("o");
    curve2.set_marker_style("*");
    let x = Vector::linspace(x0, xmax, 5);
    let y1 = x.get_mapped(f1);
    let y2 = x.get_mapped(f2);
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);

    // icon object and options
    let mut icon1 = SlopeIcon::new();
    let mut icon2 = SlopeIcon::new();
    let mut icon3 = SlopeIcon::new();
    let mut icon4 = SlopeIcon::new();
    let mut icon5 = SlopeIcon::new();
    let mut icon6 = SlopeIcon::new();
    let mut icon7 = SlopeIcon::new();
    let mut icon8 = SlopeIcon::new();

    // configure icons
    icon1.set_offset_v(1.0).set_log_x(true);
    icon2.set_offset_v(1.0).set_log_x(true).set_flipped(true);
    icon3.set_offset_v(1.0).set_log_x(true);
    icon4.set_offset_v(1.0).set_log_x(true).set_flipped(true);
    icon5.set_offset_v(1.0).set_log_x(true);
    icon6.set_offset_v(1.0).set_log_x(true).set_flipped(true);
    icon7.set_offset_v(1.0).set_log_x(true);
    icon8.set_offset_v(1.0).set_log_x(true).set_flipped(true);

    // draw icon
    icon1.draw(slope, 1e2, f1(1e2));
    icon2.draw(slope, 1e2, f1(1e2));
    icon3.draw(slope, 1e4, f1(1e4));
    icon4.draw(slope, 1e4, f1(1e4));
    icon5.draw(-slope, 1e2, f2(1e2));
    icon6.draw(-slope, 1e2, f2(1e2));
    icon7.draw(-slope, 1e4, f2(1e4));
    icon8.draw(-slope, 1e4, f2(1e4));

    // add features to plot
    let mut plot = Plot::new();
    plot.set_log_x(true)
        .add(&curve1)
        .add(&curve2)
        .add(&icon1)
        .add(&icon2)
        .add(&icon3)
        .add(&icon4)
        .add(&icon5)
        .add(&icon6)
        .add(&icon7)
        .add(&icon8);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_logx_liny.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 930);
    Ok(())
}

#[test]
fn test_slope_icon_linx_logy() -> Result<(), &'static str> {
    // linear models on x-logy
    let (p, slope) = (5.0, 1.5);
    let (x0, y0) = (0.0, 10.0);
    let f1 = |x: f64| y0 * f64::powf(10.0, slope * (x - x0));
    let g1 = |y: f64| x0 + (1.0 / slope) * f64::log10(y / y0);
    let ymax = y0 + f64::powf(10.0, p);
    let xmax = x0 + f64::log10(ymax / y0) / slope;
    let f2 = |x: f64| ymax * f64::powf(10.0, -slope * (x - x0));
    let g2 = |y: f64| x0 - (1.0 / slope) * f64::log10(y / ymax);

    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.set_marker_style("o");
    curve2.set_marker_style("*");
    let x = Vector::linspace(x0, xmax, 5);
    let y1 = x.get_mapped(f1);
    let y2 = x.get_mapped(f2);
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);

    // icon object and options
    let mut icon1 = SlopeIcon::new();
    let mut icon2 = SlopeIcon::new();
    let mut icon3 = SlopeIcon::new();
    let mut icon4 = SlopeIcon::new();
    let mut icon5 = SlopeIcon::new();
    let mut icon6 = SlopeIcon::new();
    let mut icon7 = SlopeIcon::new();
    let mut icon8 = SlopeIcon::new();

    // configure icons
    icon1.set_offset_v(1.0).set_log_y(true);
    icon2.set_offset_v(1.0).set_log_y(true).set_flipped(true);
    icon3.set_offset_v(1.0).set_log_y(true);
    icon4.set_offset_v(1.0).set_log_y(true).set_flipped(true);
    icon5.set_offset_v(1.0).set_log_y(true);
    icon6.set_offset_v(1.0).set_log_y(true).set_flipped(true);
    icon7.set_offset_v(1.0).set_log_y(true);
    icon8.set_offset_v(1.0).set_log_y(true).set_flipped(true);

    // draw icon
    icon1.draw(slope, g1(1e2), 1e2);
    icon2.draw(slope, g1(1e2), 1e2);
    icon3.draw(slope, g1(1e4), 1e4);
    icon4.draw(slope, g1(1e4), 1e4);
    icon5.draw(-slope, g2(1e2), 1e2);
    icon6.draw(-slope, g2(1e2), 1e2);
    icon7.draw(-slope, g2(1e4), 1e4);
    icon8.draw(-slope, g2(1e4), 1e4);

    // add features to plot
    let mut plot = Plot::new();
    plot.set_log_y(true)
        .add(&curve1)
        .add(&curve2)
        .add(&icon1)
        .add(&icon2)
        .add(&icon3)
        .add(&icon4)
        .add(&icon5)
        .add(&icon6)
        .add(&icon7)
        .add(&icon8);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_linx_logy.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 880);
    Ok(())
}

#[test]
fn test_slope_icon_logx_logy() -> Result<(), &'static str> {
    // linear models on log-log
    let (p, slope) = (5.0, 2.0);
    let (x0, y0) = (10.0, 100.0);
    let (lx0, ly0) = (f64::log10(x0), f64::log10(y0));
    let ly1 = |lx: f64| ly0 + slope * (lx - lx0);
    let f1 = |x: f64| f64::powf(10.0, ly1(f64::log10(x)));
    let xmax = x0 + f64::powf(10.0, p);
    let ymax = f1(xmax);
    let ly00 = f64::log10(ymax);
    let ly2 = |lx: f64| ly00 - slope * (lx - lx0);
    let f2 = |x: f64| f64::powf(10.0, ly2(f64::log10(x)));

    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.set_marker_style("o");
    curve2.set_marker_style("*");
    let x = Vector::linspace(x0, xmax, 5);
    let y1 = x.get_mapped(f1);
    let y2 = x.get_mapped(f2);
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);

    // icon object and options
    let mut icon1 = SlopeIcon::new();
    let mut icon2 = SlopeIcon::new();
    let mut icon3 = SlopeIcon::new();
    let mut icon4 = SlopeIcon::new();
    let mut icon5 = SlopeIcon::new();
    let mut icon6 = SlopeIcon::new();
    let mut icon7 = SlopeIcon::new();
    let mut icon8 = SlopeIcon::new();

    // configure icons
    icon1.set_offset_v(1.0).set_log_x(true).set_log_y(true);
    icon2
        .set_offset_v(1.0)
        .set_log_x(true)
        .set_log_y(true)
        .set_flipped(true);
    icon3.set_offset_v(1.0).set_log_x(true).set_log_y(true);
    icon4
        .set_offset_v(1.0)
        .set_log_x(true)
        .set_log_y(true)
        .set_flipped(true);
    icon5.set_offset_v(1.0).set_log_x(true).set_log_y(true);
    icon6
        .set_offset_v(1.0)
        .set_log_x(true)
        .set_log_y(true)
        .set_flipped(true);
    icon7.set_offset_v(1.0).set_log_x(true).set_log_y(true);
    icon8
        .set_offset_v(1.0)
        .set_log_x(true)
        .set_log_y(true)
        .set_flipped(true);

    // draw icon
    icon1.draw(slope, 1e2, f1(1e2));
    icon2.draw(slope, 1e2, f1(1e2));
    icon3.draw(slope, 1e4, f1(1e4));
    icon4.draw(slope, 1e4, f1(1e4));
    icon5.draw(-slope, 1e2, f2(1e2));
    icon6.draw(-slope, 1e2, f2(1e2));
    icon7.draw(-slope, 1e4, f2(1e4));
    icon8.draw(-slope, 1e4, f2(1e4));

    // add features to plot
    let mut plot = Plot::new();
    plot.set_log_x(true)
        .set_log_y(true)
        .add(&curve1)
        .add(&curve2)
        .add(&icon1)
        .add(&icon2)
        .add(&icon3)
        .add(&icon4)
        .add(&icon5)
        .add(&icon6)
        .add(&icon7)
        .add(&icon8);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_logx_logy.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 610);
    Ok(())
}
