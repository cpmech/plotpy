use plotpy::{Canvas, Curve, Plot, PolyCode, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_canvas() -> Result<(), StrError> {
    // canvas object and common options
    let mut canvas = Canvas::new();
    canvas.set_edge_color("#cd0000").set_face_color("#1862ab");

    // draw arc
    canvas.set_line_style("--");
    canvas.draw_arc(0.5, 0.5, 0.4, 195.0, -15.0);

    // draw arrow
    canvas.set_line_style("-");
    canvas.set_arrow_scale(50.0).set_arrow_style("fancy");
    canvas.draw_arrow(0.0, 0.0, 1.0, 1.0);

    // draw circle
    canvas.set_face_color("None").set_edge_color("grey");
    canvas.draw_circle(0.5, 0.5, 0.5);

    // draw polyline
    canvas.set_edge_color("blue");
    let a = 0.2;
    let c = f64::sqrt(3.0) / 2.0;
    let p = &[[0.1, 0.5], [0.1 + a, 0.5], [0.1 + a / 2.0, 0.5 + a * c]];
    let q = &[[0.9, 0.5], [0.9 - a, 0.5], [0.9 - a / 2.0, 0.5 + a * c]];
    canvas.draw_polyline(p, true);
    canvas.draw_polyline(q, false);

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas.svg");
    plot.set_equal_axes(true);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 450);
    Ok(())
}

#[test]
fn test_canvas_grid_2d() -> Result<(), StrError> {
    // canvas object and common options
    let mut s2d = Canvas::new();
    s2d.draw_grid(&[-0.2, -0.2], &[0.8, 1.8], &[5, 5], true, true)?;

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&s2d);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_grid_2d.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 780);
    Ok(())
}

#[test]
fn test_canvas_grid_3d() -> Result<(), StrError> {
    // canvas object and common options
    let mut s3d = Canvas::new();
    s3d.draw_grid(&[-1.0, -1.0, -1.0], &[1.0, 1.0, 1.0], &[2, 2, 2], true, true)?;

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&s3d);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_grid_3d.svg");
    plot.set_equal_axes(true)
        .set_show_errors(true)
        .set_save_pad_inches(0.3)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1000);
    Ok(())
}

#[test]
fn test_canvas_polyline_3d_methods() -> Result<(), StrError> {
    //           .   .  .   . ,.2|
    //         ' .           ,,'||
    //       '   .         ,,'  ||
    //     '     .       .,'    ||  →
    //  .  .   . .   .  3'      ||  n
    //           z     ||   ==========)
    //  .        |     ||       ||
    //          ,*---y || .  . ,1
    //  .      x       ||    ,,'
    //      ,'       H ||  ,,' W
    //  . ,'           ||,,'
    //  . . .   .   .  |0'
    let mut y = 0.5;
    const W: f64 = 2.0;
    const H: f64 = 1.0;
    let mut canvas = Canvas::new();
    canvas.set_edge_color("orange").set_line_width(5.0);
    canvas
        .polyline_3d_begin()
        .polyline_3d_add(W, y, 0.0)
        .polyline_3d_add(0.0, y, 0.0)
        .polyline_3d_add(0.0, y, H)
        .polyline_3d_add(W, y, H)
        .polyline_3d_end();

    y = 1.5;
    canvas
        .set_line_style("--")
        .polyline_3d_begin()
        .polyline_3d_add(W, y, 0.0)
        .polyline_3d_add(0.0, y, 0.0)
        .polyline_3d_add(0.0, y, H)
        .polyline_3d_add(W, y, H)
        .polyline_3d_add(W, y, 0.0) // close
        .polyline_3d_end();

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_polyline_3d_methods.svg");
    plot.set_equal_axes(true).set_show_errors(true);
    plot.save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 560);
    Ok(())
}

#[test]
fn test_canvas_polyline_3d() -> Result<(), StrError> {
    //           .   .  .   . ,.2|
    //         ' .           ,,'||
    //       '   .         ,,'  ||
    //     '     .       .,'    ||  →
    //  .  .   . .   .  3'      ||  n
    //           z     ||   ==========)
    //  .        |     ||       ||
    //          ,*---y || .  . ,1
    //  .      x       ||    ,,'
    //      ,'       H ||  ,,' W
    //  . ,'           ||,,'
    //  . . .   .   .  |0'
    let mut y = 0.5;
    const W: f64 = 2.0;
    const H: f64 = 1.0;
    #[rustfmt::skip]
    let points = &[
        [  W, y, 0.0],
        [0.0, y, 0.0],
        [0.0, y,   H],
        [  W, y,   H],
    ];
    let mut canvas = Canvas::new();
    canvas.draw_polyline(points, false);

    y = 1.5;
    #[rustfmt::skip]
    let points = &[
        [  W, y, 0.0],
        [0.0, y, 0.0],
        [0.0, y,   H],
        [  W, y,   H],
    ];
    canvas.draw_polyline(points, true);

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_polyline_3d.svg");
    plot.set_equal_axes(true).set_show_errors(true);
    plot.set_save_tight(false).save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 560);
    Ok(())
}

#[test]
fn test_canvas_polycurve_quadratic() -> Result<(), StrError> {
    // coordinates of control points
    let points = &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

    // codes
    let codes = &[PolyCode::MoveTo, PolyCode::Curve3, PolyCode::Curve3];

    // polycurve
    let mut canvas = Canvas::new();
    canvas.set_face_color("none").draw_polycurve(points, codes, true)?;

    // point on curve
    let mut curve = Curve::new();
    curve
        .set_marker_color("red")
        .set_marker_style("o")
        .draw(&[0.75], &[0.25]);

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas);
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_polycurve_quadratic.svg");
    plot.set_equal_axes(true).set_show_errors(true);
    plot.save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 410);
    Ok(())
}

#[test]
fn test_canvas_polycurve_cubic() -> Result<(), StrError> {
    // coordinates of control points
    let x = &[1.58, 0.35, -1.75, 0.375, 0.85, 2.2, 3.0, 2.0];
    let y = &[-2.57, -1.1, 2.0, 2.0, 1.15, 3.2, 0.05, -0.5];
    let points: Vec<_> = x.iter().zip(y).map(|(a, b)| vec![*a, *b]).collect();

    // codes
    let codes = &[
        PolyCode::MoveTo,
        PolyCode::Curve4,
        PolyCode::Curve4,
        PolyCode::Curve4,
        PolyCode::LineTo,
        PolyCode::Curve4,
        PolyCode::Curve4,
        PolyCode::Curve4,
    ];

    // polycurve
    let mut canvas = Canvas::new();
    canvas.draw_polycurve(&points, codes, true)?;

    // control points
    let mut curve = Curve::new();
    curve
        .set_line_color("green")
        .set_marker_color("red")
        .set_marker_style("o")
        .draw(x, y);

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_polycurve_cubic.svg");
    plot.set_equal_axes(true).set_show_errors(true);
    plot.save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 355);
    Ok(())
}

#[test]
fn test_canvas_polycurve_methods() -> Result<(), StrError> {
    // coordinates of control points
    let x = &[1.58, 0.35, -1.75, 0.375, 0.85, 2.2, 3.0, 2.0];
    let y = &[-2.57, -1.1, 2.0, 2.0, 1.15, 3.2, 0.05, -0.5];

    // codes
    let codes = &[
        PolyCode::MoveTo,
        PolyCode::Curve4,
        PolyCode::Curve4,
        PolyCode::Curve4,
        PolyCode::LineTo,
        PolyCode::Curve4,
        PolyCode::Curve4,
        PolyCode::Curve4,
    ];

    // polycurve
    let mut canvas = Canvas::new();

    canvas.polycurve_begin();
    for i in 0..x.len() {
        canvas.polycurve_add(x[i], y[i], codes[i]);
    }
    canvas.polycurve_end(true);

    // control points
    let mut curve = Curve::new();
    curve
        .set_line_color("orange")
        .set_marker_color("red")
        .set_marker_style("o")
        .draw(x, y);

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_polycurve_methods.svg");
    plot.set_equal_axes(true).set_show_errors(true);
    plot.save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 355);
    Ok(())
}

#[test]
fn test_canvas_rectangle() -> Result<(), StrError> {
    // canvas
    let mut canvas = Canvas::new();

    // rectangles
    canvas
        .set_line_style("--")
        .set_edge_color("#1536b3")
        .set_face_color("#fcbbbe")
        .draw_rectangle(0.5, 0.5, 2.0, 1.0)
        .set_line_style(":")
        .draw_rectangle(0.5, 2.0, 2.0, 0.5);

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_rectangle.svg");
    plot.set_range(0.0, 3.0, 0.0, 3.0)
        .set_equal_axes(true)
        .set_show_errors(true);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 420 && n < 470);
    Ok(())
}

#[test]
fn test_canvas_rectangle_and_text() -> Result<(), StrError> {
    // canvas
    let mut canvas = Canvas::new();

    // configurations
    canvas
        .set_face_color("#de3163")
        .set_line_style("None")
        .set_text_color("white")
        .set_alt_text_rotation(0.0)
        .set_alt_text_color("white")
        .set_alt_text_fontsize(14.0)
        .set_text_fontsize(20.0);

    // draw rectangles and text
    canvas
        .draw_rectangle(0.5, 0.5, 2.0, 1.0)
        .draw_text(1.5, 1.0, "HELLO")
        .set_alt_text_align_vertical("top")
        .set_alt_text_align_horizontal("left")
        .draw_alt_text(0.5, 1.5, "123")
        .set_alt_text_align_vertical("bottom")
        .set_alt_text_align_horizontal("right")
        .draw_alt_text(2.5, 0.5, "456");

    // add canvas to plot
    let mut plot = Plot::new();
    plot.add(&canvas);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_canvas_rectangle_and_text.svg");
    plot.set_range(0.0, 3.0, 0.0, 3.0)
        .set_equal_axes(true)
        .set_show_errors(true);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 530 && n < 600);
    Ok(())
}
