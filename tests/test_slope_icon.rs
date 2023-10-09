use plotpy::{linspace, Curve, Plot, SlopeIcon, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_slope_icon_below() -> Result<(), StrError> {
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
    icon1
        .set_precision(3)
        .set_offset_v(1.0)
        .set_line_style("--")
        .set_face_color("gold")
        .set_length(0.25)
        .set_no_text(true);
    icon2
        .set_precision(3)
        .set_offset_v(1.0)
        .set_fontsize(14.0)
        .set_text_h("")
        .set_text_v("$\\mathrm{\\lambda}$")
        .set_text_color("blue");
    icon3
        .set_precision(3)
        .set_offset_v(1.0)
        .set_text_offset_h(1.0)
        .set_text_offset_v(1.0);
    icon4.set_precision(3).set_offset_v(1.0).set_line_width(2.0);

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
fn test_slope_icon_above() -> Result<(), StrError> {
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
    icon1.set_precision(3).set_offset_v(1.0).set_above(true);
    icon2.set_precision(3).set_offset_v(1.0).set_above(true);
    icon3.set_precision(3).set_offset_v(1.0).set_above(true);
    icon4.set_precision(3).set_offset_v(1.0).set_above(true);

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
fn test_slope_icon_linx_liny() -> Result<(), StrError> {
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
    icon2.set_precision(3).set_offset_v(1.0).set_above(true);
    icon3.set_precision(3).set_offset_v(1.0);
    icon4.set_precision(3).set_offset_v(1.0).set_above(true);
    icon5.set_precision(3).set_offset_v(1.0);
    icon6.set_precision(3).set_offset_v(1.0).set_above(true);
    icon7.set_precision(3).set_offset_v(1.0);
    icon8.set_precision(3).set_offset_v(1.0).set_above(true);

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
fn test_slope_icon_logx_liny() -> Result<(), StrError> {
    // linear models on logx-y
    let (p, slope) = (5.0, 0.5);
    let (x0, y0) = (10.0, 0.0);
    let f1 = |x: f64| y0 + slope * (f64::log10(x / x0));
    let xmax = x0 + f64::powf(10.0, p);
    let ymax = f1(xmax);
    let f2 = |x: f64| ymax - slope * (f64::log10(x / x0));

    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.set_marker_style("o");
    curve2.set_marker_style("*");
    let x = linspace(x0, xmax, 5);
    let y1: Vec<_> = x.iter().map(|x| f1(*x)).collect();
    let y2: Vec<_> = x.iter().map(|x| f2(*x)).collect();
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
    let offset = 1.5;
    icon1.set_offset_v(offset);
    icon2.set_offset_v(offset).set_above(true);
    icon3.set_offset_v(offset);
    icon4.set_offset_v(offset).set_above(true);
    icon5.set_offset_v(offset);
    icon6.set_offset_v(offset).set_above(true);
    icon7.set_offset_v(offset);
    icon8.set_offset_v(offset).set_above(true);

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

    // NOTE: cannot set equal_axes when using log-lin axes

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_logx_liny.svg");
    plot.grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1030);
    Ok(())
}

#[test]
fn test_slope_icon_linx_logy() -> Result<(), StrError> {
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
    let x = linspace(x0, xmax, 5);
    let y1: Vec<_> = x.iter().map(|x| f1(*x)).collect();
    let y2: Vec<_> = x.iter().map(|x| f2(*x)).collect();
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
    icon1.set_offset_v(2.0);
    icon2.set_offset_v(2.0).set_above(true);
    icon3.set_offset_v(2.0);
    icon4.set_offset_v(2.0).set_above(true);
    icon5.set_offset_v(2.0);
    icon6.set_offset_v(2.0).set_above(true);
    icon7.set_offset_v(2.0);
    icon8.set_offset_v(2.0).set_above(true);

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

    // NOTE: cannot set equal_axes when using log-lin axes

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_linx_logy.svg");
    plot.grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 950);
    Ok(())
}

#[test]
fn test_slope_icon_logx_logy() -> Result<(), StrError> {
    // linear models on log-log
    //        y/y0  = (x/x0)^m
    //  log10(y/y0) = m * log10(x/x0)
    //     log10(y) = log10(y0) + m * (log10(x) - log10(x0))
    let (p, slope) = (5.0, 2.0);
    let (x0, y0) = (10.0, 100.0);
    let f1 = |x: f64| y0 * f64::powf(x / x0, slope);
    let xmax = x0 + f64::powf(10.0, p);
    let ymax = f1(xmax);
    let f2 = |x: f64| ymax * f64::powf(x / x0, -slope);

    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.set_marker_style("o");
    curve2.set_marker_style("*");
    let x = linspace(x0, xmax, 5);
    let y1: Vec<_> = x.iter().map(|x| f1(*x)).collect();
    let y2: Vec<_> = x.iter().map(|x| f2(*x)).collect();
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
    let offset = 2.0;
    icon1.set_offset_v(offset);
    icon2.set_offset_v(offset).set_above(true);
    icon3.set_offset_v(offset);
    icon4.set_offset_v(offset).set_above(true);
    icon5.set_offset_v(offset);
    icon6.set_offset_v(offset).set_above(true);
    icon7.set_offset_v(offset);
    icon8.set_offset_v(offset).set_above(true);

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

#[test]
fn test_slope_icon_example() -> Result<(), StrError> {
    // linear y vs linear x //////////////////////////////////////////

    // models
    let slope1 = 0.5;
    let (x1i, x1f, y1i) = (2.0, 12.0, 3.0);
    let f1a = |x: f64| y1i + slope1 * (x - x1i);
    let f1b = |x: f64| f1a(x1f) - slope1 * (x - x1i);

    // curves
    let mut curve1a = Curve::new();
    let mut curve1b = Curve::new();
    let x1 = linspace(x1i, x1f, 3);
    let y1a: Vec<_> = x1.iter().map(|x| f1a(*x)).collect();
    let y1b: Vec<_> = x1.iter().map(|x| f1b(*x)).collect();
    curve1a.set_marker_style("o").draw(&x1, &y1a);
    curve1b.set_marker_style("*").draw(&x1, &y1b);

    // icons
    let mut icon1a = SlopeIcon::new();
    let mut icon1b = SlopeIcon::new();
    icon1a.set_offset_v(0.0);
    icon1b.set_offset_v(0.0);
    icon1a.draw(slope1, 5.0, f1a(5.0));
    icon1b.draw(-slope1, 5.0, f1b(5.0));

    // plot
    let mut plot = Plot::new();
    plot.set_horizontal_gap(0.2);
    plot.set_subplot(2, 2, 1)
        .add(&curve1a)
        .add(&curve1b)
        .set_equal_axes(true)
        .add(&icon1a)
        .add(&icon1b)
        .grid_and_labels("x", "y");

    // linear y vs log10 x //////////////////////////////////////////

    // models
    let slope2 = 0.75;
    let (x2i, x2f, y2i) = (1.0, 1e6, 0.0);
    let f2a = |x: f64| y2i + slope2 * f64::log10(x / x2i);
    let f2b = |x: f64| f2a(x2f) - slope2 * f64::log10(x / x2i);

    // curves
    let mut curve2a = Curve::new();
    let mut curve2b = Curve::new();
    let x2 = linspace(x2i, x2f, 3);
    let y2a: Vec<_> = x2.iter().map(|x| f2a(*x)).collect();
    let y2b: Vec<_> = x2.iter().map(|x| f2b(*x)).collect();
    curve2a.set_marker_style("o").draw(&x2, &y2a);
    curve2b.set_marker_style("*").draw(&x2, &y2b);

    // icons
    let mut icon2a = SlopeIcon::new();
    let mut icon2b = SlopeIcon::new();
    icon2a.set_offset_v(0.0);
    icon2b.set_offset_v(0.0);
    icon2a.draw(slope2, 2e1, f2a(2e1));
    icon2b.draw(-slope2, 2e1, f2b(2e1));

    // NOTE: cannot set equal_axes when using log-lin axes

    // plot
    plot.set_subplot(2, 2, 2)
        .add(&curve2a)
        .add(&curve2b)
        .set_log_x(true) // must be set before adding icons
        .add(&icon2a)
        .add(&icon2b)
        .grid_and_labels("x", "y");

    // log y vs linear x ////////////////////////////////////////////

    // models
    let slope3 = 1.25;
    let (x3i, x3f, y3i) = (2.0, 12.0, 1.0);
    let f3a = |x: f64| y3i * f64::powf(10.0, slope3 * (x - x3i));
    let f3b = |x: f64| f3a(x3f) * f64::powf(10.0, -slope3 * (x - x3i));

    // curves
    let mut curve3a = Curve::new();
    let mut curve3b = Curve::new();
    let x3 = linspace(x3i, x3f, 3);
    let y3a: Vec<_> = x3.iter().map(|x| f3a(*x)).collect();
    let y3b: Vec<_> = x3.iter().map(|x| f3b(*x)).collect();
    curve3a.set_marker_style("o").draw(&x3, &y3a);
    curve3b.set_marker_style("*").draw(&x3, &y3b);

    // icons
    let mut icon3a = SlopeIcon::new();
    let mut icon3b = SlopeIcon::new();
    icon3a.set_offset_v(0.0);
    icon3b.set_offset_v(0.0);
    icon3a.draw(slope3, 5.0, f3a(5.0));
    icon3b.draw(-slope3, 5.0, f3b(5.0));

    // NOTE: cannot set equal_axes when using log-lin axes

    // plot
    plot.set_subplot(2, 2, 3)
        .add(&curve3a)
        .add(&curve3b)
        .set_log_y(true) // must be set before adding icons
        .add(&icon3a)
        .add(&icon3b)
        .grid_and_labels("x", "y");

    // log y vs log x ///////////////////////////////////////////////

    // models
    let slope4 = 1.5;
    let (x4i, x4f, y4i) = (1.0, 1e6, 1.0);
    let f4a = |x: f64| y4i * f64::powf(x / x4i, slope4);
    let f4b = |x: f64| f4a(x4f) * f64::powf(x / x4i, -slope4);

    // curves
    let mut curve4a = Curve::new();
    let mut curve4b = Curve::new();
    let x4 = linspace(x4i, x4f, 4);
    let y4a: Vec<_> = x4.iter().map(|x| f4a(*x)).collect();
    let y4b: Vec<_> = x4.iter().map(|x| f4b(*x)).collect();
    curve4a.set_marker_style("o").draw(&x4, &y4a);
    curve4b.set_marker_style("*").draw(&x4, &y4b);

    // icons
    let mut icon4a = SlopeIcon::new();
    let mut icon4b = SlopeIcon::new();
    icon4a.set_offset_v(0.0);
    icon4b.set_offset_v(0.0).set_above(true);
    icon4a.draw(slope4, 2e1, f4a(2e1));
    icon4b.draw(-slope4, 2e1, f4b(2e1));

    // plot
    plot.set_subplot(2, 2, 4)
        .add(&curve4a)
        .add(&curve4b)
        .set_log_x(true) // must be set before adding icons
        .set_log_y(true) // must be set before adding icons
        .set_equal_axes(true)
        .add(&icon4a)
        .add(&icon4b)
        .grid_and_labels("x", "y");

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_icon_example.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1400);
    Ok(())
}
