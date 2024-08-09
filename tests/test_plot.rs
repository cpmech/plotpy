use plotpy::{linspace, Curve, Image, Plot, StrError, SuperTitleParams, Text};
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_plot() -> Result<(), StrError> {
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
        .set_equal_axes(true)
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
        .set_save_transparent(true)
        .set_label_x("x-label")
        .set_label_y("y-label")
        .set_labels("x", "y")
        .clear_current_axes();
    plot.clear_current_figure();
    plot.set_title("my plot'") // the extra "'" should be sanitized
        .set_frame_borders(false)
        .set_frame_borders(true)
        .set_frame_borders(false)
        .set_ticks_x(1.5, 0.5, "%.2f")
        .set_ticks_y(0.5, 0.1, "%g");
    plot.grid_and_labels("x", "y");

    // add curve to plot
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot.svg");
    plot.set_figure_size_points(250.0, 250.0 * 0.75);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 900);
    Ok(())
}

#[test]
fn test_plot_3d() -> Result<(), StrError> {
    // curve
    let mut curve = Curve::new();
    // https://matplotlib.org/stable/gallery/mplot3d/lines3d.html
    // theta = np.linspace(-4 * np.pi, 4 * np.pi, 100)
    // z = np.linspace(-2, 2, 100)
    // r = z**2 + 1
    // x = r * np.sin(theta)
    // y = r * np.cos(theta)
    let np = 101;
    let t0 = -4.0 * PI;
    let t1 = 4.0 * PI;
    let dt = (t1 - t0) / ((np - 1) as f64);
    let z0 = -2.0;
    let z1 = 2.0;
    let dz = (z1 - z0) / ((np - 1) as f64);
    let mut xx = vec![0.0; np];
    let mut yy = vec![0.0; np];
    let mut zz = vec![0.0; np];
    for i in 0..np {
        let theta = t0 + (i as f64) * dt;
        let z = z0 + (i as f64) * dz;
        let r = z * z + 1.0;
        xx[i] = r * f64::sin(theta);
        yy[i] = r * f64::cos(theta);
        zz[i] = z;
    }
    curve.draw_3d(&xx, &yy, &zz);

    // plot
    let mut plot = Plot::new();
    plot.set_subplot_3d(2, 2, 1)
        .add(&curve)
        .set_labels_3d("X AXIS", "Y AXIS", "Z AXIS")
        .set_num_ticks_x(0)
        .set_num_ticks_y(0)
        .set_num_ticks_z(0)
        .set_subplot_3d(2, 2, 2)
        .add(&curve)
        .set_label_x("X AXIS IS BEAUTIFUL")
        .set_label_y("Y AXIS IS BEAUTIFUL")
        .set_label_z("Z AXIS IS BEAUTIFUL")
        .set_xrange(-3.0, 3.0)
        .set_yrange(-3.0, 3.0)
        .set_zrange(-1.5, 1.5)
        .set_num_ticks_x(3)
        .set_num_ticks_y(3)
        .set_num_ticks_z(3)
        .set_hide_xticks()
        .set_hide_yticks()
        .set_hide_zticks()
        .set_subplot_3d(2, 2, 3)
        .add(&curve)
        .set_labels_3d("X HERE", "Y HERE", "Z HERE")
        .set_xmin(-2.0)
        .set_xmax(2.0)
        .set_ymin(-2.0)
        .set_ymax(2.0)
        .set_zmin(-1.0)
        .set_zmax(1.0)
        .set_subplot_3d(2, 2, 4)
        .add(&curve)
        .set_hide_xticks()
        .set_hide_yticks()
        .set_hide_zticks()
        .set_label_x_and_pad("X IS CLOSER NOW", -15.0)
        .set_label_y_and_pad("Y IS CLOSER NOW", -15.0)
        .set_label_z_and_pad("Z IS CLOSER NOW", -15.0)
        .set_range_3d(-10.0, 10.0, -10.0, 10.0, -3.0, 3.0);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_3d.svg");
    plot.set_horizontal_gap(0.2)
        .set_save_pad_inches(0.4)
        .set_figure_size_points(600.0, 600.0)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n_lines = lines_iter.count();
    assert!(n_lines > 1800 && n_lines < 1900);
    Ok(())
}

#[test]
fn test_plot_error() {
    let plot = Plot::new();
    let path = Path::new(OUT_DIR).join("integ_plot_error.xyz");
    assert_eq!(plot.save(&path).err(), Some("python3 failed; please see the log file"));
}

#[test]
fn test_plot_handles_quotes() -> Result<(), StrError> {
    let mut plot = Plot::new();
    plot.set_title("\"$\\int$ \"The Plot of the Developer\" $versus$ \"Developer's Plot\" $\\mathrm{d}\\sigma$\"");

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_handles_quotes.svg");
    plot.set_figure_size_points(250.0, 250.0 * 0.75);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let count = lines_iter.count();
    assert!(count > 920 && count < 980);
    Ok(())
}

#[test]
fn test_plot_title_handles_tex() -> Result<(), StrError> {
    let mut plot = Plot::new();
    plot.set_title("Van der Pol ($\\varepsilon = 10^{-6}$) - Radau5 - Tol = 1e-4");

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_title_handles_tex.svg");
    plot.set_figure_size_points(250.0, 250.0 * 0.75);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let count = lines_iter.count();
    assert!(count > 800 && count < 850);
    Ok(())
}

#[test]
fn test_plot_subplots() -> Result<(), StrError> {
    // curve object and options
    let mut curve = Curve::new();

    // draw curve
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let y = &[1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
    curve.draw(x, y);

    // params for super title
    let mut params = SuperTitleParams::new();
    params.set_align_vertical("bottom");

    // configure plot
    let mut plot = Plot::new();
    plot.set_super_title("\"$\\int$ Plot's Owner Says $\\mathrm{d}\\sigma$\": This is the \"super title\", \\n followed by a very long text to see \\n if this whole thing will be wrapped or not \\n we hope that it gets wrapped and beautifully formatted.\"", Some(params))
        .set_horizontal_gap(0.5)
        .set_vertical_gap(0.5)
        .set_gaps(0.3, 0.3);

    // add curve to subplots
    plot.set_subplot(2, 2, 1).set_title("\"Owner's First\"").add(&curve);
    plot.set_subplot(2, 2, 2).set_title("\"Owner's Second\"").add(&curve);
    plot.set_subplot(2, 2, 3).set_title("\"Owner's Third\"").add(&curve);
    plot.set_subplot(2, 2, 4).set_title("\"Owner's Fourth\"").add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_subplots.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1450);
    Ok(())
}

#[test]
fn test_plot_log() -> Result<(), StrError> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let mut curve3 = Curve::new();
    let mut curve4 = Curve::new();

    // draw curve
    let x = linspace(1.0, 11.0, 11);
    let y: Vec<_> = x.iter().map(|v| f64::exp(*v)).collect();
    curve1.draw(&x, &x);
    curve2.draw(&x, &y);
    curve3.draw(&y, &x);
    curve4.draw(&y, &y);

    // configure plot
    let mut plot = Plot::new();

    // add curve to subplots
    plot.set_subplot(2, 2, 1);
    plot.set_log_x(false);
    plot.set_log_y(false);
    plot.add(&curve1);

    plot.set_subplot(2, 2, 2);
    plot.set_log_x(false);
    plot.set_log_y(true);
    plot.add(&curve2);

    plot.set_subplot(2, 2, 3);
    plot.set_log_x(true);
    plot.set_log_y(false);
    plot.add(&curve3);

    plot.set_subplot(2, 2, 4);
    plot.set_log_x(true);
    plot.set_log_y(true);
    plot.add(&curve4);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_log.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 980);
    Ok(())
}

#[test]
fn test_plot_multiple_of_pi() -> Result<(), StrError> {
    // configure curve
    let mut cos_curve = Curve::new();
    let mut sin_curve = Curve::new();
    cos_curve.set_line_width(2.0);
    sin_curve.set_line_width(2.0).set_line_color("#cd0000");

    // add points
    const N: usize = 30;
    cos_curve.points_begin();
    sin_curve.points_begin();
    for i in 0..N {
        let u = (i as f64) * 2.0 * PI / ((N - 1) as f64);
        cos_curve.points_add(u, f64::cos(u));
        sin_curve.points_add(f64::sin(u), u);
    }
    cos_curve.points_end();
    sin_curve.points_end();

    // configure plot
    let mut plot = Plot::new();
    plot.set_gaps(0.3, 0.0).set_figure_size_points(600.0, 250.0);

    // add cos curve to plot
    plot.set_subplot(1, 2, 1);
    plot.add(&cos_curve).grid_and_labels("x", "y=cos(x)");
    plot.set_ticks_x_multiple_of_pi(0.0);

    // add sin curve to plot
    plot.set_subplot(1, 2, 2);
    plot.add(&sin_curve).grid_and_labels("x=sin(y)", "y");
    plot.set_ticks_y_multiple_of_pi(0.0);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_multiple_of_pi.svg");
    plot.set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1060);
    Ok(())
}

#[test]
fn test_plot_extra_functionality() -> Result<(), StrError> {
    // plot
    let mut plot = Plot::new();
    plot.set_horiz_line(-0.5, "green", "-", 1.0)
        .set_vert_line(-0.75, "gold", ":", 10.0)
        .set_cross(0.25, 0.75, "red", "--", 3.0)
        .set_range(-1.0, 1.0, -1.0, 1.0);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_extra_functionality.svg");
    plot.set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 490 && n < 530);
    Ok(())
}

#[test]
fn test_plot_tick_labels() -> Result<(), StrError> {
    // data
    let vegetables = [
        "cucumber",
        "tomato",
        "lettuce",
        "asparagus",
        "potato",
        "wheat",
        "barley",
    ];
    let farmers = [
        "Farmer Joe",
        "Upland Bros.",
        "Smith Gardening",
        "Agrifun",
        "Organiculture",
        "BioGoods Ltd.",
        "Cornylee Corp.",
    ];
    let harvest = [
        [0.8, 2.4, 2.5, 3.9, 0.0, 4.0, 0.0],
        [2.4, 0.0, 4.0, 1.0, 2.7, 0.0, 0.0],
        [1.1, 2.4, 0.8, 4.3, 1.9, 4.4, 0.0],
        [0.6, 0.0, 0.3, 0.0, 3.1, 0.0, 0.0],
        [0.7, 1.7, 0.6, 2.6, 2.2, 6.2, 0.0],
        [1.3, 1.2, 0.0, 0.0, 0.0, 3.2, 5.1],
        [0.1, 2.0, 0.0, 1.4, 0.0, 1.9, 6.3],
    ];

    // draw image
    let mut img = Image::new();
    img.draw(&harvest);

    // set tick labels
    let mut plot = Plot::new();
    let ticks: Vec<_> = (0..vegetables.len()).into_iter().collect();
    plot.add(&img)
        .set_rotation_ticks_x(45.0)
        .set_ticks_x_labels(&ticks, &farmers)
        .set_ticks_y_labels(&ticks, &vegetables);

    // add text
    let mut text = Text::new();
    text.set_color("white").set_align_horizontal("center");
    for i in 0..vegetables.len() {
        for j in 0..farmers.len() {
            text.draw(j as f64, i as f64, harvest[i][j].to_string().as_str());
        }
    }
    plot.add(&text);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_plot_tick_labels.svg");
    plot.set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 1620 && n < 1700);
    Ok(())
}
