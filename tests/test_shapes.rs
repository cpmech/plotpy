use plotpy::{Curve, Plot, PolyCode, Shapes, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_shapes() -> Result<(), StrError> {
    // shapes object and common options
    let mut shapes = Shapes::new();
    shapes.set_edge_color("#cd0000").set_face_color("#1862ab");

    // draw arc
    shapes.draw_arc(0.5, 0.5, 0.4, 195.0, -15.0);

    // draw arrow
    shapes.set_arrow_scale(50.0).set_arrow_style("fancy");
    shapes.draw_arrow(0.0, 0.0, 1.0, 1.0);

    // draw circle
    shapes.set_face_color("None").set_edge_color("grey");
    shapes.draw_circle(0.5, 0.5, 0.5);

    // draw polyline
    shapes.set_edge_color("blue");
    let a = 0.2;
    let c = f64::sqrt(3.0) / 2.0;
    let p = &[[0.1, 0.5], [0.1 + a, 0.5], [0.1 + a / 2.0, 0.5 + a * c]];
    let q = &[[0.9, 0.5], [0.9 - a, 0.5], [0.9 - a / 2.0, 0.5 + a * c]];
    shapes.draw_polyline(p, true);
    shapes.draw_polyline(q, false);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes.svg");
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
fn test_shapes_grid_2d() -> Result<(), StrError> {
    // shapes object and common options
    let mut s2d = Shapes::new();
    s2d.draw_grid(&[-0.2, -0.2], &[0.8, 1.8], &[5, 5], true, true)?;

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&s2d);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes_grid_2d.svg");
    plot.set_equal_axes(true).grid_and_labels("x", "y");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 780);
    Ok(())
}

#[test]
fn test_shapes_grid_3d() -> Result<(), StrError> {
    // shapes object and common options
    let mut s3d = Shapes::new();
    s3d.draw_grid(&[-1.0, -1.0, -1.0], &[1.0, 1.0, 1.0], &[2, 2, 2], true, true)?;

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&s3d);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes_grid_3d.svg");
    plot.set_equal_axes(true).set_show_errors(true);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1000);
    Ok(())
}

#[test]
fn test_shapes_polyline_3d() -> Result<(), StrError> {
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
    let mut shapes = Shapes::new();
    shapes.draw_polyline(points, false);

    y = 1.5;
    #[rustfmt::skip]
    let points = &[
        [  W, y, 0.0],
        [0.0, y, 0.0],
        [0.0, y,   H],
        [  W, y,   H],
    ];
    shapes.draw_polyline(points, true);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes_polyline_3d.svg");
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
fn test_shapes_polycurve_quadratic() -> Result<(), StrError> {
    // coordinates of control points
    let points = &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

    // codes
    let codes = &[PolyCode::MoveTo, PolyCode::Curve3, PolyCode::Curve3];

    // polycurve
    let mut shapes = Shapes::new();
    shapes.set_face_color("none").draw_polycurve(points, codes, true)?;

    // point on curve
    let mut curve = Curve::new();
    curve
        .set_marker_color("red")
        .set_marker_style("o")
        .draw(&[0.75], &[0.25]);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes);
    plot.add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes_polycurve_quadratic.svg");
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
fn test_shapes_polycurve_cubic() -> Result<(), StrError> {
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
    let mut shapes = Shapes::new();
    shapes.draw_polycurve(&points, codes, true)?;

    // control points
    let mut curve = Curve::new();
    curve
        .set_line_color("green")
        .set_marker_color("red")
        .set_marker_style("o")
        .draw(x, y);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes_polycurve_cubic.svg");
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
