use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
fn test_curve() -> Result<(), &'static str> {
    // curve object and options
    let mut curve1 = Curve::new();
    curve1.line_alpha = 0.7;
    curve1.line_color = "#cd0000".to_string();
    curve1.line_style = "--".to_string();
    curve1.line_width = 2.0;
    curve1.marker_color = "#1862ab".to_string();
    curve1.marker_every = 2;
    curve1.marker_void = false;
    curve1.marker_line_color = "#cda500".to_string();
    curve1.marker_line_width = 3.0;
    curve1.marker_size = 8.0;
    curve1.marker_style = "p".to_string();

    // another curve
    let mut curve2 = Curve::new();
    curve2.line_style = "None".to_string();
    curve2.marker_line_color = "#1862ab".to_string();
    curve2.marker_style = "s".to_string();
    curve2.marker_void = true;

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
    let path = Path::new(OUT_DIR).join("curve.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 511);
    Ok(())
}
